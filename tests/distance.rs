mod helpers;
use helpers::units::{approx_eq, DISTANCE, DURATION, SPEED};

mod distance {
    use kissunits::{
        distance::{Kilometers, Meters},
        speed::{KilometersPerHour, MetersPerSecond},
        time::{Hours, Seconds},
    };

    #[test]
    fn m_to_km() {
        let from = Meters(1_000.0);
        let to = Kilometers::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 1_000.0;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn km_to_m() {
        let from = Kilometers(1_000.0);
        let to = Meters::from(from);
        let raw_from = from.0;
        let raw_to = to.0;
        let scale = 0.001;
        assert!(super::approx_eq(raw_from, scale * raw_to));
    }

    #[test]
    fn kmph_mul_h() {
        let kmph = KilometersPerHour::from(super::SPEED);
        let h = Hours::from(super::DURATION);

        let km = Kilometers::from(super::DISTANCE);
        let result = kmph * h;

        assert!(
            super::approx_eq(*result, *km),
            format!("{} != {} = {} * {}", result, km, kmph, h)
        );
    }

    #[test]
    fn h_mul_kmph() {
        let h = Hours::from(super::DURATION);
        let kmph = KilometersPerHour::from(super::SPEED);

        let km = Kilometers::from(super::DISTANCE);
        let result = h * kmph;

        assert!(
            super::approx_eq(*result, *km),
            format!("{} != {} = {} * {}", result, km, h, kmph)
        );
    }

    #[test]
    fn mps_mul_s() {
        let mps = MetersPerSecond::from(super::SPEED);
        let s = Seconds::from(super::DURATION);

        let m = Meters::from(super::DISTANCE);
        let result = mps * s;

        assert!(
            super::approx_eq(*result, *m),
            format!("{} != {} = {} * {}", result, m, mps, s)
        );
    }

    #[test]
    fn s_mul_mps() {
        let s = Seconds::from(super::DURATION);
        let mps = MetersPerSecond::from(super::SPEED);

        let m = Meters::from(super::DISTANCE);
        let result = s * mps;

        assert!(
            super::approx_eq(*result, *m),
            format!("{} != {} = {} * {}", result, m, s, mps)
        );
    }
}
