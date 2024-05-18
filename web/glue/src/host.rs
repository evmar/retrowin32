//! Implementations of the traits in win32/host.rs, providing the hosting API for the emulator.

use std::collections::VecDeque;

use anyhow::bail;
use wasm_bindgen::prelude::*;

struct WebSurface {
    js_host: JsHost,
    hwnd: u32,
    canvas: web_sys::OffscreenCanvas,
    width: u32,
    ctx: web_sys::OffscreenCanvasRenderingContext2d,
}

impl WebSurface {
    pub fn new(js_host: JsHost, hwnd: u32, opts: &win32::SurfaceOptions) -> Self {
        let canvas = web_sys::OffscreenCanvas::new(opts.width, opts.height).unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .unwrap();
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill_rect(0.0, 0.0, opts.width as f64, opts.height as f64);
        ctx.fill();
        Self {
            js_host,
            hwnd,
            canvas,
            width: opts.width,
            ctx,
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
        let bitmap = self.canvas.transfer_to_image_bitmap().unwrap();
        self.js_host.window_show(self.hwnd, bitmap);
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
            .draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
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

struct Window {
    js_host: JsHost,
    hwnd: u32,
}

impl win32::Window for Window {
    fn set_title(&mut self, title: &str) {
        self.js_host.window_set_title(self.hwnd, title);
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.js_host.window_set_size(self.hwnd, width, height);
    }

    fn fullscreen(&mut self) {
        log::warn!("todo: fullscreen");
    }
}

pub struct File {
    content: Box<[u8]>,
    ofs: usize,
}

impl win32::File for File {
    fn info(&self) -> u32 {
        log::info!("len {}", self.content.len());
        self.content.len() as u32
    }

    fn seek(&mut self, ofs: u32) -> bool {
        self.ofs = ofs as usize;
        true
    }

    fn read(&mut self, buf: &mut [u8], len: &mut u32) -> bool {
        let n = std::cmp::min(buf.len(), self.content.len() - self.ofs);
        buf[..n].copy_from_slice(&self.content[self.ofs..][..n]);
        self.ofs += n;
        *len = n as u32;
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

#[wasm_bindgen(typescript_custom_section)]
const HOST_TS: &'static str = r#"
export interface JsHost {
    exit(code: number): void;
    write(buf: Uint8Array);
    window_create(hwnd: number);
    window_set_title(hwnd: number, title: string);
    window_set_size(hwnd: number, w: number, h: number);
    window_show(hwnd: number, pixels: ImageBitmap);
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsHost")]
    pub type JsHost;

    #[wasm_bindgen(method)]
    fn exit(this: &JsHost, exit_code: u32);

    // #[wasm_bindgen(method)]
    // fn ensure_timer(this: &Host, when: u32);

    // #[wasm_bindgen(method)]
    // fn get_event(this: &Host) -> web_sys::Event;

    #[wasm_bindgen(method)]
    fn write(this: &JsHost, buf: &[u8]) -> usize;

    #[wasm_bindgen(method)]
    fn window_create(this: &JsHost, hwnd: u32);
    #[wasm_bindgen(method)]
    fn window_set_title(this: &JsHost, hwnd: u32, title: &str);
    #[wasm_bindgen(method)]
    fn window_set_size(this: &JsHost, hwnd: u32, w: u32, h: u32);
    #[wasm_bindgen(method)]
    fn window_show(this: &JsHost, hwnd: u32, pixels: web_sys::ImageBitmap);
}

pub struct Host {
    js_host: JsHost,
    files: Vec<(String, Box<[u8]>)>,
    messages: VecDeque<win32::Message>,
}

impl Host {
    pub fn new(js_host: JsHost, files: Vec<(String, Box<[u8]>)>) -> Self {
        Host {
            js_host,
            files,
            messages: Default::default(),
        }
    }

    pub fn open(&self, path: &str) -> Option<&[u8]> {
        Some(&*self.files.iter().find(|f| f.0 == path)?.1)
    }
}

impl win32::Host for Host {
    fn exit(&self, exit_code: u32) {
        self.js_host.exit(exit_code)
    }

    fn time(&self) -> u32 {
        let global = js_sys::global()
            .dyn_into::<web_sys::WorkerGlobalScope>()
            .unwrap();
        global.performance().unwrap().now() as u32
    }

    fn get_message(&self) -> Option<win32::Message> {
        None
        // self.messages.pop_front()
        // let event = Host::get_event(self);
        // if event.is_undefined() {
        //     return None;
        // }
        // message_from_event(event)
        //     .inspect_err(|err| log::error!("failed to convert message: {err}"))
        //     .ok()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        if let Some(t) = wait {
            // Enqueue a timer to wake up caller.
            todo!();
            //Host::ensure_timer(self, t);
        }
        false
    }

    fn open(&self, path: &str) -> Box<dyn win32::File> {
        let content = match Host::open(self, path) {
            // XXX copies
            Some(content) => content.to_vec().into_boxed_slice(),
            None => {
                log::error!("unknown file {path}, returning empty file");
                Box::new([])
            }
        };
        Box::new(File { content, ofs: 0 })
    }

    fn write(&self, buf: &[u8]) -> usize {
        self.js_host.write(buf);
        buf.len()
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        self.js_host.window_create(hwnd);
        Box::new(Window {
            js_host: self.js_host.clone().unchecked_into::<JsHost>(),
            hwnd,
        })
    }

    fn create_surface(
        &mut self,
        hwnd: u32,
        opts: &win32::SurfaceOptions,
    ) -> Box<dyn win32::Surface> {
        Box::new(WebSurface::new(
            self.js_host.clone().unchecked_into::<JsHost>(),
            hwnd,
            opts,
        ))
    }
}
