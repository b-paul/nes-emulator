// parses an iNES rom file

use nom::{
    bits::bits,
    bits::complete::{bool, take},
    bytes::complete::{tag, take as take_bytes},
    error::Error,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
pub struct INesHeader {
    pub map_number: u8,
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
    four_screen: bool,
    trainer: bool,
    battery: bool,
    mirroring: Mirroring,
    vs: bool,
    prg_ram_size: u8,
    tv_system: TvSystem,
}

#[derive(Debug)]
pub struct INesRom {
    pub header: INesHeader,
    // remember that video about Arc<[u8]> you should do that
    pub trainer: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

#[derive(Debug)]
enum Mirroring {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
enum TvSystem {
    Ntsc,
    Pal,
}

fn parse_header(input: &[u8]) -> IResult<&[u8], INesHeader> {
    let (input, _) = tag(b"NES\x1a")(input)?;

    let flags = tuple((
        take::<_, u8, usize, Error<_>>(8usize), // PRG ROM size in 16KB units
        take::<_, u8, usize, Error<_>>(8usize), // CHR ROM size in 8KB units
        take::<_, u8, usize, Error<_>>(4usize), // lower nybble of mapper number
        bool,                                   // four screen VRAM layout
        bool,                                   // 512 bit trainer
        bool,                                   // battery
        bool,                                   // mirroring: 1 for vertical, 0 for horizontal
        take::<_, u8, usize, Error<_>>(4usize), // higher nybble of mapper number
        take::<_, u8, usize, Error<_>>(3usize), // if == 4, this rom is an NES2.0 rom, should be 0 otherwise I think
        bool,                                   // VS Unisystem
        take::<_, u8, usize, Error<_>>(8usize), // prg ram size, assumed 1x8kb if this is 0
        take::<_, u8, usize, Error<_>>(7usize), // reserved (used in nes 2.0!)
        bool,                                   // TV system (0 = NTSC, 1 = PAL)
        take::<_, u64, usize, Error<_>>(48usize), // padding
    ));
    bits::<_, _, Error<_>, _, _>(flags)(input).map(
        |(
            input,
            (
                prg_rom_size,
                chr_rom_size,
                map0,
                four_screen,
                trainer,
                battery,
                mirroring,
                map1,
                _nes2,
                vs,
                mut prg_ram_size,
                _,
                tv_system,
                _,
            ),
        )| {
            let map_number = map1 << 4 | map0;
            prg_ram_size = prg_ram_size.min(1);

            let prg_rom_size = prg_rom_size as usize * 16384;
            let chr_rom_size = chr_rom_size as usize * 8192;

            let mirroring = match mirroring {
                false => Mirroring::Horizontal,
                true => Mirroring::Vertical,
            };

            let tv_system = match tv_system {
                false => TvSystem::Ntsc,
                true => TvSystem::Pal,
            };

            (
                input,
                INesHeader {
                    map_number,
                    prg_rom_size,
                    chr_rom_size,
                    four_screen,
                    trainer,
                    battery,
                    mirroring,
                    vs,
                    prg_ram_size,
                    tv_system,
                },
            )
        },
    )
}

pub fn parse_rom(input: &[u8]) -> IResult<&[u8], INesRom> {
    let (input, header) = parse_header(input)?;
    let (input, trainer) = match header.trainer {
        true => take_bytes(512usize)(input).map(|(input, s)| (input, s.to_vec())),
        false => Ok((input, vec![])),
    }?;
    let (input, prg_rom) =
        take_bytes(header.prg_rom_size)(input).map(|(input, s)| (input, s.to_vec()))?;
    let (input, chr_rom) =
        take_bytes(header.chr_rom_size)(input).map(|(input, s)| (input, s.to_vec()))?;
    // something about playchoice idk

    Ok((
        input,
        INesRom {
            header,
            trainer,
            prg_rom,
            chr_rom,
        },
    ))
}
