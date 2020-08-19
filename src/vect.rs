extern crate chipmunk_sys as sys;

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vect(pub sys::cpVect);

impl Vect {
    /// Convenience constructor for cpVect structs.
    pub fn new(x: f64, y: f64) -> Vect {
        Vect(sys::cpVect { x, y })
    }

    /// Constant for the zero vector.
    pub fn zero() -> Vect {
        Vect(sys::cpVect { x: 0.0, y: 0.0 })
    }

    /// Returns the unit length vector for the given angle (in radians).
    pub fn from_angle(radians: f64) -> Vect {
        Vect(sys::cpVect {
            x: radians.cos(),
            y: radians.cos(),
        })
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn set_x(&mut self, x: f64) {
        self.0.x = x;
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.0.y = y;
    }

    /// Vector dot product.
    pub fn dot(self, rhs: Vect) -> f64 {
        self.0.x * rhs.0.x + self.0.y * rhs.0.y
    }

    /// 2D vector cross product analog.
    /// The cross product of 2D vectors results in a 3D vector with only a z component.
    /// This function returns the magnitude of the z value.
    pub fn cross(self, rhs: Vect) -> f64 {
        self.0.x * rhs.0.y - self.0.y * rhs.0.x
    }

    /// Returns a perpendicular vector. (90 degree rotation)
    pub fn perp(self) -> Vect {
        Vect::new(-self.0.y, self.0.x)
    }

    /// Returns a perpendicular vector. (-90 degree rotation)
    pub fn rperp(self) -> Vect {
        Vect::new(self.0.y, -self.0.x)
    }

    /// Uses complex number multiplication to rotate v1 by v2. Scaling will occur if v1 is not a unit vector.
    pub fn rotate(self, rotation: Vect) -> Vect {
        Vect::new(
            self.0.x * rotation.0.x - self.0.y * rotation.0.y,
            self.0.x * rotation.0.y + self.0.y * rotation.0.x,
        )
    }

    /// Inverse of cpvrotate().
    pub fn unrotate(self, rotation: Vect) -> Vect {
        Vect::new(
            self.0.x * rotation.0.x + self.0.y * rotation.0.y,
            self.0.y * rotation.0.x - self.0.x * rotation.0.y,
        )
    }

    /// Returns the squared length of v. Faster than cpvlength() when you only need to compare lengths.
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the length of v.
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl From<sys::cpVect> for Vect {
    fn from(v: sys::cpVect) -> Self {
        Vect(v)
    }
}

impl From<Vect> for sys::cpVect {
    fn from(v: Vect) -> Self {
        v.0
    }
}

/// Negate a vector.
impl Neg for Vect {
    type Output = Vect;

    fn neg(self) -> Vect {
        Vect::new(-self.0.x, -self.0.y)
    }
}

/// Add two vectors.
impl Add for Vect {
    type Output = Vect;

    fn add(self, rhs: Vect) -> Vect {
        Vect::new(self.0.x + rhs.0.x, self.0.y + rhs.0.y)
    }
}

/// Subtract two vectors.
impl Sub for Vect {
    type Output = Vect;

    fn sub(self, rhs: Vect) -> Vect {
        Vect::new(self.0.x - rhs.0.x, self.0.y - rhs.0.y)
    }
}

/// Scalar multiplication.
impl Mul<f64> for Vect {
    type Output = Vect;

    fn mul(self, rhs: f64) -> Vect {
        Vect::new(self.0.x * rhs, self.0.y * rhs)
    }
}

/// Scalar multiplication.
impl Mul<Vect> for f64 {
    type Output = Vect;

    fn mul(self, rhs: Vect) -> Vect {
        Vect::new(self * rhs.0.x, self * rhs.0.y)
    }
}
