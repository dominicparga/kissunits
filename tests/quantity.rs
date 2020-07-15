mod quantity {
    use kissunits::quantity::Promille;

    #[test]
    fn promille_from_div() {
        let dividend = 8;
        let divisor = 2;
        let promille = Promille::from_div(dividend, divisor);
        assert_eq!(4_000, *promille);
    }

    #[test]
    fn deref_promille() {
        let raw_from = 3_000;
        let from = Promille(raw_from);
        assert_eq!(raw_from, *from);
    }

    #[test]
    fn promille_to_usize() {
        let from = Promille(3_000);
        let to: usize = from.into();
        assert_eq!(from.0 / 1_000, to);
    }

    #[test]
    fn usize_to_promille() {
        let from: usize = 3;
        let to = Promille::from(from);
        assert_eq!(from, to.0 / 1_000);
    }

    #[test]
    fn i64_to_promille() {
        let from: i64 = 3;
        let to = Promille::from(from);
        assert_eq!(from, (to.0 / 1_000) as i64);
    }

    #[test]
    fn i32_to_promille() {
        let from: i32 = 3;
        let to = Promille::from(from);
        assert_eq!(from, (to.0 / 1_000) as i32);
    }

    #[test]
    fn f64_to_promille() {
        let from: f64 = 3.0;
        let to = Promille::from(from);
        assert_eq!(from, (to.0 / 1_000) as f64);
    }

    #[test]
    fn f32_to_promille() {
        let from: f32 = 3.0;
        let to = Promille::from(from);
        assert_eq!(from, (to.0 / 1_000) as f32);
    }

    #[test]
    fn promille_add_promille() {
        let a = Promille::from(3);
        let b = Promille::from(5);
        assert_eq!(8_000, *(a + b));
    }

    #[test]
    fn promille_mul_usize() {
        let promille = Promille::from(3);
        let u = 5;
        assert_eq!(15_000, *(promille * u));
    }

    #[test]
    fn usize_mul_promille() {
        let u = 5;
        let promille = Promille::from(3);
        assert_eq!(15_000, *(u * promille));
    }

    #[test]
    fn promille_div_usize() {
        let promille = Promille::from(3);
        let u = 5;
        assert_eq!(600, *(promille / u));
    }
}
