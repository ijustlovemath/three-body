extern crate num_rational;
extern crate num_bigint;
extern crate num_traits;

use num_rational::{Ratio, BigRational};
use num_bigint::BigInt;
use std::ops::{Add, Sub, Mul, Neg, Div, AddAssign};

#[derive(Clone, Debug)]
struct Coords {
    x: BigRational,
    y: BigRational,
    z: BigRational
}

impl Coords {
    fn from_floats(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Ratio::from_float(x).unwrap(),
            y: Ratio::from_float(y).unwrap(),
            z: Ratio::from_float(z).unwrap()
        }
    }

    fn from_float(f: f64) -> Self {
        Self {
            x: Ratio::from_float(f).unwrap(),
            y: Ratio::from_float(f).unwrap(),
            z: Ratio::from_float(f).unwrap(),
        }
    }

    fn from_scalar(s: BigRational) -> Self {
        Self {
            x: s.clone(),
            y: s.clone(),
            z: s
        }
    }

    fn norm(&self) -> BigRational {
        let mut res = Ratio::from_float(0.0).unwrap();
        res += self.x.pow(2);
        res += self.y.pow(2);
        res += self.z.pow(2);
        res
    }

    fn abs(&self) -> BigRational {
        let res = self.norm();
        Ratio::new(res.numer().sqrt(), res.denom().sqrt())
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

//impl Sub for &Coords {
//    type Output = Self;
//
//    fn sub(&self, other: &Self) -> Self {
//        Self {
//            x: *self.x - *other.x,
//            y: *self.y - *other.y,
//            z: *self.z - *other.z
//        }
//    }
//}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Neg for Coords {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul for Coords {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div for Coords {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Clone, Debug)]
struct Planet {
    location: Coords,
    mass: BigRational
}

impl Planet {
    fn from_floats (m: f64, x: f64, y: f64, z: f64) -> Self {
        Self {
            location: Coords::from_floats(x, y, z),
            mass: Ratio::from_float(m).unwrap()
        }
    }

    fn dot(&mut self, others: &[&Self]) {
        let mut change = Coords::from_floats(0.0, 0.0, 0.0);
        let G = Coords::from_float(6.674e-11);
        let m = Coords::from_scalar(self.mass.clone());
        for planet in others {
            let diff = (self.location.clone() - planet.location.clone());
            let cube = Coords::from_scalar(diff.norm() * diff.abs());
            change += -G.clone() * m.clone() * diff / cube;
        }
        self.location += change;
    }

    fn loc(&self) -> BigInt {
        let a = self.location.abs();
        return a.numer() / a.denom();
    }
}

fn main() {
    let r1 = Coords::from_floats(0.0, 0.1, 0.3);
    let r2 = Coords::from_floats(100.0, 100.0, 200.0);
    assert_eq!((r2.clone() - r1.clone()).norm().round()
               , Ratio::from_integer(
                    BigInt::parse_bytes(b"59860", 10).unwrap()
                )
    );
    let d = (r2.clone() - r1.clone()).norm();
    let a = (r2-r1).abs();
    println!("Hello, world! {}, {}", a.numer()/a.denom(), d=d.numer()/d.denom());

    let mut earth = Planet::from_floats(5.97e24,  1.0, 0.0, 0.0);
    let mut moon = Planet::from_floats(7.342e22, 362600e3, 0.0, 0.0);
    for i in [0, 1, 2].iter() {
        moon.dot(&[&earth]);
        earth.dot(&[&moon]);
        println!("moon: {:?}", moon);
        println!("earth: {:?}", earth);
    }
}
