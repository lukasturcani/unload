use dioxus::prelude::*;
use std::sync::Once;

#[allow(dead_code)]
static INIT: Once = Once::new();

#[derive(Clone, Copy, Debug)]
pub struct WindowSize {
    /// The horizontal size in pixels.
    pub width: u32,
    /// The vertical size in pixels.
    #[allow(dead_code)]
    pub height: u32,
}

pub fn use_window_size() -> ReadOnlySignal<WindowSize> {
    let window_size = match try_use_context::<Signal<WindowSize>>() {
        Some(w) => w,
        // This should only run once.
        None => {
            let signal = Signal::new_in_scope(get_window_size(), ScopeId::ROOT);
            let size = provide_root_context(signal);
            listen(size);

            size
        }
    };

    use_hook(|| ReadOnlySignal::new(window_size))
}

#[cfg(target_family = "wasm")]
fn listen(mut window_size: Signal<WindowSize>) {
    use wasm_bindgen::{closure::Closure, JsCast, JsValue};

    INIT.call_once(|| {
        let window = web_sys::window().expect("no wasm window found; are you in wasm?");
        let window2 = window.clone();

        // We will fail silently for conversion errors.
        let on_resize = Closure::wrap(Box::new(move || {
            let height = window2
                .inner_height()
                .unwrap_or(JsValue::from_f64(0.0))
                .as_f64()
                .unwrap_or(0.0) as u32;

            let width = window2
                .inner_width()
                .unwrap_or(JsValue::from_f64(0.0))
                .as_f64()
                .unwrap_or(0.0) as u32;

            window_size.set(WindowSize { width, height });
        }) as Box<dyn FnMut()>);

        let on_resize_cb = on_resize.as_ref().clone();
        on_resize.forget();
        window.set_onresize(Some(on_resize_cb.unchecked_ref()));
    });
}

#[cfg(not(target_family = "wasm"))]
fn listen(mut window_size: Signal<WindowSize>) {
    use dioxus::mobile::{tao::event::Event, window, WindowEvent};

    let window = window();
    window.create_wry_event_handler(move |event, _| {
        if let Event::WindowEvent {
            event: WindowEvent::Resized(size),
            ..
        } = event
        {
            window_size.set(WindowSize {
                width: size.width,
                height: size.height,
            });
        }
    });
}

pub fn get_window_size() -> WindowSize {
    get_window_size_platform()
}

#[cfg(target_family = "wasm")]
fn get_window_size_platform() -> WindowSize {
    use wasm_bindgen::JsValue;
    let window = web_sys::window().expect("no wasm window found; are you in wasm?");

    // We will fail silently for conversion errors.
    let height = window
        .inner_height()
        .unwrap_or(JsValue::from_f64(0.0))
        .as_f64()
        .unwrap_or(0.0) as u32;

    let width = window
        .inner_width()
        .unwrap_or(JsValue::from_f64(0.0))
        .as_f64()
        .unwrap_or(0.0) as u32;

    WindowSize { width, height }
}

#[cfg(not(target_family = "wasm"))]
fn get_window_size_platform() -> WindowSize {
    let window = dioxus::mobile::window();
    let size = window.inner_size();
    WindowSize {
        width: size.width,
        height: size.height,
    }
}
