#[allow(dead_code)]

use anyhow::{anyhow, Result};
use std::ops::{Add, Sub};

#[derive(Clone)]
pub struct Percent {
    percent: u32,
}

impl Percent {
    pub fn new(percent: u32) -> Result<Percent> {
        if percent > 100 {
            return Err(anyhow!("percent must be 0-100; {}", percent));
        }
        Ok(Percent { percent })
    }
}

impl Add for Percent {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut percent = self.percent + other.percent;
        if percent > 100 {
            percent = 100;
        }
        Self { percent }
    }
}

impl Sub for Percent {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.percent > other.percent {
            Self {
                percent: self.percent - other.percent,
            }
        } else {
            Self { percent: 0 }
        }
    }
}
