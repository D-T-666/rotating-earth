use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub f64, pub f64);

impl Mul<f64> for Vector {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Vector(self.0 * rhs, self.1 * rhs)
	}
}

impl Add for Vector {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Vector(self.0 + rhs.0, self.1 + rhs.1)
	}
}

impl AddAssign<Vector> for Vector {
	fn add_assign(&mut self, rhs: Vector) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

impl Sub for Vector {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Vector(self.0 - rhs.0, self.1 - rhs.1)
	}
}
pub trait AngleToVector {
	fn angle_to_vector(self) -> Vector;
}

impl AngleToVector for f64 {
	fn angle_to_vector(self) -> Vector {
		Vector(self.sin(), self.cos())
	}
}

pub trait Length {
	fn length_squared(&self) -> f64;
	fn length(&self) -> f64;
}

impl Length for Vector {
	fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	fn length_squared(&self) -> f64 {
		self.0 * self.0 + self.1 * self.1
	}
}
