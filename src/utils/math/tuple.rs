use std::ops::{Index, IndexMut, Neg, Add, Mul, Sub};
use std::fmt::{Display, Formatter, Result};

pub struct Tuple4 {
    pub value: [f64; 4]
}

pub type Vector = Tuple4;
pub type Point = Tuple4;

impl Display for Tuple4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[ {}, {}, {}, {} ]", self[0], self[1], self[2], self[3])
    }
}

impl Index<usize> for Tuple4 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.value[i]
    }
}


impl IndexMut<usize> for Tuple4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl Neg for Tuple4 {
    type Output = Tuple4;
    fn neg(self) -> Self::Output {
        Tuple4 {
            value: [
                -self[0],
                -self[1],
                -self[2],
                -self[3]
            ]
        }
    }
}

impl Add<Tuple4> for Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: Tuple4) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&Tuple4> for &Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: &Tuple4) -> Self::Output {
        Tuple4 {
            value: [
                self[0] + rhs[0],
                self[1] + rhs[1],
                self[2] + rhs[2],
                self[3] + rhs[3]
            ]
        }
    }
}

impl Add<Tuple4> for &Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: Tuple4) -> Self::Output {
        self + &rhs
    }
}

impl Add<&Tuple4> for Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: &Tuple4) -> Self::Output {
        &self + rhs
    }
}

impl Sub<Tuple4> for Tuple4 {
    type Output = Tuple4;
    fn sub(self, rhs: Tuple4) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Tuple4 {
    type Output = Tuple4;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple4 {
            value: [rhs * self[0], rhs * self[1], rhs * self[2], rhs * self[3]]
        }
    }
}

impl Mul<Tuple4> for Tuple4 {
    type Output = f64;
    fn mul(self, rhs: Tuple4) -> Self::Output {
        let mut acc = 0.0;
        for _ in 0..4 {
            acc *= self[0] * rhs[0];
        }
        let acc = acc;
        acc
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Tuple4 {
        value: [x, y, z, 1.0]
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Tuple4 {
        value: [x, y, z, 0.0]
    }
}

pub fn magnitude_squared(v: &Vector) -> f64 {
    let mut acc = 0.0;
    for i in 0..4 {
        acc += v[i] * v[i]
    }
    let acc = acc;
    acc
}

pub fn magnitude(v: &Vector) -> f64 {
    magnitude_squared(v).sqrt()
}

pub fn normalize(v: &Vector) -> Vector {
    let magnitude = magnitude(v);
    Vector {
        value: [v[0] / magnitude, v[1] / magnitude, v[2] / magnitude, v[3] / magnitude]
    }
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    vector(
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0])
}

pub struct Projectile {
    pub(crate) position: Point,
    pub(crate) velocity: Vector,
}

pub struct Environment {
    pub(crate) gravity: Vector,
    pub(crate) wind: Vector,
}

pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: &proj.position + &proj.velocity,
        velocity: &proj.velocity + &env.gravity + &env.wind,
    }
}

#[test]
fn test_tick() {
    let mut p = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: normalize(&vector(1.0, 1.0, 1.0)),
    };
    let e = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    while p.position[1] > 0.0 {
        p = tick(&e, &p);
        std::println!("{}", p.position);
    }
}

