//! Given an age in seconds, calculate how old someone would be on:
//! * Mercury: orbital period 0.2408467 Earth years
//! * Venus: orbital period 0.61519726 Earth years
//! * Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
//! * Mars: orbital period 1.8808158 Earth years
//! * Jupiter: orbital period 11.862615 Earth years
//! * Saturn: orbital period 29.447498 Earth years
//! * Uranus: orbital period 84.016846 Earth years
//! * Neptune: orbital period 164.79132 Earth years

extern crate derive;
extern crate self as space_age;

mod duration;

pub use derive::Planet;
pub use duration::Duration;

const EARTH_YEAR_IN_SECONDS: u64 = 31557600;

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d / (EARTH_YEAR_IN_SECONDS as f64 * Self::ORBITAL_PERIOD)
    }
}

#[derive(Planet)]
#[orbital_period = 0.2408467]
pub struct Mercury;

#[derive(Planet)]
#[orbital_period = 0.61519726]
pub struct Venus;

#[derive(Planet)]
#[orbital_period = 1.0]
pub struct Earth;

#[derive(Planet)]
#[orbital_period = 1.8808158]
pub struct Mars;

#[derive(Planet)]
#[orbital_period = 11.862615]
pub struct Jupiter;

#[derive(Planet)]
#[orbital_period = 29.447498]
pub struct Saturn;

#[derive(Planet)]
#[orbital_period = 84.016846]
pub struct Uranus;

#[derive(Planet)]
#[orbital_period = 164.79132]
pub struct Neptune;
