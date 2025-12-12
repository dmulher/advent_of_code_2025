use std::{fmt::Debug, iter::Sum, ops::{Add, Div, Mul, Sub}};

pub trait Lcm {
  fn lcm(a: Self, b: Self) -> Self;
}
pub trait Gcd {
  fn gcd(a: Self, b: Self) -> Self;
}

impl Lcm for u64 {
  fn lcm(a: u64, b: u64) -> u64 {
    a * b / u64::gcd(a, b)
  }
}

impl Gcd for u64 {
  fn gcd(a: u64, b: u64) -> u64 {
    match b {
      0 => a,
      _ => u64::gcd(b, a % b)
    }
  }
}

impl Lcm for i16 {
  fn lcm(a: i16, b: i16) -> i16 {
    a * b / i16::gcd(a, b)
  }
}

impl Gcd for i16 {
  fn gcd(a: i16, b: i16) -> i16 {
    match b {
      0 => a,
      _ => i16::gcd(b, a % b)
    }
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Frac {
  num: i16,
  den: i16
}

impl Frac {
  pub fn new(numerator: i16, denominator: i16) -> Self {
    if numerator == 0 {
      Frac{
        num: 0,
        den: 1
      }
    } else {
      let gcd = i16::gcd(numerator.abs(), denominator.abs());
      let num = if (numerator < 0) ^ (denominator < 0) {- (numerator.abs() / gcd)} else {numerator.abs() / gcd};
      let den = denominator.abs() / gcd;
      Frac{ num, den }
    }
  }

  pub fn is_zero(&self) -> bool {
    self.num == 0
  }

  pub fn is_whole(&self) -> bool {
    self.num % self.den == 0
  }

  pub fn floor(&self) -> i16 {
    self.num / self.den
  }

  pub fn is_negative(&self) -> bool {
    self.num < 0
  }
}

impl Debug for Frac {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}/{}", self.num, self.den)
  }
}

impl Add for Frac {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Frac::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
  }
}

impl Sub for Frac {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Frac::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
  }
}

impl Mul for Frac {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Frac::new(self.num * rhs.num, self.den * rhs.den)
  }
}

impl Mul<i16> for Frac {
    type Output = Self;

    fn mul(self, rhs: i16) -> Self::Output {
      Frac::new(self.num * rhs, self.den)
    }
}

impl Div for Frac {
  type Output = Self;

  fn div(self, rhs: Self) -> Self::Output {
    Frac::new(self.num * rhs.den, self.den * rhs.num)
  }
}

impl Sum for Frac {
  fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
    iter.fold(Frac::new(0, 1), |acc, s| acc + s)
  }
}
