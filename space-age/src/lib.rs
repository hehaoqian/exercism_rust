#[derive(Debug, Clone)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

impl From<Duration> for u64 {
    fn from(d: Duration) -> u64 {
        d.seconds
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! PlanetWithPeriod {
    ($planet:ident,$years:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                const SECONDS_IN_YEARS: f64 = 31557600.0;
                let s: u64 = d.clone().into();
                s as f64 / ($years as f64) / SECONDS_IN_YEARS
            }
        }
    };
}

PlanetWithPeriod!(Mercury, 0.2408467);
PlanetWithPeriod!(Venus, 0.61519726);
PlanetWithPeriod!(Earth, 1.0);
PlanetWithPeriod!(Mars, 1.8808158);
PlanetWithPeriod!(Jupiter, 11.862615);
PlanetWithPeriod!(Saturn, 29.447498);
PlanetWithPeriod!(Uranus, 84.016846);
PlanetWithPeriod!(Neptune, 164.79132);
