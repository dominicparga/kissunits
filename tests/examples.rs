mod examples {
    #[test]
    fn basic() {
        #[allow(dead_code)]
        mod example {
            include!("../examples/basic.rs");

            pub fn test() {
                main();
            }
        }

        example::test();
    }
}
