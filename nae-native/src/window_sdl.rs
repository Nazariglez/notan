use crate::{System, ToNaeValue};
use nae_core::window::BaseWindow;
use nae_core::{BaseApp, BuilderOpts, Event, KeyCode, MouseButton};
use sdl2::keyboard::{Keycode as SdlKeycode, Scancode};
use sdl2::mouse::MouseButton as SdlMouseButton;
use sdl2::video::{FullscreenType, Window as SdlWindow};
use sdl2::{Sdl, VideoSubsystem};

pub struct Window {
    pub(crate) sdl: sdl2::Sdl,
    pub(crate) video: VideoSubsystem,
    pub(crate) win: SdlWindow,
}

impl Window {
    pub(crate) fn new(opts: &BuilderOpts) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        let mut win_builder = video.window(&opts.title, opts.width as _, opts.height as _);

        win_builder.opengl();

        if opts.resizable {
            win_builder.resizable();
        }

        if opts.maximized {
            win_builder.maximized();
        }

        if opts.fullscreen {
            win_builder.fullscreen();
        }

        // TODO add all funcionality like min_size or max_size

        let mut win = win_builder.build().map_err(|e| e.to_string())?;

        if let Some((width, height)) = opts.min_size {
            win.set_minimum_size(width as _, height as _);
        }

        if let Some((width, height)) = opts.max_size {
            win.set_maximum_size(width as _, height as _);
        }

        dbg!(video.display_dpi(0));

        Ok(Self { sdl, video, win })
    }

    pub(crate) fn set_fullscreen(&mut self, full: bool) {
        let state = if full {
            FullscreenType::True
        } else {
            FullscreenType::Off
        };
        self.win.set_fullscreen(state);
    }
}

impl BaseWindow for Window {
    fn width(&self) -> i32 {
        let (width, _) = self.win.size();
        width as _
    }

    fn height(&self) -> i32 {
        let (_, height) = self.win.size();
        height as _
    }

    fn fullscreen(&self) -> bool {
        match self.win.fullscreen_state() {
            FullscreenType::True => true,
            _ => false,
        }
    }

    fn title(&self) -> &str {
        self.win.title()
    }

    fn dpi(&self) -> f32 {
        //TODO set the real value
        1.0
    }
}

pub fn run<A, S, F, D>(mut app: A, mut state: S, mut update: F, mut draw: D) -> Result<(), String>
where
    A: BaseApp<System = System> + 'static,
    S: 'static,
    F: FnMut(&mut A, &mut S) + 'static,
    D: FnMut(&mut A, &mut S) + 'static,
{
    use sdl2::event::{Event as SdlEvent, WindowEvent};
    let mut event_pump = app
        .system()
        .window
        .sdl
        .event_pump()
        .map_err(|e| e.to_string())?;

    let mut running = true;
    let (mut last_mouse_x, mut last_mouse_y) = (0, 0);
    while running {
        for evt in event_pump.poll_iter() {
            match evt {
                SdlEvent::Quit { .. } => running = false,
                SdlEvent::Window { win_event, .. } => match &win_event {
                    WindowEvent::Resized(width, height) => {
                        //TODO Dpi?
                        app.system().events.push(Event::WindowResize {
                            width: *width,
                            height: *height,
                        });
                    }
                    WindowEvent::Leave => {
                        app.system().events.push(Event::MouseLeft {
                            x: last_mouse_x,
                            y: last_mouse_y,
                        });
                    }
                    WindowEvent::Leave => {
                        app.system().events.push(Event::MouseEnter {
                            x: last_mouse_x,
                            y: last_mouse_y,
                        });
                    }
                    _ => {}
                },
                SdlEvent::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    last_mouse_x = x;
                    last_mouse_y = y;
                    app.system().events.push(Event::MouseDown {
                        button: mouse_btn.to_nae(),
                        x,
                        y,
                    });
                }
                SdlEvent::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    last_mouse_x = x;
                    last_mouse_y = y;
                    app.system().events.push(Event::MouseUp {
                        button: mouse_btn.to_nae(),
                        x,
                        y,
                    });
                }
                SdlEvent::MouseMotion { x, y, .. } => {
                    last_mouse_x = x;
                    last_mouse_y = y;
                    app.system().events.push(Event::MouseMove { x, y });
                }
                SdlEvent::MouseWheel { x, y, .. } => {
                    let delta_x = x as f32 / 10.0;
                    let delta_y = y as f32 / 10.0;
                    app.system()
                        .events
                        .push(Event::MouseWheel { delta_x, delta_y })
                }
                SdlEvent::KeyDown {
                    keycode, scancode, ..
                } => {
                    let key = (keycode, scancode).to_nae();
                    app.system().events.push(Event::KeyDown { key });
                }
                SdlEvent::KeyUp {
                    keycode, scancode, ..
                } => {
                    let key = (keycode, scancode).to_nae();
                    app.system().events.push(Event::KeyUp { key });
                }
                SdlEvent::TextInput { text, .. } => {
                    let mut chars = text.chars();
                    if let Some(c) = chars.next() {
                        app.system().events.push(Event::ReceivedCharacter(c));
                    }
                }
                _ => {}
            }
        }

        update(&mut app, &mut state);
        draw(&mut app, &mut state);
        app.system().window.win.gl_swap_window();
    }
    Ok(())
}

impl ToNaeValue for SdlMouseButton {
    type Kind = MouseButton;

    fn to_nae(&self) -> Self::Kind {
        match &self {
            SdlMouseButton::Right => MouseButton::Right,
            SdlMouseButton::Left => MouseButton::Left,
            SdlMouseButton::Middle => MouseButton::Middle,
            SdlMouseButton::X1 => MouseButton::Other(1),
            SdlMouseButton::X2 => MouseButton::Other(2),
            SdlMouseButton::Unknown => MouseButton::Other(0),
        }
    }
}

/* TODO this sdl keycode to winit keycode are just tested on mac and maybe other platforms
    behave in some other way. Also, maybe some key could be missing, and jus tto think about
    it's better use sdl scancodes to winit virtual keycodes?
*/
impl ToNaeValue for (Option<SdlKeycode>, Option<Scancode>) {
    type Kind = KeyCode;

    fn to_nae(&self) -> Self::Kind {
        use SdlKeycode::*;
        match self.0 {
            Some(k) => match k {
                Backspace => KeyCode::Back,
                Tab => KeyCode::Tab,
                Return => KeyCode::Return,
                Escape => KeyCode::Escape,
                Space => KeyCode::Space,
                //                Exclaim => KeyCode::Exclaim,
                //                Quotedbl => KeyCode::Quotedbl,
                //                Hash => KeyCode::Hash,
                //                Dollar => KeyCode::Dollar,
                //                Percent => KeyCode::Percent,
                //                Ampersand => KeyCode::Ampersand,
                Quote => KeyCode::Apostrophe,
                //                LeftParen => KeyCode::LeftParen,
                //                RightParen => KeyCode::RightParen,
                Asterisk => KeyCode::Multiply,
                Plus => KeyCode::Equals,
                Comma => KeyCode::Comma,
                Minus => KeyCode::Minus,
                Period => KeyCode::Period,
                Slash => KeyCode::Slash,
                Num0 => KeyCode::Key0,
                Num1 => KeyCode::Key1,
                Num2 => KeyCode::Key2,
                Num3 => KeyCode::Key3,
                Num4 => KeyCode::Key4,
                Num5 => KeyCode::Key5,
                Num6 => KeyCode::Key6,
                Num7 => KeyCode::Key7,
                Num8 => KeyCode::Key8,
                Num9 => KeyCode::Key9,
                Colon => KeyCode::Colon,
                Semicolon => KeyCode::Semicolon,
                Less => KeyCode::Caret,
                Equals => KeyCode::Equals,
                //                Greater => KeyCode::Greater,
                //                Question => KeyCode::Question,
                At => KeyCode::At,
                LeftBracket => KeyCode::LBracket,
                Backslash => KeyCode::Backslash,
                RightBracket => KeyCode::RBracket,
                Caret => KeyCode::Caret,
                Underscore => KeyCode::Underline,
                Backquote => KeyCode::Grave,
                A => KeyCode::A,
                B => KeyCode::B,
                C => KeyCode::C,
                D => KeyCode::D,
                E => KeyCode::E,
                F => KeyCode::F,
                G => KeyCode::G,
                H => KeyCode::H,
                I => KeyCode::I,
                J => KeyCode::J,
                K => KeyCode::K,
                L => KeyCode::L,
                M => KeyCode::M,
                N => KeyCode::N,
                O => KeyCode::O,
                P => KeyCode::P,
                Q => KeyCode::Q,
                R => KeyCode::R,
                S => KeyCode::S,
                T => KeyCode::T,
                U => KeyCode::U,
                V => KeyCode::V,
                W => KeyCode::W,
                X => KeyCode::X,
                Y => KeyCode::Y,
                Z => KeyCode::Z,
                Delete => KeyCode::Delete,
                //                CapsLock => KeyCode::CapsLock,
                F1 => KeyCode::F1,
                F2 => KeyCode::F2,
                F3 => KeyCode::F3,
                F4 => KeyCode::F4,
                F5 => KeyCode::F5,
                F6 => KeyCode::F6,
                F7 => KeyCode::F7,
                F8 => KeyCode::F8,
                F9 => KeyCode::F9,
                F10 => KeyCode::F10,
                F11 => KeyCode::F11,
                F12 => KeyCode::F12,
                PrintScreen => KeyCode::Snapshot,
                ScrollLock => KeyCode::Scroll,
                Pause => KeyCode::Pause,
                Insert => KeyCode::Insert,
                Home => KeyCode::Home,
                PageUp => KeyCode::PageUp,
                End => KeyCode::End,
                PageDown => KeyCode::PageDown,
                Right => KeyCode::Right,
                Left => KeyCode::Left,
                Down => KeyCode::Down,
                Up => KeyCode::Up,
                NumLockClear => KeyCode::Numlock,
                KpDivide => KeyCode::Divide,
                KpMultiply => KeyCode::Multiply,
                KpMinus => KeyCode::Minus,
                //                KpPlus => KeyCode::Plus,
                //                KpEnter => KeyCode::Enter,
                Kp1 => KeyCode::Numpad1,
                Kp2 => KeyCode::Numpad2,
                Kp3 => KeyCode::Numpad3,
                Kp4 => KeyCode::Numpad4,
                Kp5 => KeyCode::Numpad5,
                Kp6 => KeyCode::Numpad6,
                Kp7 => KeyCode::Numpad7,
                Kp8 => KeyCode::Numpad8,
                Kp9 => KeyCode::Numpad9,
                Kp0 => KeyCode::Numpad0,
                KpPeriod => KeyCode::Period,
                Application => KeyCode::Apps,
                Power => KeyCode::Power,
                KpEquals => KeyCode::Equals,
                F13 => KeyCode::F13,
                F14 => KeyCode::F14,
                F15 => KeyCode::F15,
                F16 => KeyCode::F16,
                F17 => KeyCode::F17,
                F18 => KeyCode::F18,
                F19 => KeyCode::F19,
                F20 => KeyCode::F20,
                F21 => KeyCode::F21,
                F22 => KeyCode::F22,
                F23 => KeyCode::F23,
                F24 => KeyCode::F24,
                //                Execute => KeyCode::Execute,
                //                Help => KeyCode::Help,
                //                Menu => KeyCode::Menu,
                //                Select => KeyCode::Select,
                Stop => KeyCode::Stop,
                //                Again => KeyCode::Again,
                //                Undo => KeyCode::Undo,
                Cut => KeyCode::Cut,
                Copy => KeyCode::Copy,
                Paste => KeyCode::Paste,
                //                Find => KeyCode::Find,
                Mute => KeyCode::Mute,
                VolumeUp => KeyCode::VolumeUp,
                VolumeDown => KeyCode::VolumeDown,
                KpComma => KeyCode::Comma,
                KpEqualsAS400 => KeyCode::Equals,
                //                AltErase => KeyCode::AltErase,
                //                Sysreq => KeyCode::Sysreq,
                //                Cancel => KeyCode::Cancel,
                //                Clear => KeyCode::Clear,
                //                Prior => KeyCode::Prior,
                //                Return2 => KeyCode::Return2,
                //                Separator => KeyCode::Separator,
                //                Out => KeyCode::Out,
                //                Oper => KeyCode::Oper,
                //                ClearAgain => KeyCode::ClearAgain,
                //                CrSel => KeyCode::CrSel,
                //                ExSel => KeyCode::ExSel,
                //                Kp00 => KeyCode::Kp00,
                //                Kp000 => KeyCode::Kp000,
                //                ThousandsSeparator => KeyCode::ThousandsSeparator,
                //                DecimalSeparator => KeyCode::DecimalSeparator,
                //                CurrencyUnit => KeyCode::CurrencyUnit,
                //                CurrencySubUnit => KeyCode::CurrencySubUnit,
                //                KpLeftParen => KeyCode::KpLeftParen,
                //                KpRightParen => KeyCode::KpRightParen,
                //                KpLeftBrace => KeyCode::KpLeftBrace,
                //                KpRightBrace => KeyCode::RBrace,
                KpTab => KeyCode::Tab,
                KpBackspace => KeyCode::Back,
                //                KpA => KeyCode::KpA,
                //                KpB => KeyCode::KpB,
                //                KpC => KeyCode::KpC,
                //                KpD => KeyCode::KpD,
                //                KpE => KeyCode::KpE,
                //                KpF => KeyCode::KpF,
                //                KpXor => KeyCode::Xor,
                KpPower => KeyCode::Power,
                //                KpPercent => KeyCode::Percent,
                //                KpLess => KeyCode::Less,
                //                KpGreater => KeyCode::Greater,
                //                KpAmpersand => KeyCode::Ampersand,
                //                KpDblAmpersand => KeyCode::DblAmpersand,
                //                KpVerticalBar => KeyCode::VerticalBar,
                //                KpDblVerticalBar => KeyCode::DblVerticalBar,
                KpColon => KeyCode::Colon,
                //                KpHash => KeyCode::Hash,
                KpSpace => KeyCode::Space,
                KpAt => KeyCode::At,
                //                KpExclam => KeyCode::Exclam,
                //                KpMemStore => KeyCode::MemStore,
                //                KpMemRecall => KeyCode::MemRecall,
                //                KpMemClear => KeyCode::MemClear,
                //                KpMemAdd => KeyCode::MemAdd,
                //                KpMemSubtract => KeyCode::MemSubtract,
                //                KpMemMultiply => KeyCode::MemMultiply,
                //                KpMemDivide => KeyCode::MemDivide,
                //                KpPlusMinus => KeyCode::PlusMinus,
                //                KpClear => KeyCode::Clear,
                //                KpClearEntry => KeyCode::ClearEntry,
                //                KpBinary => KeyCode::Binary,
                //                KpOctal => KeyCode::Octal,
                KpDecimal => KeyCode::Decimal,
                //                KpHexadecimal => KeyCode::Hexadecimal,
                LCtrl => KeyCode::LControl,
                LShift => KeyCode::LShift,
                LAlt => KeyCode::LAlt,
                LGui => KeyCode::LWin,
                RCtrl => KeyCode::RControl,
                RShift => KeyCode::RShift,
                RAlt => KeyCode::RAlt,
                RGui => KeyCode::RWin,
                //                Mode => KeyCode::Mode,
                AudioNext => KeyCode::NextTrack,
                AudioPrev => KeyCode::PrevTrack,
                AudioStop => KeyCode::Stop,
                AudioPlay => KeyCode::PlayPause,
                AudioMute => KeyCode::Mute,
                MediaSelect => KeyCode::MediaSelect,
                //                Www => KeyCode::Www,
                Mail => KeyCode::Mail,
                Calculator => KeyCode::Calculator,
                Computer => KeyCode::MyComputer,
                //                AcSearch => KeyCode::Search,
                AcHome => KeyCode::Home,
                AcBack => KeyCode::Back,
                //                AcForward => KeyCode::Forward,
                AcStop => KeyCode::Stop,
                //                AcRefresh => KeyCode::Refresh,
                //                AcBookmarks => KeyCode::Bookmarks,
                //                BrightnessDown => KeyCode::BrightnessDown,
                //                BrightnessUp => KeyCode::BrightnessUp,
                //                DisplaySwitch => KeyCode::DisplaySwitch,
                //                KbdIllumToggle => KeyCode::KbdIllumToggle,
                //                KbdIllumDown => KeyCode::KbdIllumDown,
                //                KbdIllumUp => KeyCode::KbdIllumUp,
                //                Eject => KeyCode::Eject,
                Sleep => KeyCode::Sleep,
                _ => KeyCode::Unknown,
            },
            _ => match self.1 {
                Some(k) => match k {
                    Scancode::NonUsBackslash => KeyCode::Comma,
                    Scancode::Equals => KeyCode::Equals,
                    Scancode::Backslash => KeyCode::Backslash,
                    Scancode::Apostrophe => KeyCode::Apostrophe,
                    _ => KeyCode::Unknown,
                },
                _ => KeyCode::Unknown,
            },
        }
    }
}
