#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A grid-based two-dimensional representation of a mathematical vector.
pub struct Vector2D {
    /// Defines the origin point of the vector.
    pub origin: Point2D,
    /// The end of the vector in relation to its origin.
    ///
    /// While this is a [`Point2D`] type,
    /// much like `origin`, it represents a relative value
    /// rather than an absolute one.
    pub target: Point2D
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[must_use]
/// A structure that represents a point in two-dimensional space.
/// 
/// The grid is centered around (0, 0).
pub struct Point2D {
    /// The x coordinate of the [`Point2D`]
    pub x: f64,
    /// The y coordinate of the [`Point2D`]
    pub y: f64,
}

impl Point2D {
    /// Constructs a new [`Point2D`] at coordinates (0, 0).
    pub fn zero() -> Self {
        Point2D {
            x: 0.0,
            y: 0.0,
        }
    }

    /// Constructs a new [`Point2D`] at the given coordinates.
    pub fn new(x: f64, y: f64) -> Self {
        Point2D {
            x,
            y,
        }
    }

    /// Sets the `x` coordinate of [`self`]
    pub fn set_x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    /// Sets the `y` coordinate of [`self`]
    pub fn set_y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    /// Shifts the left-hand-side [`Point2D`] by the right-hand-side tuple of types
    /// which implement [`Into<f64>`].
    pub fn shift(mut self, shift: (impl Into<f64>, impl Into<f64>)) -> Self {
        self.x += shift.0.into();
        self.y += shift.1.into();
        self
    }

    /// Returns the distance between [`self`] and the given [`Point2D`].
    pub fn get_distance(&self, rhs: Point2D) -> f64 {
        f64::sqrt(
            (self.x - rhs.x).powf(2.0)
                +
                (self.y - rhs.y).powf(2.0)
        )
    }
    
    /// Constructs a new [`Vector2D`] with an `origin` [`Point2D`]([`self`]) and an absolute
    /// `target` [`Point2D`]
    pub fn to_vector2d(&self, target: &Point2D) -> Vector2D {
        Vector2D {
            origin: *self,
            target: *target
        }
    }
}

impl Vector2D {
    /// A zero [`Vector2D`] with both `origin` and `target` at (0, 0).
    pub fn zero() -> Self {
        Vector2D {
            origin: Point2D { x: 0.0, y: 0.0},
            target: Point2D { x: 0.0, y: 0.0},
        }
    }

    /// A [`Vector2D`] with a `target` of (0, 0).
    ///
    /// Also known as a zero-length vector
    pub fn null(origin: Point2D) -> Self {
        Vector2D {
            origin,
            target: Point2D { x: 0.0, y: 0.0},
        }
    }

    /// Constructs a [`Vector2D`] from the provided `origin` and `target`.
    pub fn new(origin: Point2D, target: Point2D) -> Self {
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
    pub fn with_absolute_target(origin: Point2D, target: Point2D) -> Self {
        Vector2D {
            origin,
            target: Point2D {
                x: target.x - origin.x,
                y: target.y - origin.y
            },
        }
    }

    /// Sets the `origin` of [`self`]
    pub fn set_origin(mut self, origin: Point2D) -> Self {
        self.origin = origin;
        self
    }

    /// Sets the `target` of [`self`]
    pub fn set_target(mut self, target: Point2D) -> Self {
        self.target = target;
        self
    }

    /// Sets the `target` of [`self`].
    ///
    /// Uses an absolute `target` instead of a relative one.
    ///
    /// Due to issues with imprecise float arithmetic operations,
    /// only the first six digits of the input `target` are kept,
    /// and anything past that is rounded.
    /// 
    /// So far, I have only encountered this issue with this function,
    /// but if you encounter any others during use, please report them in the issue tracker.
    pub fn set_target_absolute(mut self, target: Point2D) -> Self {
        self.target = Point2D {
            x: ((target.x - self.origin.x) * 1_000_000.0).round() / 1_000_000.0,
            y: ((target.y - self.origin.y) * 1_000_000.0).round() / 1_000_000.0,
        };
        self
    }

    /// Shifts the left-hand-side [`Vector2D`] by the right-hand-side tuple of types
    /// which implement [`Into<f64>`].
    ///
    /// The resulting [`Vector2D`] has the same `target` value, as it's relative to its `origin`,
    /// but a shifted `origin`, which results in an overall shift of the [`Vector2D`].
    pub fn shift(mut self, shift: (impl Into<f64>, impl Into<f64>)) -> Self {
        self.origin = Point2D {
            x: self.origin.x + shift.0.into(),
            y: self.origin.y + shift.1.into()
        };
        self
    }

    /// Returns the magnitude of the given [`Vector2D`].
    pub fn get_magnitude(&self) -> f64 {
        f64::sqrt(
            (self.origin.x - self.target.x).powf(2.0)
                +
                (self.origin.y - self.target.y).powf(2.0)
        )
    }
    
    /// Returns the dot product of the given [`Vector2D`]s.
    pub fn dot_product(&self, vector: Vector2D) -> f64 {
        self.target.x * vector.target.x + self.target.y * vector.target.y
    }

    /// Returns the angle between the given [`Vector2D`]s in radians.
    pub fn get_angle_between(&self, vector: Vector2D) -> f64 {
        f64::acos(self.dot_product(vector) / (self.get_magnitude() * vector.get_magnitude()))
    }
}

impl Display for Vector2D {
    /// Formats the [`Vector2D`] with the given formatter and prepares it for user-facing output.
    /// 
    /// The format is `(origin.0,origin.1)[target.0,target.1]`.
    /// 
    /// Example:
    /// ```rust
    /// use simple_2d_vector::{Vector2D, Point2D};
    /// 
    /// let vector = Vector2D::new(
    ///     Point2D { x: 2.2, y: 1.1 },
    ///     Point2D { x: 1.1, y: 2.2 },
    /// );
    /// 
    /// assert_eq!(format!("{vector}"), "(2.2,1.1)[1.1,2.2]");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{})[{},{}]",
            self.origin.x,
            self.origin.y,
            self.target.x,
            self.target.y,
        )
    }
}

impl Add for Vector2D {
    type Output = Self;

    /// Performs a mathematical addition of two [`Vector2D`]s.
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            origin: self.origin,
            target: Point2D {
                x: rhs.target.x + self.target.x,
                y: rhs.target.y + self.target.y
            },
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    /// Performs a mathematical subtraction of two [`Vector2D`]s.
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D {
            origin: self.origin,
            target: Point2D {
                x: self.target.x - rhs.target.x,
                y: self.target.y - rhs.target.y
            },
        }
    }
}

impl<T: Into<f64>, E: Into<f64>> From<(T, E)> for Point2D {
    fn from(value: (T, E)) -> Self {
        let value: (f64, f64) = (value.0.into(), value.1.into());

        Point2D {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add for Point2D {
    type Output = Self;

    /// Performs a mathematical addition of two [`Point2D`]s.
    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    /// Performs a mathematical subtraction of two [`Point2D`]s.
    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}