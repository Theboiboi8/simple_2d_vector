#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A grid-based two-dimensional representation of a mathematical vector.
pub struct Vector2D {
    /// Defines the origin point of the vector on a grid centered around (0,0).
    origin: (f32, f32),
    /// The end of the vector in relation to its origin.
    target: (f32, f32)
}

impl Vector2D {
    /// A null [`Vector2D`] with both `origin` and `target` at (0,0).
    pub fn null() -> Self {
        Vector2D {
            origin: (0.0, 0.0),
            target: (0.0, 0.0),
        }
    }

    /// Constructs a [`Vector2D`] from the provided `origin` and `target`.
    pub fn new(origin: (f32, f32), target: (f32, f32)) -> Self {
        Vector2D {
            origin,
            target,
        }
    }

    /// Sets the `origin` of [`self`]
    pub fn set_origin(mut self, origin: (f32, f32)) -> Self {
        self.origin = origin;
        self
    }

    /// Sets the `target` of [`self`]
    pub fn set_target(mut self, target: (f32, f32)) -> Self {
        self.target = target;
        self
    }

    /// Shifts the left-hand-side [`Vector2D`] by the right-hand-side tuple of types
    /// which implement [`Into<f32>`].
    ///
    /// The resulting [`Vector2D`] has the same `target` value, as it's relative to its `origin`,
    /// but a shifted `origin`, which results in an overall shift of the [`Vector2D`].
    pub fn shift(mut self, shift: (impl Into<f32>, impl Into<f32>)) -> Self {
        self.origin = (self.origin.0 + shift.0.into(), self.origin.1 + shift.1.into());
        self
    }
}

impl Display for Vector2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{})[{},{}]",
            self.origin.0,
            self.origin.1,
            self.target.0,
            self.target.1,
        )
    }
}

impl Add for Vector2D {
    type Output = Self;

    /// Performs a mathematical addition of two [`Vector2D`]s.
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            origin: self.origin,
            target: (rhs.target.0 + self.target.0, rhs.target.1 + self.target.1),
        }
    }
}


impl Sub for Vector2D {
    type Output = Self;

    /// Performs a mathematical subtraction of two [`Vector2D`]s.
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            origin: self.origin,
            target: (self.target.0 - rhs.target.0, self.target.1 - rhs.target.1),
        }
    }
}