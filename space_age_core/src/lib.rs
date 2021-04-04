//! Given an age in seconds, calculate how old someone would be on:
//! * Mercury: orbital period 0.2408467 Earth years
//! * Venus: orbital period 0.61519726 Earth years
//! * Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
//! * Mars: orbital period 1.8808158 Earth years
//! * Jupiter: orbital period 11.862615 Earth years
//! * Saturn: orbital period 29.447498 Earth years
//! * Uranus: orbital period 84.016846 Earth years
//! * Neptune: orbital period 164.79132 Earth years

use std::ops::Div;

pub use space_age_derive::Planet;

const EARTH_YEAR_IN_SECONDS: u64 = 31557600;

#[derive(Clone, Copy, Debug)]
pub struct Duration(u64);

impl Div<f64> for Duration {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0_f64 {
            panic!("Cannot divide by zero.");
        }

        self.0 as f64 / rhs
    }
}

impl Div<f64> for &Duration {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0_f64 {
            panic!("Cannot divide by zero.");
        }

        self.0 as f64 / rhs
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d / (EARTH_YEAR_IN_SECONDS as f64 * Self::ORBITAL_PERIOD)
    }
}
