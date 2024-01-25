use super::utils::win_id;
use crate::winit::{keyboard, mouse};
use crate::App;
use hashbrown::HashMap;
use notan_core::events::{DrawEvent, UpdateEvent};
use notan_core::window::{NotanWindow, WindowAction, WindowEvent, WindowId};
use notan_core::{AppState, System};
use winit::event::{Event, StartCause, WindowEvent as WWindowEvent};
use winit::event_loop::ControlFlow;

#[derive(Default)]
struct InnerWindowList(HashMap<WindowId, InnerWindowData>);

impl InnerWindowList {
    fn init_window<S: AppState + 'static>(&mut self, id: WindowId, sys: &mut System<S>) {
        if !self.0.contains_key(&id) {
            let (size, scale_factor) = sys
                .get_mut_plugin::<App>()
                .map(|app| {
                    app.window_by_id(id)
                        .map_or(((0, 0), 1.0), |win| (win.size(), win.scale()))
                })
                .unwrap_or(((0, 0), 1.0));

            self.0.insert(
                id,
                InnerWindowData {
                    id,
                    mouse_pos: None,
                    size,
                    scale_factor,
                },
            );
            sys.event(WindowEvent {
                id,
                action: WindowAction::Init,
            });
        }
    }

    fn remove(&mut self, id: &WindowId) {
        self.0.remove(id);
    }

    fn mouse_pos(&self, id: &WindowId) -> Option<(f32, f32)> {
        debug_assert!(self.0.get(id).is_some(), "Invalid window id: {:?}", id);
        self.0.get(id).and_then(|inner| inner.mouse_pos)
    }

    fn set_mouse_pos(&mut self, id: &WindowId, pos: Option<(f32, f32)>) {
        debug_assert!(self.0.get(id).is_some(), "Invalid window id: {:?}", id);
        if let Some(win) = self.0.get_mut(id) {
            win.mouse_pos = pos;
        }
    }

    fn size(&self, id: &WindowId) -> (u32, u32, f64) {
        debug_assert!(self.0.get(id).is_some(), "Invalid window id: {:?}", id);
        self.0.get(id).map_or((0, 0, 1.0), |win| {
            (win.size.0, win.size.1, win.scale_factor)
        })
    }

    fn set_size(&mut self, id: &WindowId, size: (u32, u32), scale_factor: f64) {
        debug_assert!(self.0.get(id).is_some(), "Invalid window id: {:?}", id);
        if let Some(win) = self.0.get_mut(id) {
            win.size = size;
            win.scale_factor = scale_factor;
        }
    }
}

struct InnerWindowData {
    id: WindowId,
    mouse_pos: Option<(f32, f32)>,
    size: (u32, u32),
    scale_factor: f64,
}

pub fn runner<S: AppState + 'static>(mut sys: System<S>) -> Result<(), String> {
    let event_loop = sys
        .get_mut_plugin::<App>()
        .ok_or("Cannot find Windows plugin.")?
        .manager
        .event_loop
        .take()
        .ok_or("Something went wrong acquiring the Winit's EventLoop.")?;

    let mut initialized_app = false;

    // track some inner data
    let mut inner_window_list = InnerWindowList::default();
    event_loop
        .run(move |evt, event_loop| {
            sys.get_mut_plugin::<App>()
                .unwrap()
                .manager
                .event_loop
                .set(event_loop);

            event_loop.set_control_flow(ControlFlow::Poll);

            println!(" --> {:?}", evt);

            match evt {
                // -- App life cycle events
                Event::Resumed => {
                    // init the app's logic on the first resumed event
                    if !initialized_app {
                        initialized_app = true;
                        sys.init();
                    }
                }
                Event::NewEvents(t) => {
                    match t {
                        StartCause::Init => {}
                        _ => {
                            sys.frame_start();
                            sys.update();
                        }
                    }
                }
                Event::AboutToWait => {
                    sys.frame_end();
                }
                Event::LoopExiting => {
                    sys.close();
                }
                // -- Windowing events
                Event::WindowEvent { window_id, event } => {
                    let windows = sys.get_mut_plugin::<App>().unwrap();
                    let id = win_id(window_id);
                    if let Some(win) = windows.window_by_id(id) {
                        win.request_redraw();
                        let scale_factor = win.scale();
                        inner_window_list.init_window(id, &mut sys);

                        match event {
                            WWindowEvent::RedrawRequested => {
                                // Sometimes this event comes before any WindowEvent
                                // Initializing windows here too we avoid a first blank frame
                                inner_window_list.init_window(id, &mut sys);
                                let (width, height, scale_factor) = inner_window_list.size(&id);

                                sys.event(DrawEvent {
                                    window_id: id,
                                    width,
                                    height,
                                    scale_factor,
                                });
                            }

                            // keyboard events
                            WWindowEvent::KeyboardInput { event, .. } => {
                                let evt = keyboard::process(id, event);
                                sys.event(evt);
                            }

                            // mouse events
                            WWindowEvent::MouseInput { state, button, .. } => {
                                let evt = mouse::process_input(
                                    id,
                                    state,
                                    button,
                                    inner_window_list.mouse_pos(&id),
                                );
                                sys.event(evt);
                            }
                            WWindowEvent::MouseWheel { delta, .. } => {
                                let evt = mouse::process_wheel(
                                    id,
                                    delta,
                                    scale_factor,
                                    inner_window_list.mouse_pos(&id),
                                );
                                sys.event(evt);
                            }
                            WWindowEvent::CursorMoved { position, .. } => {
                                let pos = position.to_logical::<f64>(scale_factor);
                                let evt = mouse::process_motion(
                                    id,
                                    pos.into(),
                                    inner_window_list.mouse_pos(&id),
                                );
                                inner_window_list
                                    .set_mouse_pos(&id, Some((pos.x as _, pos.y as _)));
                                sys.event(evt);
                            }
                            WWindowEvent::CursorEntered { .. } => {
                                let evt =
                                    mouse::process_enter(id, inner_window_list.mouse_pos(&id));
                                sys.event(evt);
                            }
                            WWindowEvent::CursorLeft { .. } => {
                                let evt =
                                    mouse::process_leave(id, inner_window_list.mouse_pos(&id));
                                sys.event(evt);
                            }

                            // window events
                            WWindowEvent::Resized(size) => {
                                let size = size.to_logical::<u32>(scale_factor);
                                inner_window_list.set_size(&id, size.into(), scale_factor);
                                sys.event(WindowEvent {
                                    id,
                                    action: WindowAction::Resized {
                                        width: size.width,
                                        height: size.height,
                                        scale_factor,
                                    },
                                });
                            }
                            WWindowEvent::Moved(pos) => {
                                let pos = pos.to_logical::<i32>(scale_factor);
                                sys.event(WindowEvent {
                                    id,
                                    action: WindowAction::Moved { x: pos.x, y: pos.y },
                                });
                            }
                            WWindowEvent::CloseRequested => {
                                let windows = sys.get_mut_plugin::<App>().unwrap();
                                windows.close(id);
                                sys.event(WindowEvent {
                                    id,
                                    action: WindowAction::Close,
                                });
                            }
                            WWindowEvent::Destroyed => {}
                            WWindowEvent::DroppedFile(_) => {}
                            WWindowEvent::HoveredFile(_) => {}
                            WWindowEvent::HoveredFileCancelled => {}
                            // WWindowEvent::ReceivedCharacter(_) => {}
                            WWindowEvent::Focused(focus) => {
                                sys.event(WindowEvent {
                                    id,
                                    action: if focus {
                                        WindowAction::FocusGained
                                    } else {
                                        WindowAction::FocusLost
                                    },
                                });
                            }
                            WWindowEvent::KeyboardInput { .. } => {}
                            WWindowEvent::ModifiersChanged(_) => {}
                            WWindowEvent::Ime(_) => {}
                            WWindowEvent::CursorMoved { .. } => {}
                            WWindowEvent::CursorEntered { .. } => {}
                            WWindowEvent::CursorLeft { .. } => {}
                            WWindowEvent::MouseWheel { .. } => {}
                            WWindowEvent::MouseInput { .. } => {}
                            WWindowEvent::TouchpadMagnify { .. } => {}
                            WWindowEvent::SmartMagnify { .. } => {}
                            WWindowEvent::TouchpadRotate { .. } => {}
                            WWindowEvent::TouchpadPressure { .. } => {}
                            WWindowEvent::AxisMotion { .. } => {}
                            WWindowEvent::Touch(_) => {}
                            WWindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                                // TODO
                                /*let size = new_inner_size.to_logical::<u32>(scale_factor);
                                inner_window_list.set_size(&id, size.into(), scale_factor);
                                sys.event(WindowEvent {
                                    id,
                                    action: WindowAction::Resized {
                                        width: size.width,
                                        height: size.height,
                                        scale_factor,
                                    },
                                });*/
                            }
                            WWindowEvent::ThemeChanged(_) => {}
                            WWindowEvent::Occluded(_) => {}
                            _ => {}
                        }
                    }
                }
                _ => (),
            }

            let manager = &mut sys.get_mut_plugin::<App>().unwrap().manager;
            manager.event_loop.unset();
            if manager.request_exit {
                println!("EXIT?");
                event_loop.exit();
            }
        })
        .map_err(|err| err.to_string())
}
