#[cfg(test)]
pub mod test {
    use log::*;
    use std::fmt::Debug;

    use crate::logging_utils::logger_builder;

    pub fn init() {
        logger_builder().is_test(true).init();
    }

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
            if self.messages.len() > 0 {
                for m in self.messages.iter() {
                    error!("test failure: {}", m);
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
