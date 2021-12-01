pub mod test_support {

    pub struct TestCase<I, E> {
        pub input: I,
        pub expected: E,
    }

    pub fn run_tests<I: Copy, E: std::fmt::Debug + std::cmp::PartialEq>(
        fn_under_test: fn(input: I) -> E,
        test_cases: &[TestCase<I, E>],
    ) {
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(fn_under_test(*input), *expected);
        }
    }
}
