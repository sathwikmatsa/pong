use arrayvec::ArrayVec;
use std::convert::From;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Ball {
    pub centre: [i32; 2],
    pub velocity: Vector,
    radius: f64,
}

impl Ball {
    pub fn new(centre: [i32; 2], velocity: [f32; 2], radius: f64) -> Self {
        Self {
            centre,
            velocity: velocity.into(),
            radius,
        }
    }
    pub fn centre_x(&self) -> i32 {
        self.centre[0]
    }
    pub fn centre_y(&self) -> i32 {
        self.centre[1]
    }
    pub fn centre_x_f64(&self) -> f64 {
        self.centre[0] as f64
    }
    pub fn centre_y_f64(&self) -> f64 {
        self.centre[1] as f64
    }
    pub fn is_between_vseg(&self, top: u32, vlen: u32) -> bool {
        let ball_below_top = self.centre[1] + self.radius as i32 > top as i32;
        let ball_above_bottom = (self.centre[1] - self.radius as i32) < (top + vlen) as i32;
        ball_below_top && ball_above_bottom
    }
    pub fn collides_left_vseg(&self, top: u32, vlen: u32, x: u32) -> bool {
        let ball_left_of_vseg = (self.centre[0] - self.radius as i32) <= x as i32;
        if self.is_between_vseg(top, vlen) && ball_left_of_vseg {
            return true;
        }
        false
    }
    pub fn collides_right_vseg(&self, top: u32, vlen: u32, x: u32) -> bool {
        let ball_right_of_vseg = (self.centre[0] + self.radius as i32) >= x as i32;
        if self.is_between_vseg(top, vlen) && ball_right_of_vseg {
            return true;
        }
        false
    }
    pub fn reflect_from_left(&mut self, theta: f64, speed: u32) {
        self.velocity = [
            speed as f32 * theta.cos() as f32,
            speed as f32 * -theta.sin() as f32,
        ]
        .into();
    }
    pub fn reflect_from_right(&mut self, theta: f64, speed: u32) {
        self.velocity = [
            -(speed as f32 * theta.cos() as f32),
            speed as f32 * -theta.sin() as f32,
        ]
        .into();
    }
    pub fn reflect_with_normal(&mut self, normal: [f32; 2]) {
        self.velocity = self.velocity.reflect(normal.into());
    }
    pub fn export(&self) -> Vec<u8> {
        //[ball_centre_x(4) - ball_centre_y(4) - ball_velocity_i(4) - ball_velocity_j(4)]
        let ball_centre_x_bytes = self.centre[0].to_le_bytes();
        let ball_centre_y_bytes = self.centre[1].to_le_bytes();
        let ball_vel_i_bytes = self.velocity.i.to_bits().to_ne_bytes();
        let ball_vel_j_bytes = self.velocity.j.to_bits().to_ne_bytes();

        let mut serialized = Vec::new();
        serialized.reserve_exact(16);
        serialized.extend_from_slice(&ball_centre_x_bytes);
        serialized.extend_from_slice(&ball_centre_y_bytes);
        serialized.extend_from_slice(&ball_vel_i_bytes);
        serialized.extend_from_slice(&ball_vel_j_bytes);

        serialized
    }
    pub fn reset(&mut self, serialized: [u8; 16]) {
        //[ball_centre_x(4) - ball_centre_y(4) - ball_velocity_i(4) - ball_velocity_j(4)]
        let ball_centre_x_bytes: ArrayVec<[u8; 4]> = serialized.iter().cloned().take(4).collect();
        let ball_centre_y_bytes: ArrayVec<[u8; 4]> =
            serialized.iter().cloned().skip(4).take(4).collect();
        let ball_vel_i_bytes: ArrayVec<[u8; 4]> =
            serialized.iter().cloned().skip(8).take(4).collect();
        let ball_vel_j_bytes: ArrayVec<[u8; 4]> =
            serialized.iter().cloned().skip(12).take(4).collect();

        self.centre[0] = i32::from_le_bytes(ball_centre_x_bytes.into_inner().unwrap());
        self.centre[1] = i32::from_le_bytes(ball_centre_y_bytes.into_inner().unwrap());
        self.velocity.i =
            f32::from_bits(u32::from_le_bytes(ball_vel_i_bytes.into_inner().unwrap()));
        self.velocity.j =
            f32::from_bits(u32::from_le_bytes(ball_vel_j_bytes.into_inner().unwrap()));
    }
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub i: f32,
    pub j: f32,
}

impl Vector {
    fn dot(self, other: Self) -> f32 {
        self.i * other.i + self.j * other.j
    }
    fn reflect(self, normal: Self) -> Self {
        self - normal * 2. * self.dot(normal)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            i: self.i * rhs,
            j: self.j * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            i: -self.i,
            j: -self.j,
        }
    }
}

impl From<[f32; 2]> for Vector {
    fn from(item: [f32; 2]) -> Self {
        Self {
            i: item[0],
            j: item[1],
        }
    }
}
