use std::fmt::Display;

use rand_chacha::rand_core::RngCore;
use wasmer::Value;

use crate::float::*;

#[derive(Debug, Copy, Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
        }
    }
}

impl Type {
    pub fn random_value(self, rand: &mut impl RngCore) -> Value {
        match self {
            Type::I32 => Value::I32(rand.next_u32() as i32),
            Type::I64 => Value::I64(rand.next_u64() as i64),
            Type::F32 => {
                let decider = rand.next_u32();
                let bits = if decider < u32::MAX / 4 {
                    // 25% chance of being a NaN
                    random_nan_32(rand)
                } else if decider < u32::MAX / 2 {
                    // 25% chance of being a subnormal
                    random_subnormal_32(rand)
                } else if decider < u32::MAX / 4 * 3 {
                    // 25% chance of being an infinite
                    if decider % 2 == 0 {
                        INF_32
                    } else {
                        NEG_INF_32
                    }
                } else {
                    // 25% chance of being a random bit pattern
                    rand.next_u32()
                };
                Value::F32(f32::from_bits(bits))
            }
            Type::F64 => {
                let decider = rand.next_u64();
                let bits = if decider < u64::MAX / 4 {
                    // 25% chance of being a NaN
                    random_nan_64(rand)
                } else if decider < u64::MAX / 2 {
                    // 25% chance of being a subnormal
                    random_subnormal_64(rand)
                } else if decider < u64::MAX / 4 * 3 {
                    // 25% chance of being an infinite
                    if decider % 2 == 0 {
                        INF_64
                    } else {
                        NEG_INF_64
                    }
                } else {
                    // 25% chance of being a random bit pattern
                    rand.next_u64()
                };
                Value::F64(f64::from_bits(bits))
            }
        }
    }
}

pub trait Transposable {
    type Transposed;
    fn transpose(self) -> Self::Transposed;
}

impl<A, B, C, E: Clone> Transposable for Result<(A, B, C), E> {
    type Transposed = (Result<A, E>, Result<B, E>, Result<C, E>);

    fn transpose(self) -> Self::Transposed {
        match self {
            Ok((a, b, c)) => (Ok(a), Ok(b), Ok(c)),
            Err(e) => (Err(e.clone()), Err(e.clone()), Err(e)),
        }
    }
}
