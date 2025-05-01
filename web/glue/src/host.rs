//! Implementations of the traits in win32/host.rs, providing the hosting API for the emulator.

use anyhow::bail;
use wasm_bindgen::prelude::*;
use win32::WindowsPath;

/// We create one <canvas> per DirectDraw Surface.  We draw to them by pushing pixels
/// via putImageData (not hw accelerated) and then use drawImage (likely hw accelerated)
/// to implement bit_blt between surfaces.
///
/// We also use drawImage to draw the "primary" surface to the canvas in the HTML DOM
/// when page flipping.  It's not clear whether this performs well vs just putting the
/// relevant canvas in place in the DOM, nor can I find good references that talk about
/// these relative options.  FWIW ChatGPT believes that drawImage is faster than DOM
/// manipulation.
///
/// https://developer.chrome.com/blog/taking-advantage-of-gpu-acceleration-in-the-2d-canvas
/// https://web.dev/articles/canvas-performance
pub struct WebSurface {
    _hwnd: u32,
    pub canvas: web_sys::HtmlCanvasElement,
    width: u32,
    ctx: web_sys::CanvasRenderingContext2d,
    screen: web_sys::CanvasRenderingContext2d,
}

impl WebSurface {
    pub fn new(
        hwnd: u32,
        opts: &win32::host::SurfaceOptions,
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

impl win32::host::Surface for WebSurface {
    fn write_pixels(&self, pixels: &[u8]) {
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(
            wasm_bindgen::Clamped(pixels),
            self.width,
        )
        .unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

    fn show(&self) {
        self.screen
            .draw_image_with_html_canvas_element(&self.canvas, 0.0, 0.0)
            .unwrap();
    }

    fn bit_blt(
        &self,
        dst_rect: &win32::RECT,
        src: &dyn win32::host::Surface,
        src_rect: &win32::RECT,
    ) {
        // Hack: we know all surfaces are WebSurface.
        // I think to fix this properly I might need to make every X86 generic across all the
        // host types, eek.
        let src = unsafe { &*(src as *const dyn win32::host::Surface as *const WebSurface) };
        self.ctx
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &src.canvas,
                src_rect.left as f64,
                src_rect.top as f64,
                (src_rect.right - src_rect.left) as f64,
                (src_rect.bottom - src_rect.top) as f64,
                dst_rect.left as f64,
                dst_rect.top as f64,
                (dst_rect.right - dst_rect.left) as f64,
                (dst_rect.bottom - dst_rect.top) as f64,
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
    #[wasm_bindgen(typescript_type = "JsWindow")]
    pub type JsWindow;
    #[wasm_bindgen(method, setter)]
    fn set_title(this: &JsWindow, title: &str);
    #[wasm_bindgen(method)]
    fn set_size(this: &JsWindow, width: u32, height: u32);
    #[wasm_bindgen(method)]
    fn fullscreen(this: &JsWindow);
}

impl win32::host::Window for JsWindow {
    fn set_title(&self, title: &str) {
        JsWindow::set_title(self, title);
    }

    fn set_size(&self, width: u32, height: u32) {
        JsWindow::set_size(self, width, height);
    }

    fn fullscreen(&self) {
        JsWindow::fullscreen(self);
    }
}

#[wasm_bindgen(typescript_custom_section)]
const JSFILE_TS: &'static str = r#"
export interface JsFile {
    info(): number;
    seek(from: number, ofs: number): number;
    read(buf: Uint8Array): number;
    write(buf: Uint8Array): number;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsFile")]
    pub type JsFile;
    #[wasm_bindgen(method)]
    fn info(this: &JsFile) -> u64;
    #[wasm_bindgen(method, catch)]
    fn seek(this: &JsFile, from: u32, ofs: i32) -> Result<u32, JsValue>;
    #[wasm_bindgen(method)]
    fn read(this: &JsFile, buf: &mut [u8]) -> u32;
    #[wasm_bindgen(method)]
    fn write(this: &JsFile, buf: &[u8]) -> u32;
}

impl win32::host::File for JsFile {
    fn stat(&self) -> Result<win32::host::Stat, win32::ERROR> {
        Ok(win32::host::Stat {
            kind: win32::host::StatKind::File,
            size: JsFile::info(self),
            atime: 0,
            ctime: 0,
            mtime: 0,
        })
    }

    fn set_len(&self, len: u64) -> Result<(), win32::ERROR> {
        todo!("set_len {len}")
    }
}

impl std::io::Read for JsFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = JsFile::read(self, buf);
        Ok(n as usize)
    }
}

impl std::io::Write for JsFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n = JsFile::write(self, buf);
        Ok(n as usize)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl std::io::Seek for JsFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let (from, ofs) = match pos {
            std::io::SeekFrom::Start(ofs) => (0, ofs as i32),
            std::io::SeekFrom::End(ofs) => (1, ofs as i32),
            std::io::SeekFrom::Current(ofs) => (2, ofs as i32),
        };
        let ofs = JsFile::seek(self, from, ofs)
            .map(|ofs| ofs as u64)
            .map_err(|err| {
                let err = err.dyn_into::<js_sys::Error>().unwrap();
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    err.to_string().as_string().unwrap(),
                )
            })?;
        Ok(ofs as u64)
    }
}

fn map_mousevent(event: web_sys::MouseEvent) -> anyhow::Result<win32::host::MouseMessage> {
    Ok(win32::host::MouseMessage {
        down: true,
        button: match event.button() {
            0 => win32::host::MouseButton::Left,
            1 => win32::host::MouseButton::Middle,
            2 => win32::host::MouseButton::Right,
            _ => bail!("unhandled button"),
        },
        x: event.offset_x() as u32,
        y: event.offset_y() as u32,
    })
}

fn message_from_event(event: web_sys::Event) -> anyhow::Result<win32::host::Message> {
    let hwnd = js_sys::Reflect::get(&event, &JsValue::from_str("hwnd"))
        .unwrap()
        .as_f64()
        .unwrap() as u32;
    let time = event.time_stamp() as u32;
    let detail = match event.type_().as_str() {
        "mousedown" => {
            let mut event = map_mousevent(event.unchecked_into::<web_sys::MouseEvent>())?;
            event.down = true;
            win32::host::MessageDetail::Mouse(event)
        }
        "mousemove" => {
            let mut event = map_mousevent(event.unchecked_into::<web_sys::MouseEvent>())?;
            event.down = false;
            event.button = win32::host::MouseButton::None;
            win32::host::MessageDetail::Mouse(event)
        }
        "mouseup" => {
            let mut event = map_mousevent(event.unchecked_into::<web_sys::MouseEvent>())?;
            event.down = false;
            win32::host::MessageDetail::Mouse(event)
        }
        ty => bail!("unhandled event type {ty}"),
    };
    log::info!("msg: {:?}", detail);
    Ok(win32::host::Message { hwnd, detail, time })
}

struct ReadDir {}
impl win32::host::ReadDir for ReadDir {
    fn next(&mut self) -> Result<Option<win32::host::ReadDirEntry>, win32::ERROR> {
        log::warn!("TODO: ReadDir");
        Ok(None)
    }
}

#[wasm_bindgen(typescript_custom_section)]
const JSHOST_TS: &'static str = r#"
export interface JsHost {
  log(level: number, msg: string): void;
  win32_trace(contest: string, msg: string): void;

  ensure_timer(when: number): void;
  get_event(): Event | undefined;
  
  open(path: string, access: FileOptions): JsFile|null;
  stdout(buf: Uint8Array): void;
  
  create_window(hwnd: number): JsWindow;
  screen(): CanvasRenderingContext2D;
  audio(buf: Int16Array): void;
}"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "JsHost")]
    pub type JsHost;

    #[wasm_bindgen(method)]
    pub fn log(this: &JsHost, level: u8, msg: String);

    #[wasm_bindgen(method)]
    pub fn win32_trace(this: &JsHost, context: &str, msg: &str);

    #[wasm_bindgen(method)]
    fn ensure_timer(this: &JsHost, when: u32);

    #[wasm_bindgen(method)]
    fn get_event(this: &JsHost) -> web_sys::Event;

    #[wasm_bindgen(method)]
    fn open(this: &JsHost, path: &str, options: win32::host::FileOptions) -> Option<JsFile>;
    #[wasm_bindgen(method)]
    fn stdout(this: &JsHost, buf: &[u8]);

    #[wasm_bindgen(method)]
    fn create_window(this: &JsHost, hwnd: u32) -> JsWindow;

    #[wasm_bindgen(method)]
    fn screen(this: &JsHost) -> web_sys::CanvasRenderingContext2d;

    #[wasm_bindgen(method)]
    fn audio(this: &JsHost, buf: &[i16]);
}

impl win32::host::FileSystem for JsHost {
    fn open(
        &self,
        path: &WindowsPath,
        options: win32::host::FileOptions,
    ) -> Result<Box<dyn win32::host::File>, win32::ERROR> {
        match JsHost::open(self, &path.to_string_lossy(), options) {
            Some(file) => Ok(Box::new(file)),
            None => Err(win32::ERROR::FILE_NOT_FOUND),
        }
    }

    fn stat(&self, path: &WindowsPath) -> Result<win32::host::Stat, win32::ERROR> {
        todo!("stat {path}")
    }

    fn read_dir(&self, _path: &WindowsPath) -> Result<Box<dyn win32::host::ReadDir>, win32::ERROR> {
        Ok(Box::new(ReadDir {}))
    }

    fn current_dir(&self) -> Result<win32::WindowsPathBuf, win32::ERROR> {
        Ok(win32::WindowsPathBuf::from("Z:\\".to_string()))
    }

    fn create_dir(&self, path: &WindowsPath) -> Result<(), win32::ERROR> {
        todo!("create_dir {path}")
    }

    fn remove_file(&self, path: &WindowsPath) -> Result<(), win32::ERROR> {
        todo!("remove_file {path}")
    }

    fn remove_dir(&self, path: &WindowsPath) -> Result<(), win32::ERROR> {
        todo!("remove_dir {path}")
    }
}

impl win32::host::Host for JsHost {
    fn ticks(&self) -> u32 {
        web_sys::window().unwrap().performance().unwrap().now() as u32
    }

    fn system_time(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    fn get_message(&self) -> Option<win32::host::Message> {
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

    fn stdout(&self, buf: &[u8]) {
        JsHost::stdout(self, buf)
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::host::Window> {
        let window = JsHost::create_window(self, hwnd);
        Box::new(window)
    }

    fn create_surface(
        &mut self,
        hwnd: u32,
        opts: &win32::host::SurfaceOptions,
    ) -> Box<dyn win32::host::Surface> {
        Box::new(WebSurface::new(hwnd, opts, JsHost::screen(self)))
    }

    fn init_audio(
        &mut self,
        _sample_rate: u32,
        _callback: win32::host::AudioCallback,
    ) -> Box<dyn win32::host::Audio> {
        let host: JsHost = self.clone().into();
        Box::new(host)
    }
}

impl win32::host::Audio for JsHost {
    fn write(&mut self, _buf: &[u8]) {
        // ignore
    }

    fn pos(&mut self) -> usize {
        0
    }
}
