#[cfg(test)]
mod test {
    use std::{fmt::Debug, fs::File};

    use glob::glob;
    use serde::Deserialize;

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

    #[test]
    pub fn test() {
        let paths = glob("../submodules/ProcessorTests/nes6502/v1/*.json").unwrap();
        for path in paths {
            let path = path.unwrap();
            let path = path.as_path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let path = path.to_str().unwrap();

            // TODO do all tests

            let f = File::open(path).unwrap();
            let result: Vec<TestCase> = serde_json::from_reader(f).unwrap();
            println!(
                "TODO file_name {:?}, path {:?} has {} test cases",
                file_name,
                path,
                result.len()
            );
        }
    }
}
