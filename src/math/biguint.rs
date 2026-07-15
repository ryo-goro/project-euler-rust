use std::ops::{
    Add,
    AddAssign,
    Mul,
    MulAssign,
};

#[derive(Clone, Default)]
pub struct BigUint {
    inner: Vec<u8>,
}

impl BigUint {
    pub fn zero() -> Self {
        BigUint {
            inner: Vec::new(),
        }
    }

    pub fn one() -> Self {
        BigUint {
            inner: vec![1],
        }
    }

    pub fn is_zero(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    pub fn digit_sum(&self) -> u64 {
        self.inner.iter().map(|&digit| digit as u64).sum()
    }
}

impl From<u64> for BigUint {
    fn from(mut value: u64) -> Self {
        if value == 0{
            return Self::zero();
        }

        let mut inner = Vec::with_capacity(32);

        while value > 0 {
            inner.push((value % 10) as u8);
            value /= 10;
        }

        BigUint {
            inner,
        }
    }
}

impl Add for BigUint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for BigUint {
    fn add_assign(&mut self, mut rhs: Self) {
        if self.is_zero() {
            self.inner = rhs.inner;
            return;
        }

        if rhs.is_zero() {
            return;
        }

        let mut self_len = self.inner.len();
        let mut rhs_len = rhs.inner.len();

        if rhs_len > self_len {
            std::mem::swap(&mut self.inner, &mut rhs.inner);
            std::mem::swap(&mut self_len, &mut rhs_len);
        }

        let mut carry = 0;

        for i in 0..rhs_len {
            let x = self.inner[i] + rhs.inner[i] + carry;

            if x >= 10 {
                self.inner[i] = x - 10;
                carry = 1;
            } else {
                self.inner[i] = x;
                carry = 0;
            }
        }

        if carry == 0 {
            return;
        }

        for i in rhs_len..self_len {
            let x = self.inner[i] + carry;

            if x == 10 {
                self.inner[i] = 0;
            } else {
                self.inner[i] = x;
                return;
            }
        }

        if carry == 1 {
            self.inner.push(1);
        }
    }
}

impl Mul<u64> for BigUint {
    type Output = Self;

    fn mul(mut self, rhs: u64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<u64> for BigUint {
    fn mul_assign(&mut self, rhs: u64) {
        if rhs == 0 {
            self.inner.clear();
            return;
        }

        if rhs == 1 {
            return;
        }

        let rhs = rhs as u128;
        let self_len = self.inner.len();

        let mut carry = 0;

        for i in 0..self_len {
            let x: u128 = (self.inner[i] as u128) * rhs + carry;

            self.inner[i] = (x % 10) as u8;
            carry = x / 10;
        }

        while carry > 0 {
            self.inner.push((carry % 10) as u8);
            carry /= 10;
        }
    }
}
