pub mod accuracy {
    /// value is good because it refers to
    /// - 1 mm for km
    /// - 1 µs for s
    /// - 60 µs for min
    /// - 3.6 ms for hour
    /// - lat/lon
    ///   - lat: 1/60 ~ 0.0167 degrees equals 1_852 m
    ///     -> 1e-6 degrees equals around 0.11 m
    ///   - lon: distance depends on latitude
    ///     -> 1e-6 degrees equals <= 0.11 m (equator)
    ///   -> 1e-5 degrees points to a person in a room, see https://xkcd.com/2170/
    pub const F64_ABS: f64 = 0.000_001; // = 10^(-F64__FMT_DIGITS)
    pub const F64_FMT_DIGITS: usize = 6;
}
