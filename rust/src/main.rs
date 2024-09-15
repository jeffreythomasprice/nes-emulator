use std::{fs::File, io::Read};

use cartridge_file::Cartridge;

mod cartridge_file;
mod cpu;
mod endians;
mod flags;
mod instruction_set_test_cases;
mod memory;
mod test_utils;

fn main() -> anyhow::Result<()> {
    let mut f = File::open("/home/jeff/scratch/emulation/nes/Super Mario Bros. (Japan, USA).nes")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let cart = Cartridge::from_bytes(buffer)?;
    Ok(())
}
