pub fn square<A>(x: i32, logger: &A) -> i32
where
    A: Logger,
{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub struct PrintlnLogger;

impl PrintlnLogger {
    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub trait Logger {
    fn log(&self, msg: &str);
}

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

pub struct TestLogger;

impl Logger for TestLogger {
    fn log(&self, _msg: &str) {}
}

#[cfg(test)]
mod tests {
    use crate::TestLogger;

    use super::square;
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, &TestLogger), 4);
    }
}
