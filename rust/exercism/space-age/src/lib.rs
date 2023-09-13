// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug)]
pub struct Duration {
    pub value: f64,
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { value: s as f64 }
    }
}
pub trait Planet {
    const EARTH_YEAR_SECONDS: f64 = 31557600.0;
    const EARTH_YEAR_RATIO: f64;
    fn years_during(d: &Duration) -> f64 {
        d.value / Self::EARTH_YEAR_SECONDS / Self::EARTH_YEAR_RATIO
    }
}
macro_rules! boilerplate {
    ($($t:ident => $e:expr),+) => {
        $(
            pub struct $t {}
            impl Planet for $t {
                const EARTH_YEAR_RATIO: f64 = $e;
            }
        )*
    }
}
boilerplate!(
    Mercury => 0.2408467,
    Venus => 0.61519726,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.7913
);