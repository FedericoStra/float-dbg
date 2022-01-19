use std::fmt::{Binary, Debug, Display};

pub trait FloatDbg<U, S>: Copy + Sized + Debug
where
    U: Default + Binary + Display + PartialEq + std::ops::BitAnd<Output = U>,
    S: Display,
{
    const WIDTH: usize;
    const EXP_WIDTH: usize;
    const FRACT_WIDTH: usize;

    const SIGN_MASK: U;
    const EXP_MASK: U;
    const FRACT_MASK: U;

    const EXP_BIAS: S;

    fn to_bits(self) -> U;
    fn sign_bit(self) -> bool {
        self.to_bits() & Self::SIGN_MASK != U::default()
    }

    fn biased_exponent(self) -> U;
    fn unbiased_exponent(self) -> S;

    fn fraction(self) -> U;
    fn significand(self) -> U;

    fn explain(self) {
        println!("value = {:?}", self);
        println!("bits: {:0width$b}", self.to_bits(), width = Self::WIDTH);
        println!(
            "      ±{:^<e$}{:_<f$}",
            "",
            "",
            e = Self::EXP_WIDTH,
            f = Self::FRACT_WIDTH
        );
        println!("sign: {}", if self.sign_bit() { "-" } else { "+" });
        println!(
            "exponent = {} - {} = {}",
            self.biased_exponent(),
            Self::EXP_BIAS,
            self.unbiased_exponent()
        );
        println!(
            "significand = 2^{} + {} = {}",
            Self::FRACT_WIDTH,
            self.fraction(),
            self.significand()
        );
    }
}

impl FloatDbg<u32, i32> for f32 {
    const WIDTH: usize = 32;
    const EXP_WIDTH: usize = 8;
    const FRACT_WIDTH: usize = 23;

    const SIGN_MASK: u32 = 1 << (Self::WIDTH - 1);
    const EXP_MASK: u32 = ((1 << Self::EXP_WIDTH) - 1) << Self::FRACT_WIDTH;
    const FRACT_MASK: u32 = (1 << Self::FRACT_WIDTH) - 1;

    const EXP_BIAS: i32 = (1 << (Self::EXP_WIDTH - 1)) - 1;

    fn to_bits(self) -> u32 {
        self.to_bits()
    }

    fn biased_exponent(self) -> u32 {
        (self.to_bits() & Self::EXP_MASK) >> Self::FRACT_WIDTH
    }

    fn unbiased_exponent(self) -> i32 {
        (self.biased_exponent() as i32).wrapping_sub(Self::EXP_BIAS)
    }

    fn fraction(self) -> u32 {
        self.to_bits() & Self::FRACT_MASK
    }

    fn significand(self) -> u32 {
        self.fraction() + (1 << Self::FRACT_WIDTH)
    }
}

impl FloatDbg<u64, i64> for f64 {
    const WIDTH: usize = 64;
    const EXP_WIDTH: usize = 11;
    const FRACT_WIDTH: usize = 52;

    const SIGN_MASK: u64 = 1 << (Self::WIDTH - 1);
    const EXP_MASK: u64 = ((1 << Self::EXP_WIDTH) - 1) << Self::FRACT_WIDTH;
    const FRACT_MASK: u64 = (1 << Self::FRACT_WIDTH) - 1;

    const EXP_BIAS: i64 = (1 << (Self::EXP_WIDTH - 1)) - 1;

    fn to_bits(self) -> u64 {
        self.to_bits()
    }

    fn biased_exponent(self) -> u64 {
        (self.to_bits() & Self::EXP_MASK) >> Self::FRACT_WIDTH
    }

    fn unbiased_exponent(self) -> i64 {
        (self.biased_exponent() as i64).wrapping_sub(Self::EXP_BIAS)
    }

    fn fraction(self) -> u64 {
        self.to_bits() & Self::FRACT_MASK
    }

    fn significand(self) -> u64 {
        self.fraction() + (1 << Self::FRACT_WIDTH)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
