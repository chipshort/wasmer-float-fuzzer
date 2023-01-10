use rand_chacha::rand_core::RngCore;

pub const INF_32: u32 = 0x7f800000;
pub const NEG_INF_32: u32 = 0xff800000;
pub const INF_64: u64 = 0x7ff0000000000000;
pub const NEG_INF_64: u64 = 0xfff0000000000000;

const EXPONENT_MASK_32: u32 = 0x7f800000;
const EXPONENT_MASK_64: u64 = 0x7ff0000000000000;
const SIGN_MASK_32: u32 = 0x80000000;
const SIGN_MASK_64: u64 = 0x8000000000000000;
const MANTISSA_MASK_32: u32 = 0x007fffff;
const MANTISSA_MASK_64: u64 = 0x000fffffffffffff;

/// Returns bits for a random NaN (with a small chance for an Infinity)
pub fn random_nan_32(rng: &mut impl RngCore) -> u32 {
    // Set the exponent to all 1s and remaining bits random
    EXPONENT_MASK_32 | rng.next_u32()
}

/// Returns bits for a random NaN (with a small chance for an Infinity)
pub fn random_nan_64(rng: &mut impl RngCore) -> u64 {
    // Set the exponent to all 1s and remaining bits random
    EXPONENT_MASK_64 | rng.next_u64()
}

/// Returns bits for a random subnormal (with a small chance for a zero)
pub fn random_subnormal_32(rng: &mut impl RngCore) -> u32 {
    // Set the exponent to all 0s and remaining bits random
    rng.next_u32() & (SIGN_MASK_32 | MANTISSA_MASK_32)
}

/// Returns bits for a random subnormal (with a small chance for a zero)
pub fn random_subnormal_64(rng: &mut impl RngCore) -> u64 {
    // Set the exponent to all 0s and remaining bits random
    rng.next_u64() & (SIGN_MASK_64 | MANTISSA_MASK_64)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Normal,
    Subnormal,
    Zero,
    Infinite,
    NaN,
}

pub trait Classifier {
    fn classify(&self) -> Class;
}

impl Classifier for u32 {
    fn classify(&self) -> Class {
        let exponent = self & EXPONENT_MASK_32;
        let mantissa = self & MANTISSA_MASK_32;

        match (exponent, mantissa) {
            (0, 0) => Class::Zero,
            (0, _) => Class::Subnormal,
            (EXPONENT_MASK_32, 0) => Class::Infinite,
            (EXPONENT_MASK_32, _) => Class::NaN,
            _ => Class::Normal,
        }
    }
}

impl Classifier for u64 {
    fn classify(&self) -> Class {
        let exponent = self & EXPONENT_MASK_64;
        let mantissa = self & MANTISSA_MASK_64;

        match (exponent, mantissa) {
            (0, 0) => Class::Zero,
            (0, _) => Class::Subnormal,
            (EXPONENT_MASK_64, 0) => Class::Infinite,
            (EXPONENT_MASK_64, _) => Class::NaN,
            _ => Class::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand_chacha::rand_core::SeedableRng;

    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(INF_32, f32::INFINITY.to_bits());
        assert_eq!(NEG_INF_32, f32::NEG_INFINITY.to_bits());
        assert_eq!(INF_64, f64::INFINITY.to_bits());
        assert_eq!(NEG_INF_64, f64::NEG_INFINITY.to_bits());
    }

    #[test]
    fn test_classify() {
        // for 32-bit floats
        assert_eq!((-0f32).to_bits().classify(), Class::Zero);
        assert_eq!(0u32.classify(), Class::Zero);
        assert_eq!(1f32.to_bits().classify(), Class::Normal);
        assert_eq!(INF_32.classify(), Class::Infinite);
        assert_eq!(NEG_INF_32.classify(), Class::Infinite);

        // for 64-bit floats
        assert_eq!((-0f64).to_bits().classify(), Class::Zero);
        assert_eq!(0u64.classify(), Class::Zero);
        assert_eq!(1f64.to_bits().classify(), Class::Normal);
        assert_eq!(INF_64.classify(), Class::Infinite);
        assert_eq!(NEG_INF_64.classify(), Class::Infinite);

        // random floats
        let mut rng = rand_chacha::ChaChaRng::seed_from_u64(123456);
        for _ in 0..1000 {
            assert_eq!(random_subnormal_32(&mut rng).classify(), Class::Subnormal);
            assert_eq!(random_nan_32(&mut rng).classify(), Class::NaN);

            assert_eq!(random_subnormal_64(&mut rng).classify(), Class::Subnormal);
            assert_eq!(random_nan_64(&mut rng).classify(), Class::NaN);
        }
    }
}
