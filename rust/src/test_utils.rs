#[cfg(test)]
pub mod test {
    use std::fmt::Debug;

    pub struct TestResults {
        messages: Vec<String>,
    }

    impl TestResults {
        pub fn new() -> Self {
            Self {
                messages: Vec::new(),
            }
        }

        pub fn assert(&self) {
            // TODO if any messages, fail and print them
            if self.messages.len() > 0 {
                for m in self.messages.iter() {
                    println!("test failure: {}", m);
                }
                assert!(false, "at least one test case failed")
            }
        }

        pub fn eq<T>(&mut self, expected: &T, actual: &T, message: String)
        where
            T: Eq + Debug,
        {
            if expected != actual {
                self.messages.push(format!(
                    "{:?} (expected) != {:?} (actual), {}",
                    expected, actual, message
                ));
            }
        }
    }
}
