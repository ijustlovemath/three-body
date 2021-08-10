extern crate num_rational;
extern crate num_bigint;
extern crate num_traits;

use num_rational::{Ratio};
use num_bigint::BigInt;
use std::ops::{Add, Sub};

#[derive(Clone)]
struct Coords {
    x: Ratio<BigInt>,
    y: Ratio<BigInt>,
    z: Ratio<BigInt>
}

impl Coords {
    fn from_floats(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Ratio::from_float(x).unwrap(),
            y: Ratio::from_float(y).unwrap(),
            z: Ratio::from_float(z).unwrap()
        }
    }

    fn norm(self) -> Ratio<BigInt> {
        let mut res = Ratio::from_float(0.0).unwrap();
        res += self.x.pow(2);
        res += self.y.pow(2);
        res += self.z.pow(2);
        res
    }

    fn abs(self) -> Ratio<BigInt> {
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
}
