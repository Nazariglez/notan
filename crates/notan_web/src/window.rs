use crate::keyboard::{enable_keyboard, KeyboardCallbacks};
use crate::mouse::{enable_mouse, MouseCallbacks};
use crate::touch::{enable_touch, PointerCallbacks};
use crate::utils::{
    canvas_add_event_listener, canvas_mouse_passthrough, canvas_visible, get_notan_size,
    get_or_create_canvas, request_animation_frame, set_size_dpi, window_add_event_listener,
};
use notan_app::{CursorIcon, WindowConfig};
use notan_app::{Event, EventIterator, WindowBackend};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event as WebEvent, HtmlCanvasElement, Window};

#[cfg(feature = "clipboard")]
use crate::clipboard::{enable_clipboard, ClipboardCallbacks};

#[cfg(feature = "drop_files")]
use crate::files::{enable_files, FileCallbacks};

type RafType = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

pub struct WebWindowBackend {
    pub canvas: HtmlCanvasElement,
    pub window: Window,
    pub document: Document,
    pub canvas_parent: Element,
    pub dpi: f64,

    pub lazy: Rc<RefCell<bool>>,

    pub(crate) antialias: bool,
    pub(crate) transparent: bool,
    pub(crate) visible: bool,

    events: Rc<RefCell<EventIterator>>,

    fullscreen_requested: Rc<RefCell<Option<bool>>>,
    fullscreen_last_size: Rc<RefCell<Option<(i32, i32)>>>,
    fullscreen_callback_ref: Option<Closure<dyn FnMut(WebEvent)>>,

    min_size: Option<(i32, i32)>,
    max_size: Option<(i32, i32)>,
    resize_callback_ref: Option<Closure<dyn FnMut(WebEvent)>>,

    _context_menu_callback_ref: Closure<dyn FnMut(WebEvent)>,

    pub(crate) mouse_callbacks: MouseCallbacks,
    pub(crate) keyboard_callbacks: KeyboardCallbacks,
    pub(crate) touch_callbacks: PointerCallbacks,

    #[cfg(feature = "clipboard")]
    pub(crate) clipboard_callbacks: ClipboardCallbacks,

    #[cfg(feature = "drop_files")]
    pub(crate) file_callbacks: FileCallbacks,

    config: WindowConfig,

    raf: RafType,
    pub(crate) frame_requested: Rc<RefCell<bool>>,

    cursor: CursorIcon,

    capture_requested: Rc<RefCell<Option<bool>>>,
    pub(crate) captured: Rc<RefCell<bool>>,

    mouse_passthrough: bool,

    title: String,
    use_touch_as_mouse: bool,
}

impl WebWindowBackend {
    pub fn new(
        config: WindowConfig,
        events: Rc<RefCell<EventIterator>>,
        raf: RafType,
    ) -> Result<Self, String> {
        let window =
            web_sys::window().ok_or_else(|| String::from("Can't access window dom object."))?;
        let document = window
            .document()
            .ok_or("Can't access document dom object ")?;

        let canvas = get_or_create_canvas(&document, &config.canvas_id)?;

        let visible = config.visible;
        canvas_visible(&canvas, visible);
        canvas_mouse_passthrough(&canvas, config.mouse_passthrough);

        let canvas_parent = canvas
            .parent_element()
            .ok_or("Can't find the canvas parent element.")?;

        let context_menu_callback_ref =
            canvas_add_event_listener(&canvas, "contextmenu", |e: WebEvent| {
                e.prevent_default();
            })?;

        let fullscreen_requested = Rc::new(RefCell::new(None));
        let fullscreen_last_size = Rc::new(RefCell::new(None));
        let fullscreen_callback_ref = None;
        let capture_requested = Rc::new(RefCell::new(None));

        let min_size = config.min_size;
        let max_size = config.max_size;
        let resize_callback_ref = None;

        let mouse_callbacks = Default::default();
        let keyboard_callbacks = Default::default();
        let touch_callbacks = Default::default();

        #[cfg(feature = "clipboard")]
        let clipboard_callbacks = Default::default();

        #[cfg(feature = "drop_files")]
        let file_callbacks = Default::default();

        let antialias = config.multisampling != 0;
        let transparent = config.transparent;
        let mouse_passthrough = config.mouse_passthrough;

        let dpi = window.device_pixel_ratio();
        let lazy = Rc::new(RefCell::new(config.lazy_loop));
        let frame_requested = Rc::new(RefCell::new(false));

        let title = config.title.clone();

        let win = Self {
            window,
            document,
            canvas,
            canvas_parent,
            mouse_callbacks,
            keyboard_callbacks,
            touch_callbacks,

            #[cfg(feature = "clipboard")]
            clipboard_callbacks,

            #[cfg(feature = "drop_files")]
            file_callbacks,

            fullscreen_requested,
            fullscreen_last_size,
            fullscreen_callback_ref,
            events,
            min_size,
            max_size,
            resize_callback_ref,
            _context_menu_callback_ref: context_menu_callback_ref,
            config,
            antialias,
            transparent,
            dpi,
            lazy,

            raf,
            frame_requested,

            cursor: CursorIcon::Default,
            capture_requested,
            captured: Rc::new(RefCell::new(false)),
            visible,

            mouse_passthrough,
            title,
            use_touch_as_mouse: false,
        };

        win.init()
    }

    pub(crate) fn open_url(&self, url: &str, new_tab: bool) -> Result<(), String> {
        let target = if new_tab { "_blank" } else { "_self" };

        self.window
            .open_with_url_and_target(url, target)
            .map_err(|err| format!("{err:?}"))?;

        Ok(())
    }

    #[inline]
    pub(crate) fn init(mut self) -> Result<Self, String> {
        if let Err(e) = self
            .canvas
            .set_attribute("notan-auto-res", &self.config.high_dpi.to_string())
        {
            log::error!("{:?}", e);
        }

        let (ww, hh) = if self.config.maximized {
            (
                self.canvas_parent.client_width(),
                self.canvas_parent.client_height(),
            )
        } else {
            (self.config.width, self.config.height)
        };

        self.set_size(ww, hh);

        let fullscreen_dispatcher = fullscreen_dispatcher_callback(&mut self);

        enable_mouse(&mut self, fullscreen_dispatcher.clone())?;
        enable_keyboard(&mut self, fullscreen_dispatcher.clone())?;
        enable_touch(&mut self, fullscreen_dispatcher.clone())?;

        #[cfg(feature = "clipboard")]
        enable_clipboard(&mut self)?;

        #[cfg(feature = "drop_files")]
        enable_files(&mut self)?;

        if self.config.resizable {
            enable_resize(&mut self)?;
        }

        enable_fullscreen(&mut self)?;
        if self.config.fullscreen {
            self.set_fullscreen(true);
        }

        Ok(self)
    }

    pub(crate) fn check_dpi(&mut self) {
        let dpi = self.window.device_pixel_ratio();
        if (dpi - self.dpi).abs() > f64::EPSILON {
            let (ww, hh) = get_notan_size(&self.canvas);
            self.dpi = dpi;
            self.events
                .borrow_mut()
                .push(Event::ScreenAspectChange { ratio: dpi });
            self.set_size(ww as _, hh as _);
            self.request_frame();
        }
    }

    #[inline(always)]
    pub(crate) fn add_event_fn(&self) -> impl Fn(Event) {
        let win = self.window.clone();
        let events = self.events.clone();
        let raf = self.raf.clone();
        let lazy = self.lazy.clone();
        let frame_requested = self.frame_requested.clone();
        move |evt| {
            events.borrow_mut().push(evt);
            let needs_raf = *lazy.borrow() && !*frame_requested.borrow();
            if needs_raf {
                *frame_requested.borrow_mut() = true;
                request_animation_frame(&win, raf.borrow().as_ref().unwrap());
            }
        }
    }
}

impl WindowBackend for WebWindowBackend {
    fn capture_cursor(&self) -> bool {
        *self.captured.borrow()
    }

    fn cursor(&self) -> CursorIcon {
        self.cursor
    }

    fn dpi(&self) -> f64 {
        if self.config.high_dpi {
            self.dpi
        } else {
            1.0
        }
    }

    fn id(&self) -> u64 {
        0
    }

    // Unsupported in browser, always false
    fn is_always_on_top(&self) -> bool {
        false
    }

    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen()
    }

    fn lazy_loop(&self) -> bool {
        *self.lazy.borrow()
    }

    // No operation, as unsupported in browser
    fn mouse_passthrough(&mut self) -> bool {
        false
    }

    // No operation, as unsupported in browser
    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn request_frame(&mut self) {
        let needs_raf = self.lazy_loop() && !*self.frame_requested.borrow();
        if needs_raf {
            *self.frame_requested.borrow_mut() = true;
            request_animation_frame(&self.window, self.raf.borrow().as_ref().unwrap());
        }
    }

    fn screen_size(&self) -> (i32, i32) {
        let screen = self.window.screen().unwrap();

        let width = screen.width().unwrap();
        let height = screen.height().unwrap();
        (width, height)
    }

    fn container_size(&self) -> (i32, i32) {
        let width = self.canvas_parent.client_width();
        let height = self.canvas_parent.client_height();
        (width, height)
    }

    // No operation, as unsupported in browser
    fn set_always_on_top(&mut self, _enabled: bool) {}

    fn set_capture_cursor(&mut self, capture: bool) {
        *self.capture_requested.borrow_mut() = Some(capture);
    }

    fn set_cursor(&mut self, cursor: CursorIcon) {
        if cursor != self.cursor {
            self.cursor = cursor;
            let res = self
                .canvas
                .style()
                .set_property("cursor", web_cursor(cursor));
            match res {
                Ok(_) => {
                    self.cursor = cursor;
                }
                Err(err) => {
                    log::error!("{:?}", err);
                }
            }
        }
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        *self.fullscreen_requested.borrow_mut() = Some(enabled);
    }

    fn set_lazy_loop(&mut self, lazy: bool) {
        *self.lazy.borrow_mut() = lazy;
        if !lazy {
            self.request_frame();
        }
    }

    fn set_mouse_passthrough(&mut self, clickable: bool) {
        if self.mouse_passthrough != clickable {
            self.mouse_passthrough = clickable;
            canvas_mouse_passthrough(&self.canvas, clickable);
        }
    }

    // No operation, as unsupported in browser
    fn set_position(&mut self, _x: i32, _y: i32) {}

    fn set_size(&mut self, width: i32, height: i32) {
        set_size_dpi(&self.canvas, width as _, height as _);
        self.config.width = width;
        self.config.height = height;
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible != visible {
            self.visible = visible;
            canvas_visible(&self.canvas, visible);
        }
    }

    fn size(&self) -> (i32, i32) {
        get_notan_size(&self.canvas)
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn set_touch_as_mouse(&mut self, enable: bool) {
        self.use_touch_as_mouse = enable;
    }

    fn touch_as_mouse(&self) -> bool {
        self.use_touch_as_mouse
    }
}

unsafe impl Send for WebWindowBackend {}
unsafe impl Sync for WebWindowBackend {}

fn enable_fullscreen(win: &mut WebWindowBackend) -> Result<(), String> {
    if win.fullscreen_callback_ref.is_none() {
        let canvas = win.canvas.clone();
        let document = win.document.clone();
        let last_size = win.fullscreen_last_size.clone();
        let add_event = win.add_event_fn();
        win.fullscreen_callback_ref =
            Some(window_add_event_listener("fullscreenchange", move |_| {
                let (width, height) = if document.fullscreen() {
                    (canvas.client_width(), canvas.client_height())
                } else {
                    match *last_size.borrow() {
                        Some(size) => size,
                        _ => {
                            log::error!("Invalid fullscreen disabled size.");
                            (800, 600)
                        }
                    }
                };

                set_size_dpi(&canvas, width, height);
                add_event(Event::WindowResize { width, height });
            })?);
    }

    Ok(())
}

fn fullscreen_dispatcher_callback(win: &mut WebWindowBackend) -> Rc<RefCell<dyn Fn()>> {
    let fullscreen_requested = win.fullscreen_requested.clone();
    let canvas = win.canvas.clone();
    let doc = win.document.clone();
    let last_size = win.fullscreen_last_size.clone();

    let captured_request = win.capture_requested.clone();

    Rc::new(RefCell::new(move || {
        if let Some(full) = fullscreen_requested.borrow_mut().take() {
            if full {
                let width = canvas.client_width();
                let height = canvas.client_height();
                *last_size.borrow_mut() = Some((width, height));
                if let Err(e) = canvas.request_fullscreen() {
                    log::error!("{:?}", e);
                }
            } else {
                doc.exit_fullscreen();
            }
        }

        if let Some(capture) = captured_request.borrow_mut().take() {
            if capture {
                canvas.request_pointer_lock();
            } else {
                doc.exit_pointer_lock();
            }
        }
    }))
}

fn enable_resize(win: &mut WebWindowBackend) -> Result<(), String> {
    let canvas = win.canvas.clone();
    let parent = win.canvas_parent.clone();
    let min_size = win.min_size;
    let max_size = win.max_size;
    let add_event = win.add_event_fn();
    win.resize_callback_ref = Some(window_add_event_listener("resize", move |_| {
        let mut p_width = parent.client_width();
        let mut p_height = parent.client_height();

        if let Some((w, h)) = min_size {
            if p_width < w {
                p_width = w;
            }

            if p_height < h {
                p_height = h;
            }
        }

        if let Some((w, h)) = max_size {
            if p_width > w {
                p_width = w;
            }

            if p_height > h {
                p_height = h;
            }
        }

        set_size_dpi(&canvas, p_width as _, p_height as _);
        add_event(Event::WindowResize {
            width: p_width,
            height: p_height,
        });
    })?);
    Ok(())
}

fn web_cursor(cursor: CursorIcon) -> &'static str {
    match cursor {
        CursorIcon::Default => "default",
        CursorIcon::None => "none",
        CursorIcon::ContextMenu => "context-menu",
        CursorIcon::Help => "help",
        CursorIcon::PointingHand => "pointer",
        CursorIcon::Progress => "progress",
        CursorIcon::Wait => "wait",
        CursorIcon::Cell => "cell",
        CursorIcon::Crosshair => "crosshair",
        CursorIcon::Text => "text",
        CursorIcon::VerticalText => "vertical-text",
        CursorIcon::Alias => "alias",
        CursorIcon::Copy => "copy",
        CursorIcon::Move => "move",
        CursorIcon::NoDrop => "no-drop",
        CursorIcon::NotAllowed => "not-allowed",
        CursorIcon::Grab => "grab",
        CursorIcon::Grabbing => "grabbing",
        CursorIcon::AllScroll => "all-scroll",
        CursorIcon::ResizeHorizontal => "ew-resize",
        CursorIcon::ResizeNeSw => "nesw-resize",
        CursorIcon::ResizeNwSe => "nwse-resize",
        CursorIcon::ResizeVertical => "ns-resize",
        CursorIcon::ZoomIn => "zoom-in",
        CursorIcon::ZoomOut => "zoom-out",
        CursorIcon::ResizeEast => "e-resize",
        CursorIcon::ResizeSouthEast => "se-resize",
        CursorIcon::ResizeSouth => "s-resize",
        CursorIcon::ResizeSouthWest => "sw-resize",
        CursorIcon::ResizeWest => "w-resize",
        CursorIcon::ResizeNorthWest => "nw-resize",
        CursorIcon::ResizeNorth => "n-resize",
        CursorIcon::ResizeNorthEast => "ne-resize",
        CursorIcon::ResizeColumn => "col-resize",
        CursorIcon::ResizeRow => "row-resize",
    }
}
