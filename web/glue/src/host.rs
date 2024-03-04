//! Implementations of the traits in win32/host.rs, providing the hosting API for the emulator.

use anyhow::bail;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type JsSurface;
    #[wasm_bindgen(method)]
    fn write_pixels(this: &JsSurface, pixels: &[u8]) -> JsSurface;
    #[wasm_bindgen(method)]
    fn show(this: &JsSurface);
    #[wasm_bindgen(method)]
    fn bit_blt(
        this: &JsSurface,
        dx: u32,
        dy: u32,
        src: &JsSurface,
        sx: u32,
        sy: u32,
        w: u32,
        h: u32,
    );
}

impl win32::Surface for JsSurface {
    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        let slice = unsafe {
            let p = pixels.as_ptr() as *const u8;
            std::slice::from_raw_parts(p, pixels.len() * 4)
        };
        JsSurface::write_pixels(self, slice);
    }

    fn show(&mut self) {
        JsSurface::show(self);
    }

    fn bit_blt(
        &mut self,
        dx: u32,
        dy: u32,
        src: &dyn win32::Surface,
        sx: u32,
        sy: u32,
        w: u32,
        h: u32,
    ) {
        // Hack: we know all surfaces are JsSurface.
        // I think to fix this properly I might need to make every X86 generic across all the
        // host types, eek.
        let src = unsafe { &*(src as *const dyn win32::Surface as *const JsSurface) };
        JsSurface::bit_blt(self, dx, dy, src, sx, sy, w, h);
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsWindow;
    #[wasm_bindgen(method, setter)]
    fn set_title(this: &JsWindow, title: &str);
    #[wasm_bindgen(method)]
    fn set_size(this: &JsWindow, width: u32, height: u32);
}

impl win32::Window for JsWindow {
    fn set_title(&mut self, title: &str) {
        JsWindow::set_title(self, title);
    }
    fn set_size(&mut self, width: u32, height: u32) {
        JsWindow::set_size(self, width, height);
    }
}

#[wasm_bindgen]
extern "C" {
    pub type JsFile;
    #[wasm_bindgen(method)]
    fn seek(this: &JsFile, ofs: u32) -> bool;
    #[wasm_bindgen(method)]
    fn read(this: &JsFile, buf: &mut [u8]) -> u32;
}

impl win32::File for JsFile {
    fn seek(&mut self, ofs: u32) -> bool {
        JsFile::seek(self, ofs)
    }

    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool {
        let n = JsFile::read(self, buf);
        *len = n;
        true
    }
}

fn map_mousevent(event: web_sys::MouseEvent) -> anyhow::Result<win32::MouseMessage> {
    Ok(win32::MouseMessage {
        down: true,
        button: match event.button() {
            0 => win32::MouseButton::Left,
            1 => win32::MouseButton::Middle,
            2 => win32::MouseButton::Right,
            _ => bail!("unhandled button"),
        },
        x: event.offset_x() as u32,
        y: event.offset_y() as u32,
    })
}

fn message_from_event(event: web_sys::Event) -> anyhow::Result<win32::Message> {
    let hwnd = js_sys::Reflect::get(&event, &JsValue::from_str("hwnd"))
        .unwrap()
        .as_f64()
        .unwrap() as u32;
    let detail = match event.type_().as_str() {
        "mousedown" => {
            let mut event = map_mousevent(event.unchecked_into::<web_sys::MouseEvent>())?;
            event.down = true;
            win32::MessageDetail::Mouse(event)
        }
        "mouseup" => {
            let mut event = map_mousevent(event.unchecked_into::<web_sys::MouseEvent>())?;
            event.down = false;
            win32::MessageDetail::Mouse(event)
        }
        ty => bail!("unhandled event type {ty}"),
    };
    log::info!("msg: {:?}", detail);
    Ok(win32::Message { hwnd, detail })
}

#[wasm_bindgen]
extern "C" {
    pub type JsHost;

    #[wasm_bindgen(method)]
    fn log(this: &JsHost, level: u8, msg: String);

    #[wasm_bindgen(method)]
    fn exit(this: &JsHost, exit_code: u32);

    #[wasm_bindgen(method)]
    fn get_event(this: &JsHost) -> web_sys::Event;

    #[wasm_bindgen(method)]
    fn open(this: &JsHost, path: &str) -> JsFile;
    #[wasm_bindgen(method)]
    fn write(this: &JsHost, buf: &[u8]) -> usize;

    #[wasm_bindgen(method)]
    fn create_window(this: &JsHost, hwnd: u32) -> JsWindow;
    #[wasm_bindgen(method)]
    fn create_surface(this: &JsHost, opts: win32::SurfaceOptions) -> JsSurface;
}

impl win32::Host for JsHost {
    fn exit(&self, exit_code: u32) {
        JsHost::exit(self, exit_code)
    }

    fn time(&self) -> u32 {
        web_sys::window().unwrap().performance().unwrap().now() as u32
    }

    fn get_message(&self, wait: bool) -> Option<win32::Message> {
        assert_eq!(wait, false); // waiting implemented via async blocking
        let event = JsHost::get_event(self);
        if event.is_undefined() {
            return None;
        }
        message_from_event(event)
            .inspect_err(|err| log::error!("failed to convert message: {err}"))
            .ok()
    }

    fn open(&self, path: &str) -> Box<dyn win32::File> {
        let file = JsHost::open(self, path);
        Box::new(file)
    }

    fn write(&self, buf: &[u8]) -> usize {
        JsHost::write(self, buf)
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        let window = JsHost::create_window(self, hwnd);
        Box::new(window)
    }

    fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        Box::new(JsHost::create_surface(self, opts.clone()))
    }
}
