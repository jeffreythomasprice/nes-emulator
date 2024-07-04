#[cfg(test)]
mod test {
    use std::{fmt::Debug, fs::File, io::Read};

    use glob::glob;
    use serde::Deserialize;

    use crate::{cpu::CPU, flags::Flags, memory::Memory, test_utils::test::TestResults};

    #[derive(Debug, Deserialize)]
    struct TestCase {
        name: String,
        initial: State,
        #[serde(rename = "final")]
        expected: State,
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
        ram: Vec<RAM>,
    }

    #[derive(Debug, Deserialize)]
    struct RAM {
        address: u16,
        value: u8,
    }

    #[derive(Debug, Deserialize)]
    struct Cycle {
        address: u16,
        value: u8,
        mode: CycleMode,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "lowercase")]
    enum CycleMode {
        Read,
        Write,
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
        fn read8(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn write8(&mut self, address: u16, value: u8) {
            self.data[address as usize] = value
        }
    }

    #[test]
    pub fn test() {
        let paths = glob("../submodules/ProcessorTests/nes6502/v1/*.json").unwrap();
        let mut test_results = TestResults::new();
        for path in paths {
            let path = path.unwrap();
            let path = path.as_path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let path = path.to_str().unwrap();

            // TODO do all tests
            let instruction = u8::from_str_radix(&file_name[0..2], 16).unwrap();
            if instruction != 0x93 {
                continue;
            }

            let mut f = File::open(path).unwrap();
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();
            let test_cases: Vec<TestCase> = serde_json::from_str(&s).unwrap();
            println!(
                "path {:?}, instruction {:02x}, has {} test cases",
                path,
                instruction,
                test_cases.len()
            );

            for test_case in test_cases {
                let mut c = CPU::new();
                c.pc = test_case.initial.pc;
                c.sp = test_case.initial.sp;
                c.a = test_case.initial.a;
                c.x = test_case.initial.x;
                c.y = test_case.initial.y;
                c.flags = Flags::from_bits_truncate(test_case.initial.flags);

                let mut m = TestMemory::new();
                for x in test_case.initial.ram {
                    m.write8(x.address, x.value);
                }

                c.step(&mut m);

                test_results.eq(&test_case.expected.pc, &c.pc, "pc".to_string());
                test_results.eq(&test_case.expected.sp, &c.sp, "sp".to_string());
                test_results.eq(&test_case.expected.a, &c.a, "a".to_string());
                test_results.eq(&test_case.expected.x, &c.x, "x".to_string());
                test_results.eq(&test_case.expected.y, &c.y, "y".to_string());
                test_results.eq(
                    &Flags::from_bits_truncate(test_case.expected.flags),
                    &c.flags,
                    "flags".to_string(),
                );

                for x in test_case.expected.ram {
                    test_results.eq(
                        &x.value,
                        &m.read8(x.address),
                        format!("memory at {:04x}", x.address),
                    );
                }

                test_results.eq(
                    &(test_case.cycles.len() as u64),
                    &c.clock,
                    "clock".to_string(),
                );
                // TODO verify per-cycle memory access
            }
        }
        test_results.assert();
    }
}
