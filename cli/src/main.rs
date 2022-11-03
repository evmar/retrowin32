extern crate sdl2;
extern crate win32;

mod logging;
use std::{
    cell::{Cell, RefCell},
    io::Write,
};

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

struct Host {
    video: sdl2::VideoSubsystem,
    pump: RefCell<sdl2::EventPump>,
    exit_code: Cell<Option<u32>>,
}
impl Host {
    fn new() -> anyhow::Result<Self> {
        let sdl = sdl2::init().map_err(|err| anyhow::anyhow!(err))?;
        let video = sdl.video().map_err(|err| anyhow::anyhow!(err))?;
        let pump = sdl.event_pump().map_err(|err| anyhow::anyhow!(err))?;

        Ok(Host {
            video,
            pump: RefCell::new(pump),
            exit_code: std::cell::Cell::new(None),
        })
    }
    fn pump_messages(&self) -> bool {
        for event in self.pump.borrow_mut().poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return false,
                _ => {}
            }
            println!("ev {:?}", event);
        }
        true
    }
}
impl win32::Host for Host {
    fn exit(&self, code: u32) {
        self.exit_code.set(Some(code));
    }

    fn write(&self, buf: &[u8]) -> usize {
        std::io::stdout().lock().write(buf).unwrap()
    }

    fn time(&self) -> u32 {
        todo!()
    }

    fn create_window(&self) -> Box<dyn win32::Window> {
        Box::new(Window::new(&self.video))
    }

    fn create_surface(&self, opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        log::warn!("create_surface {opts:?}");
        Box::new(Surface::new())
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
impl win32::Window for Window {
    fn set_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(title).unwrap()
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.canvas.window_mut().set_size(width, height).unwrap()
    }
}

struct Surface {}
impl Surface {
    fn new() -> Self {
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
    let host = Host::new()?;
    let mut runner = win32::Runner::new(&host);
    runner.load_exe(&buf)?;

    while host.exit_code.get().is_none() {
        if !host.pump_messages() {
            break;
        }
        match runner.step_many(10000) {
            Err(err) => {
                dump_asm(&runner);
                log::error!("{:?}", err);
                break;
            }
            Ok(false) => {
                if host.exit_code.get().is_some() {
                    break;
                }
            }
            Ok(true) => {}
        }
    }

    Ok(())
}
