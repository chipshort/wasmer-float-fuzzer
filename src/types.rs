use std::fmt::Display;

use rand_chacha::rand_core::RngCore;
use wasmer::Value;

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
            Type::F32 => Value::F32(f32::from_bits(rand.next_u32())),
            Type::F64 => Value::F64(f64::from_bits(rand.next_u64())),
        }
    }
}
