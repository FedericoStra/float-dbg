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

macro_rules! impl_FloatDbg {
    ($F:ty, $U:ty, $S:ty; $width:expr, $exp_width:expr, $frac_width:expr) => {
        impl FloatDbg<$U, $S> for $F {
            const WIDTH: usize = $width;
            const EXP_WIDTH: usize = $exp_width;
            const FRACT_WIDTH: usize = $frac_width;

            const SIGN_MASK: $U = 1 << (Self::WIDTH - 1);
            const EXP_MASK: $U = ((1 << Self::EXP_WIDTH) - 1) << Self::FRACT_WIDTH;
            const FRACT_MASK: $U = (1 << Self::FRACT_WIDTH) - 1;

            const EXP_BIAS: $S = (1 << (Self::EXP_WIDTH - 1)) - 1;

            fn to_bits(self) -> $U {
                self.to_bits()
            }

            fn biased_exponent(self) -> $U {
                (self.to_bits() & Self::EXP_MASK) >> Self::FRACT_WIDTH
            }

            fn unbiased_exponent(self) -> $S {
                (self.biased_exponent() as $S).wrapping_sub(Self::EXP_BIAS)
            }

            fn fraction(self) -> $U {
                self.to_bits() & Self::FRACT_MASK
            }

            fn significand(self) -> $U {
                self.fraction() + (1 << Self::FRACT_WIDTH)
            }
        }
    };
}

impl_FloatDbg!(f32, u32, i32; 32, 8, 23);
impl_FloatDbg!(f64, u64, i64; 64, 11, 52);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
