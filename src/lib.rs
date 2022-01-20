use std::fmt;

type NBits = u16;

pub trait Bits: Copy + Eq + PartialEq
where
    Self: fmt::Binary,
    Self: std::ops::Shl<NBits, Output = Self>,
    Self: std::ops::Shr<NBits, Output = Self>,
    Self: std::ops::BitAnd<Output = Self>,
{
    /// The number of bits.
    const BITS: NBits;

    /// The value `0`.
    const ZERO: Self;

    // /// The value `1`.
    // const ONE: Self;

    /// Test if the value is zero.
    #[inline]
    fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

macro_rules! impl_Bits {
    ($($T:ty),*) => {$(
        impl Bits for $T {
            const BITS: NBits = <$T>::BITS as NBits;
            const ZERO: Self = 0;
            // const ONE:Self=1;
        }
    )*};
}

impl_Bits!(/*u8, u16,*/ u32, u64 /*, u128*/);

pub trait Float: Copy + PartialEq + PartialOrd {
    /// Type for the bit representation of `Self`.
    type Bits: Bits;
    /// Type for the the biased exponent.
    type BiasedExponent;
    /// Type for the true (unbiased) signed exponent.
    type Exponent;
    /// Type for the significand, both with and without implicit bit.
    type Significand;

    /// The number of bits.
    const BITS: NBits;
    /// The number of bits of the exponent.
    const EXP_BITS: NBits;
    /// The number of bits of the stored significand (not counting the implicit bit).
    const SIGNIF_BITS: NBits;

    /// The mask for the sign bit.
    const SIGN_MASK: Self::Bits;
    /// The mask for the biased exponent bits.
    const EXP_MASK: Self::Bits;
    /// The mask for the stored significand bits (no implicit bit).
    const SIGNIF_MASK: Self::Bits;

    /// The exponent bias.
    const EXP_BIAS: Self::Exponent;

    /// Convert to the underlying bits representation.
    fn to_bits(self) -> Self::Bits;
    /// Convert from the underlying bits representation.
    fn from_bits(b: Self::Bits) -> Self;

    #[inline]
    fn sign_bit(self) -> bool {
        !(self.to_bits() & Self::SIGN_MASK).is_zero()
    }

    fn biased_exponent(self) -> Self::BiasedExponent;
    fn exponent(self) -> Self::Exponent;

    fn stored_significand(self) -> Self::Significand;
    fn significand(self) -> Self::Significand;
    fn fraction(self) -> Self::Significand {
        self.stored_significand()
    }

    fn explain(self)
    where
        Self: fmt::Debug,
        Self::BiasedExponent: fmt::Debug,
        Self::Exponent: fmt::Debug,
        Self::Significand: fmt::Debug,
    {
        println!("value = {:?}", self);
        println!(
            "bits: {:0width$b}",
            self.to_bits(),
            width = Self::Bits::BITS as usize
        );
        println!(
            "      ±{:^<e$}{:_<f$}",
            "",
            "",
            e = Self::EXP_BITS as usize,
            f = Self::SIGNIF_BITS as usize
        );
        println!("sign: {}", if self.sign_bit() { "-" } else { "+" });
        println!(
            "exponent = {:?} - {:?} = {:?}",
            self.biased_exponent(),
            Self::EXP_BIAS,
            self.exponent()
        );
        println!(
            "significand = 2^{:?} + {:?} = {:?}",
            Self::SIGNIF_BITS,
            self.stored_significand(),
            self.significand()
        );
    }
}

macro_rules! impl_Float {
    ($F:ty, $Bits:ty, $BiasedExponent:ty, $Exponent:ty, $Significand:ty; $exp_bits:expr, $signif_bits:expr) => {
        impl Float for $F {
            type Bits = $Bits;
            type BiasedExponent = $BiasedExponent;
            type Exponent = $Exponent;
            type Significand = $Significand;

            const BITS: NBits = Self::Bits::BITS as NBits;
            const EXP_BITS: NBits = $exp_bits;
            const SIGNIF_BITS: NBits = $signif_bits;

            const SIGN_MASK: Self::Bits = 1 << (Self::EXP_BITS + Self::SIGNIF_BITS);
            const EXP_MASK: Self::Bits = (1 << Self::EXP_BITS) - 1 << Self::SIGNIF_BITS;
            const SIGNIF_MASK: Self::Bits = (1 << Self::SIGNIF_BITS) - 1;

            const EXP_BIAS: Self::Exponent = ((1 << Self::EXP_BITS - 1) - 1);

            #[inline]
            fn to_bits(self) -> Self::Bits {
                self.to_bits()
            }

            #[inline]
            fn from_bits(b: Self::Bits) -> Self {
                Self::from_bits(b)
            }

            #[inline]
            fn biased_exponent(self) -> Self::BiasedExponent {
                ((self.to_bits() & Self::EXP_MASK) >> Self::SIGNIF_BITS) as Self::BiasedExponent
            }

            #[inline]
            fn exponent(self) -> Self::Exponent {
                (self.biased_exponent() as Self::Exponent).wrapping_sub(Self::EXP_BIAS)
            }

            #[inline]
            fn stored_significand(self) -> Self::Significand {
                self.to_bits() & Self::SIGNIF_MASK as Self::Significand
            }

            #[inline]
            fn significand(self) -> Self::Significand {
                // Fixme
                self.stored_significand() + (1 << Self::SIGNIF_BITS)
            }
        }
    };
}

impl_Float!(f32, u32, u8, i16, u32; 8, 23);
impl_Float!(f64, u64, u16, i16, u64; 11, 52);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
