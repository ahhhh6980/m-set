#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Complex {
	pub r: f32,
	pub i: f32,
}
impl Complex {
	pub fn new(r: f32, i: f32) -> Self {
		Self { r, i }
	}
}

impl std::ops::Add for Complex {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			r: self.r + rhs.r,
			i: self.i + rhs.i,
		}
	}
}
impl std::ops::Sub for Complex {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			r: self.r - rhs.r,
			i: self.i - rhs.i,
		}
	}
}

impl std::ops::Not for Complex {
	type Output = Self;
	fn not(self) -> Self::Output {
		Self {
			r: self.r,
			i: -self.i,
		}
	}
}


impl std::ops::Mul for Complex {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			r: self.r.mul_add(rhs.r, -(self.i * rhs.i)),
			i: self.r.mul_add(rhs.i, self.i * rhs.r),
		}
	}
}

impl std::ops::Mul<f32> for Complex {
	type Output = Self;
	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			r: self.r * rhs,
			i: self.i * rhs
		}
	}
}
impl std::ops::Div for Complex {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		self * !rhs
	}
}

pub fn tri_dist(v: f32) -> f32 {
	let o = v * 2. - 1.;
	(-1f32).max(o / o.abs().sqrt()) - o.signum()
}
