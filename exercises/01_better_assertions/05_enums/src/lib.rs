#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{anything, eq, pat};

    #[derive(Debug)]
    enum MyCustomEnum {
        A,
        B(u32),
        C { a: &'static str },
    }

    #[googletest::test]
    fn failed_is_b() {
        let x = MyCustomEnum::A;
        // This will become `assert_matches!` once it stabilises!
        assert_that!(x, pat!(MyCustomEnum::B(anything())));
    }

    #[googletest::test]
    fn failed_is_c() {
        let x = MyCustomEnum::B(10);
        assert_that!(x, pat!(MyCustomEnum::C { a: anything() }));
    }

    #[googletest::test]
    fn failed_is_c_with_value() {
        let x = MyCustomEnum::B(10);
        assert_that!(x, pat!(MyCustomEnum::C { a: eq("hello") }));
    }
}
