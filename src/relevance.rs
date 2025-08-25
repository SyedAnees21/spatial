use std::ops::Deref;

pub struct Relevance(f64);

impl Relevance {
    pub fn new<T: Into<f64>>(value: T) -> Self {
        Self(Relevance::clamp(value.into()))
    }

    pub fn max() -> Self {
        Self(1.0)
    }

    pub fn min() -> Self {
        Self(0.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn clamp(value: f64) -> f64 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }
}

impl Deref for Relevance {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}