// XXX DELETE THIS WHEN DONE!! XXX
#![allow(dead_code)]
#![feature(bigint_helper_methods)]

use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};

mod controller;
mod cpu;
mod opcode;
mod parser;
mod ppu;

mod mapper0;

use cpu::{Cpu, MemoryDevice};

use std::cell::RefCell;
#[derive(Debug, Default, PartialEq, Eq)]
struct Memory {
    mem: RefCell<Vec<u8>>,
}

impl MemoryDevice for Memory {
    fn read_addr(&self, addr: u16) -> u8 {
        let val = self.mem.borrow()[addr as usize];
        //println!("read  {addr:#06x} = {val:#04x}");
        val
    }

    fn write_addr(&self, addr: u16, val: u8) {
        //println!("wrote {addr:#06x} = {val:#04x}");
        self.mem.borrow_mut()[addr as usize] = val;
    }
}

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let (_input, rom) = parser::parse_rom(&buf).unwrap();

    println!("Map number {}", rom.header.map_number);

    let mapper = mapper0::Mapper0::new(
        rom.prg_rom,
        rom.header.prg_rom_size,
        rom.chr_rom,
        rom.header.chr_rom_size,
    );
    let mut cpu = Cpu::new(&mapper);

    // copied from the docs !
    let sdl_context = sdl2::init().unwrap(); // whaaaa it's error is a string???
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("nes emulator", 256, 240)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        let key = match keycode {
                            // a press
                            Keycode::Z => 0,
                            // b press
                            Keycode::X => 1,
                            // select press
                            Keycode::S => 2,
                            // star press
                            Keycode::Return => 3,
                            // up press
                            Keycode::Up => 4,
                            // down press
                            Keycode::Down => 5,
                            // left press
                            Keycode::Left => 6,
                            // right press
                            Keycode::Right => 7,
                            _ => continue,
                        };
                        mapper.controller.borrow_mut().clear_input(key);
                    }
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        let key = match keycode {
                            // a press
                            Keycode::Z => 0,
                            // b press
                            Keycode::X => 1,
                            // select press
                            Keycode::S => 2,
                            // star press
                            Keycode::Return => 3,
                            // up press
                            Keycode::Up => 4,
                            // down press
                            Keycode::Down => 5,
                            // left press
                            Keycode::Left => 6,
                            // right press
                            Keycode::Right => 7,
                            _ => continue,
                        };
                        mapper.controller.borrow_mut().set_input(key);
                    }
                }
                _ => {}
            }
        }

        let start_time = Instant::now();

        for _ in 0..(262 * 341) {
            let _cycles = cpu.run_instruction();
            // this for loop is here because the ppu should be running 3x the speed of the cpu and
            // i used to have the loop make it do that, but for some reason mario worked the best
            // with it like this
            // something is very wrong with this program's code!
            // anyways let's just ignore this :))))))
            for _ in 0..1 {
                mapper
                    .ppu
                    .borrow_mut()
                    .cycle(&mut texture, &mut cpu)
                    .unwrap();
            }
        }

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();

        const FRAME: Duration = Duration::from_nanos(1000000000 / 60);

        if let Some(left) = FRAME.checked_sub(start_time.elapsed()) {
            std::thread::sleep(left);
        }
    }

    Ok(())

    /*
    let mut mem = vec![0; 0x10000];

    for i in 0..0x8000 {
        mem[0x8000 + i] = rom.prg_rom[i as usize];
    }

    let memory = Memory {
        mem: RefCell::new(mem),
    };

    let mut cpu = Cpu::new(&memory);

    println!("{:#x} {:#x}", rom.prg_rom[0x7FFC], rom.prg_rom[0x7FFD]);

    memory.write_addr(0x2002, 0x80);

    let mut counter = 0;

    let time = std::time::Instant::now();

    loop {
        //println!("{:#06x} {} {} {} {}", cpu.pc, cpu.a, cpu.x, cpu.y, cpu.sp);
        /*
        for i in (cpu.sp as u16 +1)..=255 {
            print!("{:#4x} ", memory.read_addr(0x0100 + i));
        }
        println!();
        */
        cpu.run_instruction();
        counter += 1;
        if counter % 30000 == 2890 {
            cpu.nmi_interrupt();
        }
        //std::thread::sleep(std::time::Duration::from_millis(100));
        if counter % 1000000 == 0 {
            println!("{:09} cycles per second", counter as f64 / time.elapsed().as_secs_f64());
        }
    }

    //Ok(())
    */
}
