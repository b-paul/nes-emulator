use super::opcode::*;

pub trait MemoryDevice {
    fn read_addr(&self, addr: u16) -> u8;
    fn write_addr(&self, addr: u16, val: u8);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cpu<'a, M: MemoryDevice> {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub brk: bool,
    pub overflow: bool,
    pub negative: bool,
    memory: &'a M,
}

impl<'a, M: MemoryDevice> Cpu<'a, M> {
    pub fn new(memory: &'a M) -> Self {
        let mut cpu = Cpu {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            brk: false,
            overflow: false,
            negative: false,
            memory,
        };

        // https://www.nesdev.org/wiki/CPU_power_up_state idk
        cpu.set_status_byte(0x34);
        cpu.a = 0;
        cpu.x = 0;
        cpu.y = 0;
        cpu.sp = 0xFD;

        // memory stuf with ppu blahblah no ppu implemented yet though
        // ...

        // Set the program counter to the value in the reset vector (0xFFFC-0xFFFD)
        let l = cpu.memory.read_addr(0xFFFC) as u16;
        let h = cpu.memory.read_addr(0xFFFD) as u16;
        cpu.pc = (h << 8) | l;

        cpu
    }

    fn get_addr(&mut self, addressing: Addressing) -> u16 {
        match addressing {
            Addressing::ZeroPage(addr) => addr as u16,
            Addressing::ZeroPageX(addr) => addr.wrapping_add(self.x) as u16,
            Addressing::ZeroPageY(addr) => addr.wrapping_add(self.y) as u16,
            Addressing::Absolute(addr) => addr,
            Addressing::AbsoluteX(addr) => addr.wrapping_add(self.x as u16),
            Addressing::AbsoluteY(addr) => addr.wrapping_add(self.y as u16),
            Addressing::Indirect(addr) => {
                let page = addr & 0xFF00;
                let base = addr & 0x00FF;
                let base2 = (base + 1) & 0x00FF;
                u16::from_le_bytes([self.memory.read_addr(page + base), self.memory.read_addr(page + base2)])
            }
            Addressing::IndirectX(addr) => {
                let addr = addr.wrapping_add(self.x) as u16;
                u16::from_le_bytes([self.memory.read_addr(addr), self.memory.read_addr(addr + 1)])
            }
            Addressing::IndirectY(addr) => u16::from_le_bytes([
                self.memory.read_addr(addr as u16),
                self.memory.read_addr(addr as u16 + 1),
            ])
            .wrapping_add(self.y as u16),
            a => panic!("get_addr shouldn't be called with {a:?} probably"),
        }
    }

    fn read_arg(&mut self, addressing: Addressing) -> u8 {
        match addressing {
            Addressing::Implied => todo!("idk if something is meant to happen"),
            Addressing::Accumulator => self.a,
            Addressing::Immediate(n) => n,
            Addressing::Relative(_) => todo!("idk"),
            Addressing::Indirect(_) => todo!("idk"),
            addressing => self.memory.read_addr(self.get_addr(addressing)),
        }
    }

    fn status_byte(&self, brk: bool) -> u8 {
        (self.carry as u8)
            | (self.zero as u8) << 1
            | (self.interrupt as u8) << 2
            | (self.decimal as u8) << 3
            | 1 << 4
            | (brk as u8) << 5
            | (self.overflow as u8) << 6
            | (self.negative as u8) << 7
    }

    fn set_status_byte(&mut self, b: u8) {
        self.carry = b & 1 != 0;
        self.zero = b & (1 << 1) != 0;
        self.interrupt = b & (1 << 2) != 0;
        self.decimal = b & (1 << 3) != 0;
        self.overflow = b & (1 << 6) != 0;
        self.negative = b & (1 << 7) != 0;
    }

    /// Push a byte onto the stack
    fn push(&mut self, b: u8) {
        let addr = 0x0100 + self.sp as u16;
        self.memory.write_addr(addr, b);
        self.sp = self.sp.wrapping_sub(1);
    }

    /// Pop a byte off of the stack
    fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let addr = 0x0100 + self.sp as u16;
        self.memory.read_addr(addr)
    }

    pub fn nmi_interrupt(&mut self) {
        self.push((self.pc >> 8) as u8);
        self.push(self.pc as u8);
        self.push(self.status_byte(false));
        let l = self.memory.read_addr(0xFFFA) as u16;
        let h = self.memory.read_addr(0xFFFB) as u16;
        self.pc = (h << 8) | l;
    }

    fn adc(&mut self, n: u8) {
        (self.a, self.carry) = self.a.carrying_add(n, self.carry);
        self.zero = self.a == 0;
        self.negative = self.a & 0x80 != 0;
    }

    fn and(&mut self, n: u8) {
        self.a &= n;
        self.zero = self.a == 0;
        self.negative = self.a & 0x80 != 0;
    }

    fn branch(&mut self, condition: bool, addressing: Addressing) {
        match addressing {
            Addressing::Relative(off) => {
                if !condition {
                    return;
                }
                self.pc = self.pc.wrapping_add(off as i8 as i16 as u16)
            }
            _ => panic!("Branching with a non relative addressing mode"),
        }
    }

    fn cmp(&mut self, n: u8) {
        self.carry = self.a >= n;
        self.zero = self.a == n;
        self.negative = (self.a as i8).wrapping_sub(n as i8) & (1 << 7) != 0;
    }

    fn cpx(&mut self, n: u8) {
        self.carry = self.x >= n;
        self.zero = self.x == n;
        self.negative = (self.x as i8).wrapping_sub(n as i8) & (1 << 7) != 0;
    }

    fn cpy(&mut self, n: u8) {
        self.carry = self.y >= n;
        self.zero = self.y == n;
        self.negative = (self.y as i8).wrapping_sub(n as i8) & (1 << 7) != 0;
    }

    fn eor(&mut self, n: u8) {
        self.a ^= n;
        self.zero = self.a == 0;
        self.negative = self.a & (1 << 7) != 0;
    }

    fn lda(&mut self, n: u8) {
        self.a = n;
        self.zero = self.a == 0;
        self.negative = self.a & (1 << 7) != 0;
    }

    fn ldx(&mut self, n: u8) {
        self.x = n;
        self.zero = self.x == 0;
        self.negative = self.x & (1 << 7) != 0;
    }

    fn ldy(&mut self, n: u8) {
        self.y = n;
        self.zero = self.y == 0;
        self.negative = self.y & (1 << 7) != 0;
    }

    fn ora(&mut self, n: u8) {
        self.a |= n;
        self.zero = self.a == 0;
        self.negative = self.a & (1 << 7) != 0;
    }

    fn sbc(&mut self, n: u8) {
        let n = n as i8;
        let n = n.wrapping_add(1);
        let n = (n.wrapping_neg()) as u8;
        (self.a, self.carry) = self.a.carrying_add(n, self.carry);
        self.zero = self.a == 0;
        self.negative = self.a & (1 << 7) != 0;
    }

    pub fn run_instruction(&mut self) -> usize {
        let (opcode, next_pc, cycles) = read_instruction(self.memory, self.pc);
        //println!("{:#4x}", self.memory.read_addr(self.pc));
        //println!("{opcode:?} {:#06x}", self.pc);
        self.pc = next_pc;
        match opcode.0 {
            Instruction::Adc => {
                let n = self.read_arg(opcode.1);
                self.adc(n);
            }
            Instruction::And => {
                let n = self.read_arg(opcode.1);
                self.and(n);
            }
            Instruction::Asl => {
                let m = self.read_arg(opcode.1);
                let tmp = m << 1;
                self.carry = m & (1 << 7) != 0;
                self.negative = tmp & (1 << 7) != 0;
                self.zero = tmp == 0;
                match opcode.1 {
                    Addressing::Accumulator => {
                        self.a = tmp;
                    }
                    Addressing::ZeroPage(_)
                    | Addressing::ZeroPageX(_)
                    | Addressing::Absolute(_)
                    | Addressing::AbsoluteX(_) => {
                        let addr = self.get_addr(opcode.1);
                        self.memory.write_addr(addr, tmp);
                    }
                    _ => panic!(),
                }
            }
            Instruction::Bcc => self.branch(!self.carry, opcode.1),
            Instruction::Bcs => self.branch(self.carry, opcode.1),
            Instruction::Beq => self.branch(self.zero, opcode.1),
            Instruction::Bit => {
                let m = self.read_arg(opcode.1);
                let tmp = self.a & m;
                self.overflow = m & (1 << 6) != 0;
                self.negative = m & (1 << 7) != 0;
                self.zero = tmp == 0;
            }
            Instruction::Bmi => self.branch(self.negative, opcode.1),
            Instruction::Bne => self.branch(!self.zero, opcode.1),
            Instruction::Bpl => self.branch(!self.negative, opcode.1),
            Instruction::Brk => {
                self.pc += 1;
                self.push((self.pc >> 8) as u8);
                self.push(self.pc as u8);
                self.push(self.status_byte(true));
                let l = self.memory.read_addr(0xFFFE) as u16;
                let h = self.memory.read_addr(0xFFFF) as u16;
                self.pc = (h << 8) | l;
                self.brk = true;
                self.interrupt = true;
            }
            Instruction::Bvc => self.branch(!self.overflow, opcode.1),
            Instruction::Bvs => self.branch(self.overflow, opcode.1),
            Instruction::Clc => self.carry = false,
            Instruction::Cld => self.decimal = false,
            Instruction::Cli => self.interrupt = false,
            Instruction::Clv => self.overflow = false,
            Instruction::Cmp => {
                let n = self.read_arg(opcode.1);
                self.cmp(n);
            }
            Instruction::Cpx => {
                let n = self.read_arg(opcode.1);
                self.cpx(n);
            }
            Instruction::Cpy => {
                let n = self.read_arg(opcode.1);
                self.cpy(n);
            }
            Instruction::Dec => {
                let addr = self.get_addr(opcode.1);
                let n = self.memory.read_addr(addr);
                let n = n.wrapping_sub(1);
                self.zero = n == 0;
                self.negative = n & (1 << 7) != 0;
                self.memory.write_addr(addr, n);
            }
            Instruction::Dex => {
                self.x = self.x.wrapping_sub(1);
                self.zero = self.x == 0;
                self.negative = self.x & (1 << 7) != 0;
            }
            Instruction::Dey => {
                self.y = self.y.wrapping_sub(1);
                self.zero = self.y == 0;
                self.negative = self.y & (1 << 7) != 0;
            }
            Instruction::Eor => {
                let n = self.read_arg(opcode.1);
                self.eor(n);
            }
            Instruction::Inc => {
                let addr = self.get_addr(opcode.1);
                let n = self.memory.read_addr(addr);
                let n = n.wrapping_add(1);
                self.zero = n == 0;
                self.negative = n & (1 << 7) != 0;
                self.memory.write_addr(addr, n);
            }
            Instruction::Inx => {
                self.x = self.x.wrapping_add(1);
                self.zero = self.x == 0;
                self.negative = self.x & (1 << 7) != 0;
            }
            Instruction::Iny => {
                self.y = self.y.wrapping_add(1);
                self.zero = self.y == 0;
                self.negative = self.y & (1 << 7) != 0;
            }
            Instruction::Jmp => {
                let addr = self.get_addr(opcode.1);
                self.pc = addr;
            }
            Instruction::Jsr => {
                // apparently it pushes pc - 1 lol
                let addr = self.get_addr(opcode.1);
                let pc = self.pc.wrapping_sub(1);
                self.push((pc >> 8) as u8);
                self.push(pc as u8);
                self.pc = addr;
            }
            Instruction::Lda => {
                let n = self.read_arg(opcode.1);
                self.lda(n);
            }
            Instruction::Ldx => {
                let n = self.read_arg(opcode.1);
                self.ldx(n);
            }
            Instruction::Ldy => {
                let n = self.read_arg(opcode.1);
                self.ldy(n);
            }
            Instruction::Lsr => {
                let m = self.read_arg(opcode.1);
                let tmp = m >> 1;
                self.carry = m & (1 << 0) != 0;
                self.negative = tmp & (1 << 7) != 0;
                self.zero = tmp == 0;
                match opcode.1 {
                    Addressing::Accumulator => {
                        self.a = tmp;
                    }
                    Addressing::ZeroPage(_)
                    | Addressing::ZeroPageX(_)
                    | Addressing::Absolute(_)
                    | Addressing::AbsoluteX(_) => {
                        let addr = self.get_addr(opcode.1);
                        self.memory.write_addr(addr, tmp);
                    }
                    _ => panic!(),
                }
            }
            Instruction::Nop => (),
            Instruction::Ora => {
                let n = self.read_arg(opcode.1);
                self.ora(n);
            }
            Instruction::Pha => self.push(self.a),
            Instruction::Php => self.push(self.status_byte(self.brk)),
            Instruction::Pla => {
                self.a = self.pop();
                self.zero = self.a == 0;
                self.negative = self.a & (1 << 7) != 0;
            }
            Instruction::Plp => {
                let status = self.pop();
                self.set_status_byte(status);
            }
            Instruction::Rol => {
                let m = self.read_arg(opcode.1);
                let mut tmp = m << 1;
                tmp |= self.carry as u8;
                self.carry = m & (1 << 7) != 0;
                self.negative = tmp & (1 << 7) != 0;
                self.zero = tmp == 0;
                match opcode.1 {
                    Addressing::Accumulator => {
                        self.a = tmp;
                    }
                    Addressing::ZeroPage(_)
                    | Addressing::ZeroPageX(_)
                    | Addressing::Absolute(_)
                    | Addressing::AbsoluteX(_) => {
                        let addr = self.get_addr(opcode.1);
                        self.memory.write_addr(addr, tmp);
                    }
                    _ => panic!(),
                }
            }
            Instruction::Ror => {
                let m = self.read_arg(opcode.1);
                let mut tmp = m >> 1;
                tmp |= (self.carry as u8) << 7;
                self.carry = m & (1 << 0) != 0;
                self.negative = tmp & (1 << 7) != 0;
                self.zero = tmp == 0;
                match opcode.1 {
                    Addressing::Accumulator => {
                        self.a = tmp;
                    }
                    Addressing::ZeroPage(_)
                    | Addressing::ZeroPageX(_)
                    | Addressing::Absolute(_)
                    | Addressing::AbsoluteX(_) => {
                        let addr = self.get_addr(opcode.1);
                        self.memory.write_addr(addr, tmp);
                    }
                    _ => panic!(),
                }
            }
            Instruction::Rti => {
                let status = self.pop();
                let l = self.pop() as u16;
                let h = self.pop() as u16;
                self.pc = (h << 8) | l;
                self.set_status_byte(status);
            }
            Instruction::Rts => {
                let l = self.pop() as u16;
                let h = self.pop() as u16;
                self.pc = ((h << 8) | l).wrapping_add(1);
            }
            Instruction::Sbc => {
                let n = self.read_arg(opcode.1);
                self.sbc(n);
            }
            Instruction::Sec => self.carry = true,
            Instruction::Sed => self.decimal = true,
            Instruction::Sei => self.interrupt = true,
            Instruction::Sta => {
                self.memory.write_addr(self.get_addr(opcode.1), self.a)
            }
            Instruction::Stx => self.memory.write_addr(self.get_addr(opcode.1), self.x),
            Instruction::Sty => self.memory.write_addr(self.get_addr(opcode.1), self.y),
            Instruction::Tax => {
                self.x = self.a;
                self.zero = self.x == 0;
                self.negative = self.x & (1 << 7) != 0;
            }
            Instruction::Tay => {
                self.y = self.a;
                self.zero = self.y == 0;
                self.negative = self.y & (1 << 7) != 0;
            }
            Instruction::Tsx => {
                self.x = self.sp;
                self.zero = self.x == 0;
                self.negative = self.x & (1 << 7) != 0;
            }
            Instruction::Txa => {
                self.a = self.x;
                self.zero = self.a == 0;
                self.negative = self.a & (1 << 7) != 0;
            }
            Instruction::Txs => self.sp = self.x,
            Instruction::Tya => {
                self.a = self.y;
                self.zero = self.a == 0;
                self.negative = self.a & (1 << 7) != 0;
            }
        }
        return cycles;
    }
}
