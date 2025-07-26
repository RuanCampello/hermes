//! `Time` module contains a little implementation of relative and absolute time.

use core::{fmt, ops};

/// Represents an absolute time value.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    micros: i64,
}

impl Instant {
    const ZERO: Instant = Instant::from_micros_const(0);

    pub fn from_micros<M: Into<i64>>(micros: M) -> Self {
        Self {
            micros: micros.into(),
        }
    }

    pub fn from_millis<M: Into<i64>>(millis: M) -> Self {
        Self {
            micros: millis.into() * 1000,
        }
    }

    pub fn from_secs<S: Into<i64>>(secs: S) -> Self {
        Self {
            micros: secs.into() * 1000000,
        }
    }

    const fn from_micros_const(micros: i64) -> Self {
        Self { micros }
    }
}
