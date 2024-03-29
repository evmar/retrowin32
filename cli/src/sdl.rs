use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "sdl")]
extern crate sdl2;

fn map_button(b: sdl2::mouse::MouseButton) -> Option<win32::MouseButton> {
    Some(match b {
        sdl2::mouse::MouseButton::Left => win32::MouseButton::Left,
        sdl2::mouse::MouseButton::Right => win32::MouseButton::Right,
        sdl2::mouse::MouseButton::Middle => win32::MouseButton::Middle,
        _ => return None,
    })
}

fn message_from_event(hwnd: u32, event: sdl2::event::Event) -> Option<win32::Message> {
    let detail = match event {
        sdl2::event::Event::Quit { .. } => win32::MessageDetail::Quit,
        sdl2::event::Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => win32::MessageDetail::Mouse(win32::MouseMessage {
            down: true,
            button: map_button(mouse_btn)?,
            x: x as u32,
            y: y as u32,
        }),
        sdl2::event::Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => win32::MessageDetail::Mouse(win32::MouseMessage {
            down: false,
            button: map_button(mouse_btn)?,
            x: x as u32,
            y: y as u32,
        }),
        _ => {
            // log::warn!("unhandled event: {:?}", event);
            return None;
        }
    };
    Some(win32::Message { hwnd, detail })
}

pub struct GUI {
    video: sdl2::VideoSubsystem,
    pump: sdl2::EventPump,
    win: Option<WindowRef>,
}
impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        assert!(sdl2::hint::set("SDL_NO_SIGNAL_HANDLERS", "1"));
        let sdl = sdl2::init().map_err(|err| anyhow::anyhow!(err))?;
        let video = sdl.video().map_err(|err| anyhow::anyhow!(err))?;
        let pump = sdl.event_pump().map_err(|err| anyhow::anyhow!(err))?;

        Ok(GUI {
            video,
            pump: pump,
            win: None,
        })
    }

    pub fn get_message(&mut self, wait: bool) -> Option<win32::Message> {
        let hwnd = match &self.win {
            Some(w) => w.0.borrow().hwnd,
            None => 0,
        };
        if wait {
            loop {
                let msg = message_from_event(hwnd, self.pump.wait_event());
                if msg.is_some() {
                    return msg;
                }
            }
        } else {
            loop {
                let msg = self.pump.poll_event()?;
                let msg = message_from_event(hwnd, msg);
                if msg.is_some() {
                    return msg;
                }
            }
        }
    }

    pub fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        let win = Window::new(&self.video, hwnd);
        let win_ref = WindowRef(Rc::new(RefCell::new(win)));
        self.win = Some(win_ref.clone());
        Box::new(win_ref)
    }

    pub fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        Box::new(Texture::new(self.win.as_ref().unwrap(), opts))
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
impl win32::Window for WindowRef {
    fn set_title(&mut self, title: &str) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_title(title)
            .unwrap();
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.0
            .borrow_mut()
            .canvas
            .window_mut()
            .set_size(width, height)
            .unwrap();
    }
}

struct Texture {
    window: WindowRef,
    texture: sdl2::render::Texture,
    width: u32,
    height: u32,
}

impl Texture {
    fn new(win: &WindowRef, opts: &win32::SurfaceOptions) -> Self {
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
            texture,
            width: opts.width,
            height: opts.height,
        }
    }
}

impl win32::Surface for Texture {
    fn write_pixels(&mut self, pixels: &[[u8; 4]]) {
        let pixels_u8 =
            unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const u8, pixels.len() * 4) };
        let rect = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        self.texture
            .update(rect, pixels_u8, self.width as usize * 4)
            .unwrap();
    }

    fn show(&mut self) {
        let rect = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        let canvas = &mut self.window.0.borrow_mut().canvas;
        canvas.copy(&self.texture, rect, rect).unwrap();
        canvas.present();
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
        let src_rect = sdl2::rect::Rect::new(sx as i32, sy as i32, w, h);
        let src = &unsafe { &*(src as *const dyn win32::Surface as *const Texture) }.texture;
        let dst_rect = sdl2::rect::Rect::new(dx as i32, dy as i32, w, h);

        self.window
            .0
            .borrow_mut()
            .canvas
            .with_texture_canvas(&mut self.texture, |canvas| {
                canvas.copy(src, src_rect, dst_rect).unwrap()
            })
            .unwrap();
    }
}
