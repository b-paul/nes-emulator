use std::cell::RefCell;

use crate::ppu::Ppu;
use crate::MemoryDevice;
use crate::controller::NesController;

pub struct Mapper0 {
    memory: RefCell<Vec<u8>>,
    pub ppu: RefCell<Ppu>,
    pub controller: RefCell<NesController>,
    prg_rom: Vec<u8>,
    prg_rom_size: usize,
}

impl Mapper0 {
    pub fn new(
        prg_rom: Vec<u8>,
        prg_rom_size: usize,
        chr_rom: Vec<u8>,
        chr_rom_size: usize,
    ) -> Self {
        Mapper0 {
            memory: RefCell::new(vec![0; 0x800]),
            ppu: RefCell::new(Ppu::new(chr_rom, chr_rom_size)),
            controller: RefCell::new(NesController::new()),
            prg_rom,
            prg_rom_size,
        }
    }
}

impl MemoryDevice for Mapper0 {
    fn read_addr(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.memory.borrow_mut()[addr as usize % 0x0800],
            0x2000..=0x3FFF => {
                let addr = 0x2000 + (addr - 0x2000) % 8;
                self.ppu.borrow_mut().read_addr(addr)
            }
            0x4014 => {
                self.ppu.borrow_mut().read_addr(addr)
            }
            0x4016 | 0x4017 => {
                self.controller.borrow_mut().read_input()
            }
            0x4000..=0x4017 => {
                //panic!("APU and or I/O :(");
                0
            }
            0x4020..=0xFFFF => {
                let addr = (addr as usize - 0x8000) % self.prg_rom_size;
                self.prg_rom[addr]
            }
            _ => panic!("dont know how to read {addr:#06x}"),
        }
    }

    fn write_addr(&self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x17FF => {
                self.memory.borrow_mut()[addr as usize % 0x0800] = val;
            }
            0x2000..=0x3FFF => {
                let addr = 0x2000 + (addr - 0x2000) % 8;
                self.ppu.borrow_mut().write_addr(addr, val, self)
            }
            0x4014 => {
                self.ppu.borrow_mut().write_addr(addr, val, self)
            }
            0x4016 => {
                self.controller.borrow_mut().poll();
            }
            0x4000..=0x4017 => {
                //panic!("APU and or I/O :(");
            }
            0x4020..=0xFFFF => {
                //panic!("reading to rom?!?! or not?? {addr:#06x}");
                //println!("Wrote {val:#04x} to {addr:#06x}");
                print!("{}", val as char);
            }
            _ => panic!("dont know how to write to {addr:#06x}"),
        }
    }
}
