use crate::cpu::MemoryDevice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpCode(pub Instruction, pub Addressing);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Addressing {
    Implied,
    Accumulator,
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Relative(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u16),
    IndirectX(u8),
    IndirectY(u8),
}

/// Reads an instruction and gives the new instruction index after that instruction
pub fn read_instruction<M: MemoryDevice> (pmem: &M, index: u16) -> (OpCode, u16, usize) {
    use Instruction as I;
    use Addressing as A;
    match pmem.read_addr(index) {
        0x69 => (OpCode(I::Adc, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0x65 => (OpCode(I::Adc, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x75 => (OpCode(I::Adc, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x6D => (OpCode(I::Adc, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x7D => (OpCode(I::Adc, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x79 => (OpCode(I::Adc, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x61 => (OpCode(I::Adc, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0x71 => (OpCode(I::Adc, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0x29 => (OpCode(I::And, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0x25 => (OpCode(I::And, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x35 => (OpCode(I::And, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x2D => (OpCode(I::And, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x3D => (OpCode(I::And, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x39 => (OpCode(I::And, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x21 => (OpCode(I::And, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0x31 => (OpCode(I::And, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0x0A => (OpCode(I::Asl, A::Accumulator), index + 1, 2),
        0x06 => (OpCode(I::Asl, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0x16 => (OpCode(I::Asl, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0x0E => (OpCode(I::Asl, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0x1E => (OpCode(I::Asl, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0x90 => (OpCode(I::Bcc, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0xB0 => (OpCode(I::Bcs, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0xF0 => (OpCode(I::Beq, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0x24 => (OpCode(I::Bit, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x2C => (OpCode(I::Bit, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0x30 => (OpCode(I::Bmi, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0xD0 => (OpCode(I::Bne, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0x10 => (OpCode(I::Bpl, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0x00 => (OpCode(I::Brk, A::Implied), index + 1, 7),

        0x50 => (OpCode(I::Bvc, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0x70 => (OpCode(I::Bvs, A::Relative(pmem.read_addr(index + 1))), index + 2, 2),

        0x18 => (OpCode(I::Clc, A::Implied), index + 1, 2),

        0xD8 => (OpCode(I::Cld, A::Implied), index + 1, 2),

        0x58 => (OpCode(I::Cli, A::Implied), index + 1, 2),

        0xB8 => (OpCode(I::Clv, A::Implied), index + 1, 2),

        0xC9 => (OpCode(I::Cmp, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xC5 => (OpCode(I::Cmp, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xD5 => (OpCode(I::Cmp, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0xCD => (OpCode(I::Cmp, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xDD => (OpCode(I::Cmp, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xD9 => (OpCode(I::Cmp, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xC1 => (OpCode(I::Cmp, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0xD1 => (OpCode(I::Cmp, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0xE0 => (OpCode(I::Cpx, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xE4 => (OpCode(I::Cpx, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xEC => (OpCode(I::Cpx, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0xC0 => (OpCode(I::Cpy, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xC4 => (OpCode(I::Cpy, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xCC => (OpCode(I::Cpy, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0xC6 => (OpCode(I::Dec, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0xD6 => (OpCode(I::Dec, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0xCE => (OpCode(I::Dec, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0xDE => (OpCode(I::Dec, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0xCA => (OpCode(I::Dex, A::Implied), index + 1, 2),

        0x88 => (OpCode(I::Dey, A::Implied), index + 1, 2),

        0x49 => (OpCode(I::Eor, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0x45 => (OpCode(I::Eor, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x55 => (OpCode(I::Eor, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x4D => (OpCode(I::Eor, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x5D => (OpCode(I::Eor, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x59 => (OpCode(I::Eor, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x41 => (OpCode(I::Eor, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0x51 => (OpCode(I::Eor, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0xE6 => (OpCode(I::Inc, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0xF6 => (OpCode(I::Inc, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0xEE => (OpCode(I::Inc, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0xFE => (OpCode(I::Inc, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0xE8 => (OpCode(I::Inx, A::Implied), index + 1, 2),

        0xC8 => (OpCode(I::Iny, A::Implied), index + 1, 2),

        0x4C => (OpCode(I::Jmp, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 3),
        0x6C => (OpCode(I::Jmp, A::Indirect(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 5),

        0x20 => (OpCode(I::Jsr, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),

        0xA9 => (OpCode(I::Lda, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xA5 => (OpCode(I::Lda, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xB5 => (OpCode(I::Lda, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0xAD => (OpCode(I::Lda, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xBD => (OpCode(I::Lda, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xB9 => (OpCode(I::Lda, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xA1 => (OpCode(I::Lda, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0xB1 => (OpCode(I::Lda, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0xA2 => (OpCode(I::Ldx, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xA6 => (OpCode(I::Ldx, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xB6 => (OpCode(I::Ldx, A::ZeroPageY(pmem.read_addr(index + 1))), index + 2, 4),
        0xAE => (OpCode(I::Ldx, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xBE => (OpCode(I::Ldx, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0xA0 => (OpCode(I::Ldy, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xA4 => (OpCode(I::Ldy, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xB4 => (OpCode(I::Ldy, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0xAC => (OpCode(I::Ldy, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xBC => (OpCode(I::Ldy, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0x4A => (OpCode(I::Lsr, A::Accumulator), index + 1, 2),
        0x46 => (OpCode(I::Lsr, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0x56 => (OpCode(I::Lsr, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0x4E => (OpCode(I::Lsr, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0x5E => (OpCode(I::Lsr, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0xEA => (OpCode(I::Nop, A::Implied), index + 1, 2),

        0x09 => (OpCode(I::Ora, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0x05 => (OpCode(I::Ora, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x15 => (OpCode(I::Ora, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x0D => (OpCode(I::Ora, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x1D => (OpCode(I::Ora, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x19 => (OpCode(I::Ora, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x01 => (OpCode(I::Ora, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0x11 => (OpCode(I::Ora, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0x48 => (OpCode(I::Pha, A::Implied), index + 1, 3),

        0x08 => (OpCode(I::Php, A::Implied), index + 1, 3),

        0x68 => (OpCode(I::Pla, A::Implied), index + 1, 4),

        0x28 => (OpCode(I::Plp, A::Implied), index + 1, 4),

        0x2A => (OpCode(I::Rol, A::Accumulator), index + 1, 2),
        0x26 => (OpCode(I::Rol, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0x36 => (OpCode(I::Rol, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0x2E => (OpCode(I::Rol, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0x3E => (OpCode(I::Rol, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0x6A => (OpCode(I::Ror, A::Accumulator), index + 1, 2),
        0x66 => (OpCode(I::Ror, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 5),
        0x76 => (OpCode(I::Ror, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 6),
        0x6E => (OpCode(I::Ror, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 6),
        0x7E => (OpCode(I::Ror, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 7),

        0x40 => (OpCode(I::Rti, A::Implied), index + 1, 6),

        0x60 => (OpCode(I::Rts, A::Implied), index + 1, 6),

        0xE9 => (OpCode(I::Sbc, A::Immediate(pmem.read_addr(index + 1))), index + 2, 2),
        0xE5 => (OpCode(I::Sbc, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0xF5 => (OpCode(I::Sbc, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0xED => (OpCode(I::Sbc, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xFD => (OpCode(I::Sbc, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xF9 => (OpCode(I::Sbc, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0xE1 => (OpCode(I::Sbc, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0xF1 => (OpCode(I::Sbc, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 5),

        0x38 => (OpCode(I::Sec, A::Implied), index + 1, 2),

        0xF8 => (OpCode(I::Sed, A::Implied), index + 1, 2),

        0x78 => (OpCode(I::Sei, A::Implied), index + 1, 2),

        0x85 => (OpCode(I::Sta, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x95 => (OpCode(I::Sta, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x8D => (OpCode(I::Sta, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),
        0x9D => (OpCode(I::Sta, A::AbsoluteX(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 5),
        0x99 => (OpCode(I::Sta, A::AbsoluteY(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 5),
        0x81 => (OpCode(I::Sta, A::IndirectX(pmem.read_addr(index + 1))), index + 2, 6),
        0x91 => (OpCode(I::Sta, A::IndirectY(pmem.read_addr(index + 1))), index + 2, 6),

        0x86 => (OpCode(I::Stx, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x96 => (OpCode(I::Stx, A::ZeroPageY(pmem.read_addr(index + 1))), index + 2, 4),
        0x8E => (OpCode(I::Stx, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0x84 => (OpCode(I::Sty, A::ZeroPage(pmem.read_addr(index + 1))), index + 2, 3),
        0x94 => (OpCode(I::Sty, A::ZeroPageX(pmem.read_addr(index + 1))), index + 2, 4),
        0x8C => (OpCode(I::Sty, A::Absolute(u16::from_le_bytes([pmem.read_addr(index + 1), pmem.read_addr(index + 2)]))), index + 3, 4),

        0xAA => (OpCode(I::Tax, A::Implied), index + 1, 2),

        0xA8 => (OpCode(I::Tay, A::Implied), index + 1, 2),

        0xBA => (OpCode(I::Tsx, A::Implied), index + 1, 2),

        0x8A => (OpCode(I::Txa, A::Implied), index + 1, 2),

        0x9A => (OpCode(I::Txs, A::Implied), index + 1, 2),

        0x98 => (OpCode(I::Tya, A::Implied), index + 1, 2),

        byte => unimplemented!("opcode byte {byte:#04x}"),
    }
}
