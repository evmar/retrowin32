//! Implementations of the traits in win32/host.rs, providing the hosting API for the emulator.

use anyhow::bail;
use wasm_bindgen::prelude::*;

struct WebSurface {
    _hwnd: u32,
    canvas: web_sys::HtmlCanvasElement,
    width: u32,
    ctx: web_sys::CanvasRenderingContext2d,
    screen: web_sys::CanvasRenderingContext2d,
}

impl WebSurface {
    pub fn new(
        hwnd: u32,
        opts: &win32::SurfaceOptions,
        screen: web_sys::CanvasRenderingContext2d,
    ) -> Self {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .unchecked_into::<web_sys::HtmlCanvasElement>();
        canvas.set_width(opts.width);
        canvas.set_height(opts.height);
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill_rect(0.0, 0.0, opts.width as f64, opts.height as f64);
        ctx.fill();
        Self {
            _hwnd: hwnd,
            canvas,
            width: opts.width,
            ctx,
            screen,
        }
    }
}

impl win32::Surface for WebSurface {
    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        let slice =
            unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const _, pixels.len() * 4) };
        let image_data =
            web_sys::ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(slice), self.width)
                .unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

    fn show(&mut self) {
        self.screen
            .draw_image_with_html_canvas_element(&self.canvas, 0.0, 0.0)
            .unwrap();
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
        // Hack: we know all surfaces are WebSurface.
        // I think to fix this properly I might need to make every X86 generic across all the
        // host types, eek.
        let src = unsafe { &*(src as *const dyn win32::Surface as *const WebSurface) };
        self.ctx
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &src.canvas,
                sx as f64,
                sy as f64,
                w as f64,
                h as f64,
                dx as f64,
                dy as f64,
                w as f64,
                h as f64,
            )
            .unwrap();
    }
}

#[wasm_bindgen(typescript_custom_section)]
const JSWINDOW_TS: &'static str = r#"
export interface JsWindow {
  title: string;
  set_size(width: number, height: number): void;
}"#;

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

    fn fullscreen(&mut self) {
        log::warn!("todo: fullscreen");
    }
}

#[wasm_bindgen(typescript_custom_section)]
const JSFILE_TS: &'static str = r#"
export interface JsFile {
  info(): number;
  seek(ofs: number): boolean;
  read(buf: Uint8Array): number;
}"#;

#[wasm_bindgen]
extern "C" {
    pub type JsFile;
    #[wasm_bindgen(method)]
    fn info(this: &JsFile) -> u32;
    #[wasm_bindgen(method)]
    fn seek(this: &JsFile, ofs: u32) -> bool;
    #[wasm_bindgen(method)]
    fn read(this: &JsFile, buf: &mut [u8]) -> u32;
}

impl win32::File for JsFile {
    fn info(&self) -> u32 {
        JsFile::info(self)
    }

    fn seek(&mut self, ofs: u32) -> bool {
        JsFile::seek(self, ofs)
    }
}

impl std::io::Read for JsFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = JsFile::read(self, buf);
        Ok(n as usize)
    }
}

impl std::io::Write for JsFile {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
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
    let time = event.time_stamp() as u32;
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
    Ok(win32::Message { hwnd, detail, time })
}

#[wasm_bindgen(typescript_custom_section)]
const JSHOST_TS: &'static str = r#"
export interface JsHost {
  exit(code: number): void;
  
  ensure_timer(when: number): void;
  get_event(): Event | undefined;
  
  open(path: string): JsFile;
  log(buf: Uint8Array);
  
  create_window(hwnd: number): JsWindow;
}"#;

#[wasm_bindgen]
extern "C" {
    pub type JsHost;

    #[wasm_bindgen(method)]
    fn log(this: &JsHost, level: u8, msg: String);

    #[wasm_bindgen(method)]
    fn exit(this: &JsHost, exit_code: u32);

    #[wasm_bindgen(method)]
    fn ensure_timer(this: &JsHost, when: u32);

    #[wasm_bindgen(method)]
    fn get_event(this: &JsHost) -> web_sys::Event;

    #[wasm_bindgen(method)]
    fn open(this: &JsHost, path: &str) -> JsFile;
    #[wasm_bindgen(method)]
    fn stdout(this: &JsHost, buf: &[u8]);

    #[wasm_bindgen(method)]
    fn create_window(this: &JsHost, hwnd: u32) -> JsWindow;

    #[wasm_bindgen(method)]
    fn screen(this: &JsHost) -> web_sys::CanvasRenderingContext2d;
}

impl win32::Host for JsHost {
    fn exit(&self, exit_code: u32) {
        JsHost::exit(self, exit_code)
    }

    fn time(&self) -> u32 {
        web_sys::window().unwrap().performance().unwrap().now() as u32
    }

    fn get_message(&self) -> Option<win32::Message> {
        let event = JsHost::get_event(self);
        if event.is_undefined() {
            return None;
        }
        message_from_event(event)
            .inspect_err(|err| log::error!("failed to convert message: {err}"))
            .ok()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        if let Some(t) = wait {
            // Enqueue a timer to wake up caller.
            JsHost::ensure_timer(self, t);
        }
        false
    }

    fn open(&self, path: &str, access: win32::FileAccess) -> Box<dyn win32::File> {
        match access {
            win32::FileAccess::READ => Box::new(JsHost::open(self, path)),
            win32::FileAccess::WRITE => todo!(),
        }
    }

    fn log(&self, buf: &[u8]) {
        JsHost::stdout(self, buf)
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        let window = JsHost::create_window(self, hwnd);
        Box::new(window)
    }

    fn create_surface(
        &mut self,
        hwnd: u32,
        opts: &win32::SurfaceOptions,
    ) -> Box<dyn win32::Surface> {
        Box::new(WebSurface::new(hwnd, opts, JsHost::screen(self)))
    }
}
