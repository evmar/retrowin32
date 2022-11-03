extern crate sdl2;
extern crate win32;

mod logging;
use std::{cell::RefCell, io::Write, rc::Rc};

use anyhow::bail;

fn dump_asm(runner: &win32::Runner) {
    let instrs = win32::disassemble(&runner.x86.mem, runner.x86.regs.eip);

    for instr in instrs {
        print!("{:08X} ", instr.addr);
        for b in instr.bytes.bytes() {
            print!("{:02x}", b);
        }
        if instr.bytes.len() < 10 {
            for _ in 0..10 - instr.bytes.len() {
                print!("  ");
            }
        }
        for part in instr.code {
            print!("{}", part.text);
        }
        println!();
    }
}

struct Env {
    video: sdl2::VideoSubsystem,
    pump: sdl2::EventPump,
    exit_code: Option<u32>,
    win: Option<WindowRef>,
}
impl Env {
    fn new() -> anyhow::Result<Self> {
        let sdl = sdl2::init().map_err(|err| anyhow::anyhow!(err))?;
        let video = sdl.video().map_err(|err| anyhow::anyhow!(err))?;
        let pump = sdl.event_pump().map_err(|err| anyhow::anyhow!(err))?;

        Ok(Env {
            video,
            pump: pump,
            exit_code: None,
            win: None,
        })
    }
    fn pump_messages(&mut self) -> bool {
        for event in self.pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,
                _ => {}
            }
            println!("ev {:?}", event);
        }
        true
    }
}

#[derive(Clone)]
struct EnvRef(Rc<RefCell<Env>>);

impl win32::Host for EnvRef {
    fn exit(&mut self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }

    fn time(&self) -> u32 {
        todo!()
    }

    fn create_window(&mut self) -> Box<dyn win32::Window> {
        let mut env = self.0.borrow_mut();
        let win = Window::new(&env.video);
        let win_ref = WindowRef(Rc::new(RefCell::new(win)));
        env.win = Some(win_ref.clone());
        Box::new(win_ref)
    }

    fn create_surface(&mut self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        Box::new(Surface::new(opts))
    }
}

struct Window {
    canvas: sdl2::render::WindowCanvas,
}
impl Window {
    fn new(video: &sdl2::VideoSubsystem) -> Self {
        let win = video.window("retrowin32", 640, 480).build().unwrap();
        let canvas = win.into_canvas().build().unwrap();
        Window { canvas }
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

struct Surface {}
impl Surface {
    fn new(_opts: &win32::SurfaceOptions) -> Self {
        Surface {}
    }
}
impl win32::Surface for Surface {
    fn write_pixels(&self, _pixels: &[[u8; 4]]) {
        todo!()
    }

    fn get_attached(&self) -> Box<dyn win32::Surface> {
        todo!()
    }

    fn flip(&self) {
        todo!()
    }

    fn bit_blt(
        &self,
        _dx: u32,
        _xy: u32,
        _other: &dyn win32::Surface,
        _sx: u32,
        _sy: u32,
        _w: u32,
        _h: u32,
    ) {
        todo!()
    }
}

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let args: Vec<String> = std::env::args().collect();
    let exe = match args.as_slice() {
        [_, exe] => exe,
        _ => bail!("specify path"),
    };

    let buf = std::fs::read(exe)?;
    let host = EnvRef(Rc::new(RefCell::new(Env::new()?)));
    let mut runner = win32::Runner::new(Box::new(host.clone()));
    runner.load_exe(&buf)?;

    loop {
        if !host.0.borrow_mut().pump_messages() {
            break;
        }
        match runner.step_many(10000) {
            Err(err) => {
                dump_asm(&runner);
                log::error!("{:?}", err);
                break;
            }
            Ok(false) => {
                if host.0.borrow().exit_code.is_some() {
                    break;
                }
            }
            Ok(true) => {}
        }
    }

    Ok(())
}
