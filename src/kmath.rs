
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub fn unlerp(a: f32, b: f32, x: f32) -> f32 {
    (x - a) / (b - a)
}

pub fn remap(x: f32, sx: f32, sy: f32, dx: f32, dy: f32) -> f32 {
    lerp(dx, dy, unlerp(sx, sy, x))
}

pub fn saturate(x: f32, a: f32, b: f32) -> f32 {
    remap(x, a, b, 0.0, 1.0).clamp(0.0, 1.0)
}

#[test]
pub fn test_lerpy() {
    assert_eq!(0.5, lerp(0.0, 1.0, 0.5));
    assert_eq!(0.5, lerp(0.0, 1.0, unlerp(0.0, 1.0, 0.5)));
    assert_eq!(0.25, unlerp(68.0, 72.0, 69.0));
    // assert_eq!(0.9, remap(0.54, 0.45, 0.55, 0.0, 1.0));
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x * scalar, self.y * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec2 { Vec2::new(self.x / scalar, self.y / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y).sqrt() }
    pub fn normalize(&self) -> Vec2 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec2, t: f32) -> Vec2 { Vec2::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t)) }
    pub fn rotate(&self, radians: f32) -> Vec2 { 
        Vec2::new(
            self.x * radians.cos() - self.y * radians.sin(), 
            self.x * radians.sin() + self.y * radians.cos()
        ) 
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - _rhs.x, y: self.y - _rhs.y }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, _rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, _rhs: f32) -> Vec2 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, _rhs: f32) -> Vec2 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        self.mul_scalar(-1.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3{x, y, z} }
    pub fn mul_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar) }
    pub fn div_scalar(&self, scalar: f32) -> Vec3 { Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar) }
    pub fn magnitude(&self) -> f32 { (self.x*self.x + self.y*self.y + self.z*self.z).sqrt() }
    pub fn square_distance(&self) -> f32 { self.x*self.x + self.y*self.y + self.z*self.z }
    pub fn normalize(&self) -> Vec3 { self.div_scalar(self.magnitude()) }
    pub fn lerp(&self, other: Vec3, t: f32) -> Vec3 { Vec3::new(self.x*(1.0-t) + other.x*(t), self.y*(1.0-t) + other.y*(t), self.z*(1.0-t) + other.z*(t)) }
    pub fn dist(&self, other: Vec3) -> f32 {(*self - other).magnitude().sqrt()}
    pub fn dot(&self, other: Vec3) -> f32 {self.x*other.x + self.y*other.y + self.z*other.z} // is squ dist lol
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y*other.z - self.z*other.y,
            self.z*other.x - self.x*other.z,
            self.x*other.y - self.y*other.x,
        )
    }
    pub fn rotate_about_vec3(&self, axis: Vec3, theta: f32) -> Vec3 {
        *self*theta.cos() + (axis.cross(*self)*theta.sin()) + axis * (axis.dot(*self)*(1.0 - theta.cos()))
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        self.mul_scalar(_rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs.mul_scalar(self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        self.div_scalar(_rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.mul_scalar(-1.0)
    }
}

impl std::ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("[{:.*}, {:.*}, {:.*}]", decimals, self.x, decimals, self.y, decimals, self.z);
        f.pad_integral(true, "", &string)
    }
}

// maybe x intervals, y heights would be better

// x 0..1
// return start index, sub interval
// first in list should be 0
pub fn x_in_list(x: f32, intervals: &[f32]) -> (usize, f32) {
    assert_eq!(x <= 1.0, true);
    assert_eq!(x >= 0.0, true);

    
    let max: f32 = intervals.iter().sum();
    let normalized_intervals = intervals.iter().map(|x| x / max);

    let mut cum_x = x;

    for (i, interval) in normalized_intervals.enumerate() {
        if cum_x <= interval {
            return (i, cum_x / interval);
        } else {
            cum_x -= interval;
        }
    }
    (intervals.len()-1, 1.0)
    // panic!("unreachable xil");
}

#[test]
fn test_xil() {
    assert_eq!(x_in_list(0.0, &[1.0]), (0, 0.0));
    assert_eq!(x_in_list(1.0, &[1.0]), (0, 1.0));
    assert_eq!(x_in_list(0.5, &[1.0]), (0, 0.5));
    assert_eq!(x_in_list(0.5, &[10.0, 1.0]), (0, 0.55));
}

pub fn bezier3(t: f32, a: Vec2, b: Vec2, c1: Vec2, c2: Vec2) -> Vec2 {
    let d = a.lerp(c1, t);
    let e = c1.lerp(c2, t);
    let f = c2.lerp(b, t);

    let g = d.lerp(e, t);
    let h = e.lerp(f, t);
    g.lerp(h, t)
}

// controls are relative to interval
// return y value
pub fn bezier_transect(t: f32, x_intervals: &[f32], y_heights: &[f32], controls: &[(Vec2, Vec2)]) -> f32 {
    assert_eq!(x_intervals.len(), controls.len());
    assert_eq!(y_heights.len(), controls.len() + 1);
    assert_eq!(t >= 0.0 && t <= 1.0, true);
    
    let (interval, interval_t) = x_in_list(t, x_intervals);
    let a = Vec2::new(0.0, y_heights[interval]);
    let b = Vec2::new(1.0, y_heights[interval + 1]);
    let (dc1, dc2) = controls[interval];
    let (c1, c2) = (dc1 + a, dc2 + b);

    bezier3(interval_t, a, b, c1, c2).y
}