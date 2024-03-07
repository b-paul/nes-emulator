use crate::{Cpu, MemoryDevice};
use sdl2::render::Texture;

const PALETTE: [[u8; 3]; 64] = [
    [0x55, 0x55, 0x55],
    [0x00, 0x17, 0x73],
    [0x00, 0x07, 0x86],
    [0x2e, 0x05, 0x78],
    [0x59, 0x02, 0x4d],
    [0x72, 0x00, 0x11],
    [0x6e, 0x00, 0x00],
    [0x4c, 0x08, 0x00],
    [0x17, 0x1b, 0x00],
    [0x00, 0x2a, 0x00],
    [0x00, 0x31, 0x00],
    [0x00, 0x2e, 0x08],
    [0x00, 0x26, 0x45],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xa5, 0xa5, 0xa5],
    [0x00, 0x57, 0xc6],
    [0x22, 0x3f, 0xe5],
    [0x6e, 0x28, 0xd9],
    [0xae, 0x1a, 0xa6],
    [0xd2, 0x17, 0x59],
    [0xd1, 0x21, 0x07],
    [0xa7, 0x37, 0x00],
    [0x63, 0x51, 0x00],
    [0x18, 0x67, 0x00],
    [0x00, 0x72, 0x00],
    [0x00, 0x73, 0x31],
    [0x00, 0x6a, 0x84],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xfe, 0xff, 0xff],
    [0x2f, 0xa8, 0xff],
    [0x5d, 0x81, 0xff],
    [0x9c, 0x70, 0xff],
    [0xf7, 0x72, 0xff],
    [0xff, 0x77, 0xbd],
    [0xff, 0x7e, 0x75],
    [0xff, 0x8a, 0x2b],
    [0xcd, 0xa0, 0x00],
    [0x81, 0xb8, 0x02],
    [0x3d, 0xc8, 0x30],
    [0x12, 0xcd, 0x7b],
    [0x0d, 0xc5, 0xd0],
    [0x3c, 0x3c, 0x3c],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
    [0xfe, 0xff, 0xff],
    [0xa4, 0xde, 0xff],
    [0xb1, 0xc8, 0xff],
    [0xcc, 0xbe, 0xff],
    [0xf4, 0xc2, 0xff],
    [0xff, 0xc5, 0xea],
    [0xff, 0xc7, 0xc9],
    [0xff, 0xcd, 0xaa],
    [0xef, 0xd6, 0x96],
    [0xd0, 0xe0, 0x95],
    [0xb3, 0xe7, 0xa5],
    [0x9f, 0xea, 0xc3],
    [0x9a, 0xe8, 0xe6],
    [0xaf, 0xaf, 0xaf],
    [0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00],
];

// !!!!!!!!!!!!!!!!
// > Writes to the following registers are ignored if earlier than ~29658 CPU clocks after reset: PPUCTRL, PPUMASK, PPUSCROLL, PPUADDR. This also means that the PPUSCROLL/PPUADDR latch will not toggle. The other registers work immediately: PPUSTATUS, OAMADDR, OAMDATA ($2004), PPUDATA, and OAMDMA ($4014).

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Ppu {
    cycle: u32,

    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,

    // THIS IS missing 3 of the internal registers?!?! is this implemented correctly???
    // TODO mirror 0x3000..=0x3EFF
    w: bool,
    scroll_x: u8,
    scroll_y: u8,
    cur_scroll_x: u8,
    cur_scroll_y: u8,

    // should be fine right ?!?!
    oam: Vec<u8>,
    vram: Vec<u8>,

    read_buffer: u8,

    // scrolling internal registers
    v: u16,
    t: u16,
    x: u8,

    // Sprite evaluation
    secondary_oam: Vec<u8>,
    cur_sprite: u8,
    cur_oam_rel_idx: u8,

    // Rendering registers
    b_pat_reg: [u16; 2],
    b_pal_reg: [u8; 2],
    b_pal_latch: [bool; 2],

    scanline_oam: Vec<u8>,
    s_pat_reg: [[u8; 2]; 8],
    s_attrs: [u8; 8],
    s_counters: [u8; 8],

    l_nametable: u8,
    l_attr: u8,
    l_pt_low: u8,
    l_pt_high: u8,

    pixel_data: Vec<u8>,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, chr_rom_size: usize) -> Self {
        let mut vram = vec![0; 0x4000];

        for i in 0..chr_rom_size {
            vram[i] = chr_rom[i]; // lol yikes
        }

        Ppu {
            oam: vec![0; 256],
            secondary_oam: vec![0; 32],
            vram,
            pixel_data: vec![0; 256 * 240 * 4],
            ..Default::default()
        }
    }

    // i hope it's ok to be mut
    pub fn write_addr<M: MemoryDevice>(&mut self, addr: u16, b: u8, memory: &M) {
        //println!("{:#x}", addr);
        match addr {
            0x2000 => {
                // PPUCTRL
                self.ppu_ctrl = b;
                self.t &= !(0b11 << 10);
                self.t |= (self.ppu_ctrl as u16 & 0b11) << 10;
            }
            0x2001 => {
                // PPUMASK
                self.ppu_mask = b;
            }
            0x2003 => {
                // OAMADDR
                self.oam_addr = b;
            }
            0x2004 => {
                // OAMDATA
                self.oam[self.oam_addr as usize] = b;
                self.oam_addr = self.oam_addr.wrapping_add(1);
            }
            0x2005 => {
                // PPUSCROLL
                match self.w {
                    false => {
                        self.scroll_x = b;
                        self.t &= !0b11111;
                        self.t |= (self.scroll_x as u16 >> 3) & 0b11111;
                        self.x = self.scroll_x & 0b111;
                        self.w = true;
                    }
                    true => {
                        self.scroll_y = b;
                        self.t &= !(0b11111 << 5);
                        self.t |= ((self.scroll_y as u16 >> 3) & 0b11111) << 5;
                        self.t &= !(0b111 << 12);
                        self.t |= (self.scroll_y as u16 & 0b111) << 12;
                        self.w = false;
                    }
                }
            }
            0x2006 => {
                // PPUADDR (write twice)
                match self.w {
                    false => {
                        self.t &= !(0x3F << 8);
                        self.t |= (b as u16 & 0x3F) << 8;
                        self.w = true;
                    }
                    true => {
                        self.t &= !0xFF;
                        self.t |= b as u16 & 0xFF;
                        self.v = self.t;
                        self.w = false;
                    }
                }
            }
            0x2007 => {
                // PPUDATA
                let mut addr = self.v % 0x4000;
                if (0..0x2000).contains(&addr) {
                    // rom (how do i generalize this to allow modifying chr-ram?!)
                    return;
                }
                if (0x3F20..0x4000).contains(&addr) {
                    addr -= addr & 0x00E0;
                }
                if [0x3F10, 0x3F14, 0x3F18, 0x3F1C].contains(&addr) {
                    addr -= 0x10;
                }
                self.vram[addr as usize] = b;
                self.v += match self.ppu_ctrl & (1 << 2) != 0 {
                    false => 1,
                    true => 32,
                };
                self.v %= 0x8000;
            }
            0x4014 => {
                // OAMDMA
                let off = (b as u16) << 8;
                for i in 0..=255 {
                    let addr = off + i;
                    self.oam[i as usize] = memory.read_addr(addr);
                }
            }
            _ => panic!("Invalid PPU address {addr:#06x}"),
        }
    }

    pub fn read_addr(&mut self, addr: u16) -> u8 {
        match addr {
            0x2002 => {
                // PPUSTATUS
                self.w = false;
                let val = self.ppu_status;
                self.ppu_status &= 0x7F;
                val
            }
            0x2004 => {
                // OAMDATA
                self.oam[self.oam_addr as usize]
            }
            0x2007 => {
                // PPUDATA
                let mut addr = self.v;
                if [0x3F10, 0x3F14, 0x3F18, 0x3F1C].contains(&addr) {
                    addr -= 0x10;
                }
                addr %= 0x4000;
                let val = self.vram[addr as usize];
                self.v += match self.ppu_ctrl & (1 << 2) != 0 {
                    false => 1,
                    true => 32,
                };
                self.v %= 0x8000;

                if (0..0x4000).contains(&addr) {
                    let ret = self.read_buffer;
                    self.read_buffer = val;
                    ret
                } else {
                    val
                }
            }
            _ => panic!("Invalid PPU address {addr:#06x}"),
        }
    }

    pub fn nmi_interrupt(&self) -> bool {
        self.ppu_ctrl & (1 << 7) != 0
    }

    fn background_enabled(&self) -> bool {
        self.ppu_mask & (1 << 3) != 0
    }

    fn sprites_enabled(&self) -> bool {
        self.ppu_mask & (1 << 4) != 0
    }

    fn sprites_8x16(&self) -> bool {
        self.ppu_ctrl & (1 << 5) != 0
    }

    fn vertical_mirror(&self) -> bool {
        // idk??!?!
        true
    }

    fn read_nametable(&self) -> u8 {
        //println!("{:#x}", self.v as usize & 0xFFF);
        self.vram[0x2000 | (self.v as usize & 0xFFF)]
    }

    fn read_attr(&self) -> u8 {
        self.vram[0x23c0
            | (self.v as usize & 0xC00)
            | ((self.v as usize >> 4) & 0x38)
            | ((self.v as usize >> 2) & 0x7)]
    }

    fn read_pt(&self, row: u32, nt: u8, high: bool, sprite_y: Option<u8>, tile_idx: u8) -> u8 {
        let base_addr = if let Some(y) = sprite_y {
            if self.sprites_8x16() {
                // this is wrong TODO
                if row - y as u32 >= 8 {
                    0x1000 + tile_idx as usize >> 1
                } else {
                    0 + tile_idx as usize >> 1
                }
            } else {
                //println!("{}", (self.ppu_ctrl as usize & 0x8) << 9);
                ((self.ppu_ctrl as usize & 0x8) << 9) + tile_idx as usize * 16
            }
        } else {
            ((self.ppu_ctrl as usize & 0x10) << 8) + ((nt as usize) << 4)
        };

        let y_off = if let Some(y) = sprite_y {
            if y == 0xFF {
                0 // uhhhhh
            } else {
                (row.wrapping_sub(y as u32)) as usize % 8
            }
        } else {
            row as usize % 8
        };

        self.vram[base_addr + high as usize * 8 + y_off]
    }

    fn render_cycle(&mut self, row: u32, cycle: u32) {
        let bg_palettes = [0, 1, 2, 3].map(|i| {
            let c0 = self.vram[0x3F00];
            let c1 = self.vram[0x3F00 + 4 * i + 1];
            let c2 = self.vram[0x3F00 + 4 * i + 2];
            let c3 = self.vram[0x3F00 + 4 * i + 3];
            [c0, c1, c2, c3]
        });

        let sprite_palettes = [0, 1, 2, 3].map(|i| {
            let c0 = self.vram[0x3F00];
            let c1 = self.vram[0x3F10 + 4 * i + 1];
            let c2 = self.vram[0x3F10 + 4 * i + 2];
            let c3 = self.vram[0x3F10 + 4 * i + 3];
            [c0, c1, c2, c3]
        });

        // clear status bits
        if row == 261 && cycle == 1 {
            self.ppu_status &= !(0b11100000);
        }

        // sprite evaluation for the next scan line
        if (1..=64).contains(&cycle) {
            // Initialize the secondary_oam to 0xFF
            // On the NES, each memory access took 2 cycles, so we'll just only write on the second
            // cycle
            if (cycle - 1) % 2 == 1 {
                self.secondary_oam[(cycle as usize - 1) / 2] = 0xFF;
            }
        }
        if cycle == 256 {
            // bumildhgbum this looks like such a pain to implement correctly to the cycle so im
            // just going to do everything in the last cycle
            // This copies the sprites on the next scanline into the secondary oam
            let sprite_height = match self.ppu_ctrl & (1 << 5) != 0 {
                false => 8,
                true => 16,
            };
            let mut s_oam_idx = 0;
            let mut final_idx = 0;
            for (sprite_idx, sprite) in self.oam.chunks(4).enumerate() {
                let y_upper = sprite[0];
                let y_lower = y_upper.saturating_add(sprite_height);
                final_idx = sprite_idx;
                if (y_upper..y_lower).contains(&(row as u8)) {
                    for (i, &val) in sprite.iter().enumerate() {
                        self.secondary_oam[s_oam_idx * 4 + i] = val;
                    }
                    s_oam_idx += 1;
                    if s_oam_idx == 8 {
                        break;
                    }
                }
            }
            // check for sprite overflow, with a bug included!
            // the bug comes from somehow incrementing the relative offset every time a byte is
            // read
            // idk if this offset overflows at 4 though, im just going to assume it does
            // because then i dont have to check for out of bounds stuff
            let mut overflow_bug = 0;
            for i in final_idx..64 {
                let y_upper = self.oam[i * 4 + overflow_bug];
                let y_lower = y_upper.saturating_add(sprite_height);
                if (y_upper..y_lower).contains(&(row as u8)) {
                    self.ppu_status |= 1 << 5;
                    break;
                }
                overflow_bug += 1;
                overflow_bug %= 4;
            }

            //println!("{} {} {}", self.scroll_x, self.fine_x, self.scroll_y);
        }

        // move the y scroll coordinate down
        if cycle == 256 {
            if self.v & 0x7000 != 0x7000 {
                //println!("7 {:#x}", self.v & 0x7000);
                self.v += 0x1000;
            } else {
                self.v &= !0x7000;
                let mut y = (self.v & (0b11111 << 5)) >> 5;
                if y == 29 {
                    y = 0;
                    self.v ^= 0x800;
                } else if y == 31 {
                    y = 0;
                } else {
                    y += 1;
                }
                self.v = (self.v & !(0b11111 << 5)) | (y << 5);
            }
        }
        // copy the horizontal scroll from t to v
        if cycle == 257 {
            let mask = 0b11111 | 0b1 << 10;
            self.v &= !mask;
            self.v |= self.t & mask;
        }
        if (280..=304).contains(&cycle) && row == 261 && self.background_enabled() {
            let mask = 0b1111 << 11 | 0b11111 << 5;
            self.v &= !mask;
            self.v |= self.t & mask;
        }

        // rendering
        if cycle == 0 {
            // do nothing lol
        } else if (1..=256).contains(&cycle) {
            let tile = ((cycle - 1) / 8 + 2) % 32;
            let rel_cycle = (cycle - 1) % 8;
            match rel_cycle {
                1 => self.l_nametable = self.read_nametable(),
                3 => self.l_attr = self.read_attr(),
                5 => self.l_pt_low = self.read_pt(row, self.l_nametable, false, None, 0),
                7 => self.l_pt_high = self.read_pt(row, self.l_nametable, true, None, 0),
                _ => (),
            }

            // increment v horizontal scroll
            if rel_cycle % 8 == 7 {
                if self.v % 32 == 31 {
                    self.v &= !0x1F;
                    self.v ^= 0x400;
                } else {
                    self.v += 1;
                }
            }

            /*
            if row == 124 && tile == 2 && rel_cycle == 7 {
                //((self.ppu_ctrl as usize & 0x10) << 8) + ((nt as usize) << 4)
                println!(
                    "{} {} {:#x} {:#x} {:#x} {} {:#x}",
                    self.l_pt_low,
                    self.l_pt_high,
                    self.l_attr,
                    self.l_nametable,
                    ((self.ppu_ctrl as usize & 0x10) << 8) + ((self.l_nametable as usize) << 4),
                    self.vram[0x1240 + 1],
                    self.v,
                );
            }
                */

            // draw pixel
            if row < 240 {
                let palette = (self.b_pal_reg[1] & (1 << self.x)) >> self.x << 1
                    | (self.b_pal_reg[0] & (1 << self.x)) >> self.x;
                let pixel = (self.b_pat_reg[1] & (1 << self.x)) >> self.x << 1
                    | (self.b_pat_reg[0] & (1 << self.x)) >> self.x;
                //let pixel = 1;
                self.b_pal_reg[0] >>= 1;
                self.b_pal_reg[0] |= (self.b_pal_latch[0] as u8) << 7;
                self.b_pal_reg[1] >>= 1;
                self.b_pal_reg[1] |= (self.b_pal_latch[1] as u8) << 7;
                self.b_pat_reg[0] >>= 1;
                self.b_pat_reg[1] >>= 1;

                let bg_pixel = pixel;
                let bg_col = if self.background_enabled() {
                    bg_palettes[palette as usize][pixel as usize] as usize
                } else {
                    0
                };
                //assert!(bg_col < 64);

                let mut sprite_pixel = 0;
                let mut sprite_col = 0;
                let mut sprite_priority = true;
                for (i, x) in self.s_counters.into_iter().enumerate().rev() {
                    if x == 0 {
                        let palette = self.s_attrs[i] & 0x3;
                        let pixel = (self.s_pat_reg[i][1] & 1) << 1 | (self.s_pat_reg[i][0] & 1);
                        self.s_pat_reg[i][0] >>= 1;
                        self.s_pat_reg[i][1] >>= 1;

                        if pixel != 0 {
                            sprite_pixel = pixel;
                            sprite_col = if self.sprites_enabled() {
                                sprite_palettes[palette as usize][pixel as usize] as usize
                            } else {
                                0
                            };
                            sprite_priority = self.s_attrs[i] & (1 << 5) == 0;
                        }
                    }
                }
                for x in self.s_counters.iter_mut() {
                    if *x != 0 {
                        *x -= 1;
                    }
                }
                //assert!(sprite_col < 64);

                let col = if bg_pixel == 0 && sprite_pixel == 0 {
                    bg_col
                } else if bg_pixel == 0 && sprite_pixel != 0 {
                    sprite_col
                } else if bg_pixel != 0 && sprite_pixel == 0 {
                    bg_col
                } else {
                    self.ppu_status |= 0x40;
                    if sprite_priority {
                        sprite_col
                    } else {
                        bg_col
                    }
                } % 64;

                let red = PALETTE[col][0];
                let green = PALETTE[col][1];
                let blue = PALETTE[col][2];

                let x = cycle as usize - 1;
                let y = row as usize;

                /*
                if y == 0 && x == 2 {
                    println!("{}", pixel);
                }
                    */

                let pitch = 3 * 256;
                let offset = y * pitch + x * 3;
                self.pixel_data[offset] = red;
                self.pixel_data[offset + 1] = green;
                self.pixel_data[offset + 2] = blue;
            }

            if cycle % 8 == 0 {
                let mut pal = self.l_attr;
                let tile = tile + self.scroll_x as u32 / 8;
                if tile & 2 == 2 {
                    pal >>= 2;
                }
                if (row / 8) & 2 == 2 {
                    pal >>= 4;
                }
                pal &= 3;
                self.b_pat_reg[0] |= (self.l_pt_low.reverse_bits() as u16) << 8;
                self.b_pat_reg[1] |= (self.l_pt_high.reverse_bits() as u16) << 8;
                self.b_pal_latch[0] = (pal & 1) != 0;
                self.b_pal_latch[1] = (pal >> 1) != 0;
            }
        } else if (257..=320).contains(&cycle) {
            let rel_cycle = (cycle - 1) % 8;
            let row = (row + 1) % 240;
            let s_oam_idx = (cycle as usize - 257) / 8;

            let y = self.secondary_oam[s_oam_idx * 4].wrapping_add(1);
            let tile_idx = self.secondary_oam[s_oam_idx * 4 + 1];
            let attr = self.secondary_oam[s_oam_idx * 4 + 2];
            let x = self.secondary_oam[s_oam_idx * 4 + 3];

            match rel_cycle {
                1 => self.l_nametable = 0xAA, // garbage
                3 => self.l_attr = 0xAA,      // garbage
                5 => self.l_pt_low = self.read_pt(row, 0, false, Some(y), tile_idx),
                7 => self.l_pt_high = self.read_pt(row, 0, true, Some(y), tile_idx),
                _ => (),
            }

            if cycle % 8 == 0 {
                let flip = attr & (1 << 6) != 0;
                if y == 0xFF {
                    self.s_pat_reg[s_oam_idx][0] = 0;
                    self.s_pat_reg[s_oam_idx][1] = 0;
                } else {
                    self.s_pat_reg[s_oam_idx][0] = if flip {
                        self.l_pt_low
                    } else {
                        self.l_pt_low.reverse_bits()
                    };
                    self.s_pat_reg[s_oam_idx][1] = if flip {
                        self.l_pt_high
                    } else {
                        self.l_pt_high.reverse_bits()
                    };
                }
                self.s_attrs[s_oam_idx] = attr;
                self.s_counters[s_oam_idx] = x;
            }
        } else if (321..=336).contains(&cycle) {
            let tile = (cycle - 1) / 8 - 40;
            let row = (row + 1) % 240;

            let rel_cycle = (cycle - 1) % 8;

            match rel_cycle {
                1 => self.l_nametable = self.read_nametable(),
                3 => self.l_attr = self.read_attr(),
                5 => self.l_pt_low = self.read_pt(row, self.l_nametable, false, None, 0),
                7 => self.l_pt_high = self.read_pt(row, self.l_nametable, true, None, 0),
                _ => (),
            }

            // increment v horizontal scroll
            if rel_cycle % 8 == 7 {
                if self.v % 32 == 31 {
                    self.v &= !0x1F;
                    self.v ^= 0x400;
                } else {
                    self.v += 1;
                }
            }

            self.b_pal_reg[0] >>= 1;
            self.b_pal_reg[0] |= (self.b_pal_latch[0] as u8) << 7;
            self.b_pal_reg[1] >>= 1;
            self.b_pal_reg[1] |= (self.b_pal_latch[1] as u8) << 7;
            self.b_pat_reg[0] >>= 1;
            self.b_pat_reg[1] >>= 1;

            if cycle % 8 == 0 {
                let mut pal = self.l_attr;
                let tile = tile + self.scroll_x as u32 / 8;
                if tile & 2 == 2 {
                    pal >>= 2;
                }
                if (row / 8) & 2 == 2 {
                    pal >>= 4;
                }
                pal &= 3;
                self.b_pat_reg[0] |= (self.l_pt_low.reverse_bits() as u16) << 8;
                self.b_pat_reg[1] |= (self.l_pt_high.reverse_bits() as u16) << 8;
                self.b_pal_latch[0] = (pal & 1) != 0;
                self.b_pal_latch[1] = (pal >> 1) != 0;
            }
        } else if (337..=340).contains(&cycle) {
            // todo
        }
    }

    pub fn cycle<M: MemoryDevice>(
        &mut self,
        texture: &mut Texture<'_>,
        cpu: &mut Cpu<'_, M>,
    ) -> anyhow::Result<()> {
        if (0..240).contains(&(self.cycle / 341)) {
            let row = self.cycle / 341;
            let cycle = self.cycle % 341;
            self.render_cycle(row, cycle);
        } else if self.cycle / 341 == 261 {
            let row = self.cycle / 341;
            let cycle = self.cycle % 341;
            self.render_cycle(row, cycle);
        }
        if self.cycle == 341 * 241 + 1 {
            self.ppu_status |= 0x80;
            if self.nmi_interrupt() {
                cpu.nmi_interrupt();
            }
        }

        self.cycle += 1;

        self.cycle %= 262 * 341;

        if self.cycle == 0 {
            texture.update(None, &self.pixel_data, 256 * 3)?;
        }

        Ok(())
    }
}
