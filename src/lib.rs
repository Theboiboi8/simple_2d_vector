#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A grid-based two-dimensional representation of a mathematical vector.
pub struct Vector2D {
    /// Defines the origin point of the vector on a grid centered around (0, 0).
    origin: (f64, f64),
    /// The end of the vector in relation to its origin.
    target: (f64, f64)
}

impl Vector2D {
    /// A zero [`Vector2D`] with both `origin` and `target` at (0, 0).
    pub fn zero() -> Self {
        Vector2D {
            origin: (0.0, 0.0),
            target: (0.0, 0.0),
        }
    }

    /// A [`Vector2D`] with a `target` of (0, 0).
    ///
    /// Also known as a zero-length vector
    pub fn null(origin: (f64, f64)) -> Self {
        Vector2D {
            origin,
            target: (0.0, 0.0),
        }
    }

    /// Constructs a [`Vector2D`] from the provided `origin` and `target`.
    pub fn new(origin: (f64, f64), target: (f64, f64)) -> Self {
        Vector2D {
            origin,
            target,
        }
    }

    /// Constructs a [`Vector2D`] from the provided `origin` and `target`.
    ///
    /// The difference between this and [`Vector2D::new()`] is that this
    /// treats `target` as an absolute value,
    /// meaning that `Vector2D::with_absolute_target((1.0,0.0),(2.0,1.0))`
    /// constructs a vector with an `origin` of (1.0, 0.0) but a `target`
    /// of (1.0, 1.0) as opposed to a `target` of (2.0, 1.0).
    pub fn with_absolute_target(origin: (f64, f64), target: (f64, f64)) -> Self {
        Vector2D {
            origin,
            target: (target.0 - origin.0, target.1 - origin.1),
        }
    }

    /// Sets the `origin` of [`self`]
    pub fn set_origin(mut self, origin: (f64, f64)) -> Self {
        self.origin = origin;
        self
    }

    /// Sets the `target` of [`self`]
    pub fn set_target(mut self, target: (f64, f64)) -> Self {
        self.target = target;
        self
    }

    /// Sets the `target` of [`self`].
    ///
    /// Uses an absolute `target` instead of a relative one.
    ///
    /// Due to issues with imprecise float arithmetic operations,
    /// only the first five digits of the input `target` are kept,
    /// and anything past that is rounded.
    /// 
    /// So far, I have only encountered this issue with this function,
    /// but if you encounter any others during use, please report them in the issue tracker.
    pub fn set_target_absolute(mut self, target: (f64, f64)) -> Self {
        self.target = (
            ((target.0 - self.origin.0) * 10000.0).round() / 10000.0, // Rounding as a workaround for errors in insignificant bits
            ((target.1 - self.origin.1) * 10000.0).round() / 10000.0, // Rounding as a workaround for errors in insignificant bits
        );
        self
    }

    /// Shifts the left-hand-side [`Vector2D`] by the right-hand-side tuple of types
    /// which implement [`Into<f32>`].
    ///
    /// The resulting [`Vector2D`] has the same `target` value, as it's relative to its `origin`,
    /// but a shifted `origin`, which results in an overall shift of the [`Vector2D`].
    pub fn shift(mut self, shift: (impl Into<f64>, impl Into<f64>)) -> Self {
        self.origin = (self.origin.0 + shift.0.into(), self.origin.1 + shift.1.into());
        self
    }

    /// Returns the magnitude of the given [`Vector2D`].
    pub fn get_magnitude(&self) -> f64 {
        f64::sqrt((self.origin.0 - self.target.0).powf(2.0) + (self.origin.1 - self.target.1).powf(2.0))
    }
    
    /// Returns the dot product of the given [`Vector2D`]s.
    pub fn dot_product(&self, vector: Vector2D) -> f64 {
        self.target.0 * vector.target.0 + self.target.1 * vector.target.1
    }

    pub fn get_angle_between(&self, vector: Vector2D) -> f64 {
        f64::acos(self.dot_product(vector) / (self.get_magnitude() * vector.get_magnitude()))
    }
}

impl Display for Vector2D {
    /// Formats the [`Vector2D`] with the given formatter and prepares it for user-facing output.
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