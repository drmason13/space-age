use std::ops::Div;

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
