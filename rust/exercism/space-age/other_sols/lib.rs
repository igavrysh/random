// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

const SECS_IN_YEAR_ON_EARTH: u64 = 31557600;

pub trait Planet {
    fn orb_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64  / (SECS_IN_YEAR_ON_EARTH as f64 * Self::orb_period())
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet {
    (for $t:ty, $e:expr) => {
        impl Planet for $t {
            fn orb_period() -> f64 {
                $e
            }
        }
    }
}

impl_planet!(for Mercury, 0.2408467);
impl_planet!(for Venus, 0.61519726);
impl_planet!(for Earth, 1.0);
impl_planet!(for Mars, 1.8808158);
impl_planet!(for Jupiter, 11.862615);
impl_planet!(for Saturn, 29.447498);
impl_planet!(for Uranus, 84.016846);
impl_planet!(for Neptune, 164.79132);
