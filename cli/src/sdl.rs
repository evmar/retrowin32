use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "sdl")]
extern crate sdl2;

fn map_button(b: sdl2::mouse::MouseButton) -> Option<win32::host::MouseButton> {
    Some(match b {
        sdl2::mouse::MouseButton::Left => win32::host::MouseButton::Left,
        sdl2::mouse::MouseButton::Right => win32::host::MouseButton::Right,
        sdl2::mouse::MouseButton::Middle => win32::host::MouseButton::Middle,
        _ => return None,
    })
}

fn message_from_event(hwnd: u32, event: sdl2::event::Event) -> Option<win32::host::Message> {
    let (time, detail) = match event {
        sdl2::event::Event::Quit { timestamp } => (timestamp, win32::host::MessageDetail::Quit),
        sdl2::event::Event::MouseButtonDown {
            timestamp,
            mouse_btn,
            x,
            y,
            ..
        } => (
            timestamp,
            win32::host::MessageDetail::Mouse(win32::host::MouseMessage {
                down: true,
                button: map_button(mouse_btn)?,
                x: x as u32,
                y: y as u32,
            }),
        ),
        sdl2::event::Event::MouseButtonUp {
            timestamp,
            mouse_btn,
            x,
            y,
            ..
        } => (
            timestamp,
            win32::host::MessageDetail::Mouse(win32::host::MouseMessage {
                down: false,
                button: map_button(mouse_btn)?,
                x: x as u32,
                y: y as u32,
            }),
        ),
        sdl2::event::Event::MouseMotion {
            timestamp, x, y, ..
        } => (
            timestamp,
            win32::host::MessageDetail::Mouse(win32::host::MouseMessage {
                down: false,
                button: win32::host::MouseButton::None,
                x: x as u32,
                y: y as u32,
            }),
        ),
        _ => {
            // log::warn!("unhandled event: {:?}", event);
            return None;
        }
    };
    Some(win32::host::Message { hwnd, detail, time })
}

fn message_from_events(
    hwnd: u32,
    mut f: impl FnMut() -> Option<sdl2::event::Event>,
) -> Option<win32::host::Message> {
    loop {
        let event = f()?;
        let msg = message_from_event(hwnd, event);
        if msg.is_some() {
            return msg;
        }
    }
}

pub struct GUI {
    sdl: sdl2::Sdl,
    video: sdl2::VideoSubsystem,
    pump: sdl2::EventPump,
    timer: sdl2::TimerSubsystem,
    win: Option<WindowRef>,
    msg_queue: Option<win32::host::Message>,
}

impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        assert!(sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1"));
        let sdl = sdl2::init().map_err(|err| anyhow::anyhow!(err))?;
        let video = sdl.video().map_err(|err| anyhow::anyhow!(err))?;
        let pump = sdl.event_pump().map_err(|err| anyhow::anyhow!(err))?;
        let timer = sdl.timer().map_err(|err| anyhow::anyhow!(err))?;

        Ok(GUI {
            sdl,
            video,
            pump,
            timer,
            win: None,
            msg_queue: None,
        })
    }

    pub fn time(&self) -> u32 {
        self.timer.ticks()
    }

    pub fn get_message(&mut self) -> Option<win32::host::Message> {
        if let Some(msg) = self.msg_queue.take() {
            return Some(msg);
        }
        let hwnd = match &self.win {
            Some(w) => w.0.borrow().hwnd,
            None => 0,
        };
        message_from_events(hwnd, || self.pump.poll_event())
    }

    pub fn block(&mut self, wait: Option<u32>) -> bool {
        let hwnd = match &self.win {
            Some(w) => w.0.borrow().hwnd,
            None => 0,
        };
        let msg = match wait {
            Some(until) => message_from_events(hwnd, || {
                let now = self.time();
                if now >= until {
                    return None;
                }
                let delta = until - now;
                self.pump.wait_event_timeout(delta)
            }),
            None => loop {
                let msg = message_from_event(hwnd, self.pump.wait_event());
                if msg.is_some() {
                    break msg;
                }
            },
        };
        assert!(self.msg_queue.is_none());
        self.msg_queue = msg;
        true
    }

    pub fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::host::Window> {
        let win = Window::new(&self.video, hwnd);
        let win_ref = WindowRef(Rc::new(RefCell::new(win)));
        if self.win.is_some() {
            log::warn!("TODO: handle multiple windows");
        } else {
            self.win = Some(win_ref.clone());
        }
        Box::new(win_ref)
    }

    pub fn create_surface(
        &mut self,
        opts: &win32::host::SurfaceOptions,
    ) -> Box<dyn win32::host::Surface> {
        Box::new(Texture::new(self.win.as_ref().unwrap(), opts))
    }

    pub fn init_audio(
        &mut self,
        sample_rate: u32,
        callback: win32::host::AudioCallback,
    ) -> Box<dyn win32::host::Audio> {
        Box::new(Audio::new(&self.sdl, sample_rate, callback))
    }
}

struct Window {
    hwnd: u32,
    canvas: sdl2::render::WindowCanvas,
}
impl Window {
    fn new(video: &sdl2::VideoSubsystem, hwnd: u32) -> Self {
        let win = video.window("retrowin32", 640, 480).build().unwrap();
        let canvas = win.into_canvas().build().unwrap();
        Window { hwnd, canvas }
    }
}

#[derive(Clone)]
struct WindowRef(Rc<RefCell<Window>>);
impl win32::host::Window for WindowRef {
    fn set_title(&self, title: &str) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_title(title)
            .unwrap();
    }

    fn set_size(&self, width: u32, height: u32) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_size(width, height)
            .unwrap();
    }

    fn fullscreen(&self) {
        log::info!("fullscreen request ignored for debugging ease");
        // self.0
        //     .borrow_mut()
        //     .canvas
        //     .window_mut()
        //     .set_fullscreen(sdl2::video::FullscreenType::Desktop)
        //     .unwrap();
    }
}

struct Texture {
    window: WindowRef,
    texture: RefCell<sdl2::render::Texture>,
    width: u32,
    height: u32,
}

impl Texture {
    fn new(win: &WindowRef, opts: &win32::host::SurfaceOptions) -> Self {
        let texture = win
            .0
            .borrow()
            .canvas
            .texture_creator()
            .create_texture_target(
                sdl2::pixels::PixelFormatEnum::ABGR8888,
                opts.width,
                opts.height,
            )
            .unwrap();
        Texture {
            window: win.clone(),
            texture: RefCell::new(texture),
            width: opts.width,
            height: opts.height,
        }
    }
}

fn sdl_rect_from_win32(rect: &win32::RECT) -> sdl2::rect::Rect {
    sdl2::rect::Rect::new(
        rect.left,
        rect.top,
        (rect.right - rect.left) as u32,
        (rect.bottom - rect.top) as u32,
    )
}

impl win32::host::Surface for Texture {
    fn write_pixels(&self, pixels: &[u8]) {
        let rect = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        self.texture
            .borrow_mut()
            .update(rect, pixels, self.width as usize * 4)
            .unwrap();
    }

    fn show(&self) {
        let canvas = &mut self.window.0.borrow_mut().canvas;
        // Passing None/None for the src/dst rects means to do a scaling full copy,
        // which is what we want for the fullscreen case in particular.
        canvas.copy(&self.texture.borrow_mut(), None, None).unwrap();
        canvas.present();
    }

    fn bit_blt(
        &self,
        dst_rect: &win32::RECT,
        src: &dyn win32::host::Surface,
        src_rect: &win32::RECT,
    ) {
        let src_rect = sdl_rect_from_win32(src_rect);
        let src = &unsafe { &*(src as *const dyn win32::host::Surface as *const Texture) }
            .texture
            .borrow();
        let dst_rect = sdl_rect_from_win32(dst_rect);

        self.window
            .0
            .borrow_mut()
            .canvas
            .with_texture_canvas(&mut self.texture.borrow_mut(), |canvas| {
                canvas.copy(src, src_rect, dst_rect).unwrap()
            })
            .unwrap();
    }
}

/// Copy bytes from a [u8] to a [T].  The slices must be of appropriate size.
/// This is like copy_from_slice, but allows for unaligned ([u8]) input.
fn copy_from_bytes<T>(out: &mut [T], bytes: &[u8]) {
    assert_eq!(out.len() * std::mem::size_of::<T>(), bytes.len());
    let out_bytes =
        unsafe { std::slice::from_raw_parts_mut(out.as_mut_ptr() as *mut u8, bytes.len()) };
    out_bytes.copy_from_slice(bytes);
}

struct Chunk {
    buf: Box<[i16]>,
    ofs: usize,
}

struct AudioBuffer {
    next: Option<Chunk>,
    /// The count of i16s that have been read in total,
    /// for use in querying current audio position.
    pos: usize,
    callback: win32::host::AudioCallback,
}

impl AudioBuffer {
    fn new(callback: win32::host::AudioCallback) -> Self {
        AudioBuffer {
            next: None,
            pos: 0,
            callback,
        }
    }
}

impl AudioBuffer {
    fn write(&mut self, buf: &[u8]) {
        assert!(self.next.is_none());
        assert!(buf.len() % 2 == 0); // must contain i16s
        let mut next = Box::<[i16]>::new_uninit_slice(buf.len() / 2);
        copy_from_bytes(&mut next, buf);
        log::debug!("host audio write: {} samples", next.len());
        self.next = Some(Chunk {
            buf: unsafe { std::mem::transmute(next) },
            ofs: 0,
        });
    }
}

impl sdl2::audio::AudioCallback for AudioBuffer {
    type Channel = i16;
    fn callback(&mut self, buf: &mut [i16]) {
        let mut buf = buf;
        if let Some(next) = self.next.as_mut() {
            let avail = &next.buf[next.ofs..];
            let n = std::cmp::min(avail.len(), buf.len());
            buf[..n].copy_from_slice(&avail[..n]);
            buf = &mut buf[n..];

            self.pos += n;
            next.ofs += n;
            if next.ofs >= next.buf.len() {
                self.next = None;
                (self.callback)();
            }
        }

        // Fill any remaining space with silence.
        if !buf.is_empty() {
            buf.fill(0);
        }
    }
}

struct Audio {
    dev: sdl2::audio::AudioDevice<AudioBuffer>,
}

impl Audio {
    fn new(sdl: &sdl2::Sdl, sample_rate: u32, callback: win32::host::AudioCallback) -> Self {
        let audio = sdl.audio().unwrap();
        let spec = sdl2::audio::AudioSpecDesired {
            freq: Some(sample_rate as i32),
            channels: Some(1),
            samples: None,
        };

        let dev = audio
            .open_playback(None, &spec, |_spec| AudioBuffer::new(callback))
            .unwrap();
        dev.resume();
        Audio { dev }
    }
}

impl win32::host::Audio for Audio {
    fn write(&mut self, buf: &[u8]) {
        self.dev.lock().write(buf);
    }

    fn pos(&mut self) -> usize {
        self.dev.lock().pos
    }
}
