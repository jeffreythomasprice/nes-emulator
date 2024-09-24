mod cartridge_file;
mod cpu;
mod endians;
mod flags;
mod instruction_set_test_cases;
mod logging_utils;
mod memory;
mod test_utils;

use cartridge_file::Cartridge;
use log::*;
use logging_utils::logger_builder;
use std::{fs::File, io::Read};

fn main() -> anyhow::Result<()> {
    logger_builder().init();

    let mut f = File::open("/home/jeff/scratch/emulation/nes/Super Mario Bros. (Japan, USA).nes")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let cartridge = Cartridge::from_bytes(buffer)?;
    info!("memory mapper = {:?}", cartridge.header().memory_mapper());
    info!(
        "nametable arragement = {:?}",
        cartridge.header().nametable_arrangement()
    );
    info!(
        "pgr rom size = {:?}, {} bytes",
        cartridge.header().prg_rom_size(),
        cartridge.header().prg_rom_size().in_bytes()
    );
    info!(
        "chr rom size = {:?}, {} bytes",
        cartridge.header().chr_rom_size(),
        cartridge.header().chr_rom_size().in_bytes()
    );
    info!(
        "pgr ram size = {:?}, {} bytes",
        cartridge.header().prg_ram_size(),
        cartridge.header().prg_ram_size().in_bytes()
    );

    let (main_memory, video_memory) = memory::new(&cartridge);

    Ok(())
}
