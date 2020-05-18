mod helpers;
use helpers::units::{approx_eq, DISTANCE, DURATION, SPEED};

mod time {
    use kissunits::{
        distance::{Kilometers, Meters},
        speed::{KilometersPerHour, MetersPerSecond},
        time::{Hours, Minutes, Seconds},
    };

    #[test]
    fn s_to_f64() {
        let from = Seconds(60.0);
        let to = Seconds(120.0) / from;
        let raw_from = from.0;
        let raw_to = to;
        let scale = 30.0;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn min_to_f64() {
        let from = Minutes(1.0);
        let to = Minutes(2.0) / from;
        let raw_from = from.0;
        let raw_to = to;
        let scale = 0.5;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn h_to_f64() {
        let from = Hours(0.4);
        let to = Hours(1.6) / from;
        let raw_from = from.0;
        let raw_to = to;
        let scale = 0.1;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn s_to_min() {
        let from = Seconds(1_000.0);
        let to = Minutes::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 1.0 / 60.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn s_to_h() {
        let from = Seconds(1_000.0);
        let to = Hours::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 1.0 / 3_600.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn min_to_s() {
        let from = Minutes(1_000.0);
        let to = Seconds::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 60.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn min_to_h() {
        let from = Minutes(1_000.0);
        let to = Hours::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 1.0 / 60.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn h_to_s() {
        let from = Hours(1_000.0);
        let to = Seconds::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 3_600.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn h_to_min() {
        let from = Hours(1_000.0);
        let to = Minutes::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 60.0;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn km_div_kmph() {
        let km = Kilometers::from(super::DISTANCE);
        let kmph = KilometersPerHour::from(super::SPEED);

        let h = Hours::from(super::DURATION);
        let result = km / kmph;

        assert!(
            super::approx_eq(*result, *h),
            format!("{} != {} = {} / {}", result, h, km, kmph)
        );
    }

    #[test]
    fn m_div_mps() {
        let m = Meters::from(super::DISTANCE);
        let mps = MetersPerSecond::from(super::SPEED);

        let s = Seconds::from(super::DURATION);
        let result = m / mps;

        assert!(
            super::approx_eq(*result, *s),
            format!("{} != {} = {} / {}", result, s, m, mps)
        );
    }
}
