use std::fmt::Display;

use bilge::prelude::*;

#[bitsize(8)]
#[derive(FromBits, Clone, Copy)]
struct Flags {
    carry: bool,
    zero: bool,
    interrupt_disable: bool,
    decimal_mode: bool,
    break_command: bool,
    unused: bool,
    overflow: bool,
    negative: bool,
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.value)
    }
}

fn main() {
    let mut f = Flags::from(0);
    println!("f = {}", f);
    f.set_decimal_mode(true);
    println!("f = {}", f);
    let mut g = f;
    g.set_negative(true);
    println!("f = {}", f);
    println!("g = {}", g);
}

#[cfg(test)]
mod test {
    use glob::glob;

    #[test]
    fn json_tests() {
        for path in glob("../submodules/ProcessorTests/nes6502/v1/*.json")
            .expect("failed to get test files")
        {
            let path = path.expect("failed to get path");
            let file_name = path.file_name().expect("failed to get file name");
            println!("path = {:?}", file_name);
            if path.to_str() == Some("") {}
        }
    }
}
