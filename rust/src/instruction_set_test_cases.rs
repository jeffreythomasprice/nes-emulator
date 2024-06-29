#[cfg(test)]
mod test {
    use std::fmt::Debug;

    type TestResult = Result<(), String>;

    struct Test<'a> {
        test_name: &'a str,
    }

    impl<'a> Test<'a> {
        pub fn new(test_name: &'a str) -> Self {
            Self { test_name }
        }

        pub fn expect<T>(&self, left: &T, right: &T) -> TestResult
        where
            T: Eq + Debug,
        {
            if left == right {
                Ok(())
            } else {
                Err(format!("{}: {:?} != {:?}", self.test_name, left, right))
            }
        }
    }

    fn run<F>(test_name: &str, f: F)
    where
        F: Fn(&Test) -> TestResult,
    {
        if let Err(e) = f(&Test::new(test_name)) {
            assert!(false, "{}", e);
        }
    }

    #[test]
    pub fn test() {
        run("foo", |t| {
            t.expect(&1, &2)?;
            Ok(())
        });
        run("bar", |t| {
            t.expect(&1, &2)?;
            Ok(())
        });
    }
}
