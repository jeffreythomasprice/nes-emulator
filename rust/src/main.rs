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

trait Memory {
    fn read_u8(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;
    fn write_u8(&mut self, address: u16, value: u8);
    fn write_u16(&mut self, address: u16, value: u16);
}

struct CPU {
    pc: u16,
    sp: u8,
    a: u8,
    x: u8,
    y: u8,
    flags: Flags,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            flags: Flags::from(0),
        }
    }

    pub fn step(&mut self) {
        // TODO implement stuff
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
    use std::{
        fmt::Debug,
        fs::{self, File},
    };

    use glob::glob;
    use serde::{Deserialize, Deserializer};

    use crate::{Flags, Memory, CPU};

    struct TestResults {
        total: u32,
        failures: Vec<String>,
    }

    impl TestResults {
        pub fn new() -> Self {
            Self {
                total: 0,
                failures: Vec::new(),
            }
        }

        pub fn assert(&self) {
            if self.failures.len() > 0 {
                for s in self.failures.iter() {
                    assert!(false, "{s}");
                }
            }
        }

        pub fn test<F>(&mut self, f: F)
        where
            F: Fn() -> Result<(), String>,
        {
            if let Err(e) = f() {
                self.failures.push(e);
            }
            self.total += 1;
        }

        pub fn eq<T>(&mut self, expected: T, actual: T)
        where
            T: Eq + Debug,
        {
            self.test(|| {
                if expected != actual {
                    Err(format!("expected {expected:?} != actual {actual:?}"))
                } else {
                    Ok(())
                }
            })
        }

        pub fn eq_s<T>(&mut self, expected: T, actual: T, message: &str)
        where
            T: Eq + Debug,
        {
            self.test(|| {
                if expected != actual {
                    Err(format!(
                        "{message}, expected {expected:?} != actual {actual:?}"
                    ))
                } else {
                    Ok(())
                }
            })
        }
    }

    struct TestMemory {
        data: [u8; 0x10000],
    }

    impl TestMemory {
        pub fn new() -> Self {
            Self { data: [0; 0x10000] }
        }
    }

    impl Memory for TestMemory {
        fn read_u8(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn read_u16(&self, address: u16) -> u16 {
            let low = self.data[address as usize];
            let high = self.data[(address + 1) as usize];
            (low as u16) | ((high as u16) << 8)
        }

        fn write_u8(&mut self, address: u16, value: u8) {
            self.data[address as usize] = value;
        }

        fn write_u16(&mut self, address: u16, value: u16) {
            let low = (value & 0xff) as u8;
            let high = ((value & 0xff00) >> 8) as u8;
            self.write_u8(address, low);
            self.write_u8(address + 1, high);
        }
    }

    #[derive(Debug, Deserialize)]
    struct TestCase {
        name: String,
        #[serde(rename = "initial")]
        before: State,
        #[serde(rename = "final")]
        after: State,
        cycles: Vec<Cycle>,
    }

    #[derive(Debug, Deserialize)]
    struct State {
        pc: u16,
        #[serde(rename = "s")]
        sp: u8,
        a: u8,
        x: u8,
        y: u8,
        #[serde(rename = "p")]
        flags: u8,
        ram: Vec<RAMState>,
    }

    #[derive(Debug, Deserialize)]
    struct RAMState {
        address: u16,
        value: u8,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum CycleMode {
        Read,
        Write,
    }

    #[derive(Debug, Deserialize)]
    struct Cycle {
        addres: u16,
        value: u8,
        mode: CycleMode,
    }

    impl TestCase {
        fn perform_test(&self, results: &mut TestResults) {
            let mut cpu = CPU::new();
            cpu.pc = self.before.pc;
            cpu.sp = self.before.sp;
            cpu.a = self.before.a;
            cpu.x = self.before.x;
            cpu.y = self.before.y;
            cpu.flags = self.before.flags.into();

            let mut memory = TestMemory::new();
            for mem_init in self.before.ram.iter() {
                memory.write_u8(mem_init.address, mem_init.value);
            }

            cpu.step();

            // TODO don't assert, collect pass and fail and report on all at once at the end?
            results.eq_s(self.after.pc, cpu.pc, "pc");
            results.eq_s(self.after.sp, cpu.sp, "sp");
            results.eq_s(self.after.a, cpu.a, "a");
            results.eq_s(self.after.x, cpu.x, "x");
            results.eq_s(self.after.y, cpu.y, "y");
            let after_flags: Flags = self.after.flags.into();
            results.eq_s(after_flags.carry(), cpu.flags.carry(), "carry");
            results.eq_s(after_flags.zero(), cpu.flags.zero(), "zero");
            results.eq_s(
                after_flags.interrupt_disable(),
                cpu.flags.interrupt_disable(),
                "interrupt_disable",
            );
            results.eq_s(
                after_flags.decimal_mode(),
                cpu.flags.decimal_mode(),
                "decimal_mode",
            );
            results.eq_s(
                after_flags.break_command(),
                cpu.flags.break_command(),
                "break_command",
            );
            results.eq_s(after_flags.unused(), cpu.flags.unused(), "unused");
            results.eq_s(after_flags.overflow(), cpu.flags.overflow(), "overflow");
            results.eq_s(after_flags.negative(), cpu.flags.negative(), "negative");

            for after_mem in self.after.ram.iter() {
                results.eq_s(after_mem.value, memory.read_u8(after_mem.address), "memory");
            }

            // TODO check that clock ticks match number of cycles
        }
    }

    #[test]
    fn json_tests() {
        for path in glob("../submodules/ProcessorTests/nes6502/v1/*.json")
            .expect("failed to get test files")
        {
            let path = path.expect("failed to get path");
            let file_name = path
                .file_name()
                .expect("failed to get file name")
                .to_str()
                .expect("failed to get real string out of file name");

            let mut results = TestResults::new();

            // TODO do all tests
            if file_name == "00.json" {
                let test_cases: Vec<TestCase> =
                    serde_json::from_reader(File::open(path.clone()).expect("failed to open file"))
                        .expect("failed to deserialize path");
                for test_case in test_cases.iter() {
                    test_case.perform_test(&mut results);
                }
            }

            results.assert();
        }
    }
}
