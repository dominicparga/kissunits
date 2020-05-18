mod helpers;
use helpers::units::{approx_eq, DISTANCE, DURATION, SPEED};

mod speed {
    use kissunits::{
        distance::{Kilometers, Meters},
        speed::{KilometersPerHour, MetersPerSecond},
        time::{Hours, Seconds},
    };

    #[test]
    fn mps_to_f64() {
        let from = MetersPerSecond(50.0);
        let to = MetersPerSecond(100.0) / from;
        let raw_from = from.0;
        let raw_to = to;
        let scale = 25.0;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn kmph_to_f64() {
        let from = KilometersPerHour(50.0);
        let to = KilometersPerHour(100.0) / from;
        let raw_from = from.0;
        let raw_to = to;
        let scale = 25.0;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn mps_to_kmph() {
        let from = MetersPerSecond(50.0);
        let to = KilometersPerHour::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 3.6;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn kmph_to_mps() {
        let from = KilometersPerHour(180.0);
        let to = MetersPerSecond::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 1.0 / 3.6;
        assert!(super::approx_eq(scale * raw_from, raw_to));
    }

    #[test]
    fn m_div_s() {
        let m = Meters::from(super::DISTANCE);
        let s = Seconds::from(super::DURATION);

        let mps = MetersPerSecond::from(super::SPEED);
        let result = m / s;

        assert!(
            super::approx_eq(*result, *mps),
            format!("{} != {} = {} / {}", result, mps, m, s)
        );
    }

    #[test]
    fn km_div_h() {
        let km = Kilometers::from(super::DISTANCE);
        let h = Hours::from(super::DURATION);

        let kmph = KilometersPerHour::from(super::SPEED);
        let result = km / h;

        assert!(
            super::approx_eq(*result, *kmph),
            format!("{} != {} = {} / {}", result, kmph, km, h)
        );
    }
}
