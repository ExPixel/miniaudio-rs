macro_rules! impl_void_debug {
    ($Type:ty, $Name:expr) => {
        impl std::fmt::Debug for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!($Name, "{{ /* omitted */ }}"))
            }
        }
    };

    ($Type:ty) => {
        impl_void_debug!($Type, stringify!($Type));
    };
}

macro_rules! impl_default {
    ($Type:ty, $Value:expr) => {
        impl Default for $Type {
            fn default() -> $Type {
                $Value
            }
        }
    };
}

macro_rules! impl_bitfield {
    ($ForType:ty, $BitField:ident, $Set:ident, $Get:ident, $Mask:expr) => {
        impl $ForType {
            #[inline]
            pub fn $Set(&mut self, value: bool) {
                if value {
                    self.$BitField |= $Mask;
                } else {
                    self.$BitField &= !($Mask);
                }
            }

            #[inline]
            pub fn $Get(&self) -> bool {
                (self.$BitField & $Mask) != 0
            }
        }
    };

    ($ForType:ty, $BitField:ident, $Set:ident, $Get:ident, $Mask:expr, $Doc:expr) => {
        impl $ForType {
            #[doc = $Doc]
            #[inline]
            pub fn $Set(&mut self, value: bool) {
                if value {
                    self.$BitField |= $Mask;
                } else {
                    self.$BitField &= !($Mask);
                }
            }

            #[doc = $Doc]
            #[inline]
            pub fn $Get(&self) -> bool {
                (self.$BitField & $Mask) != 0
            }
        }
    };
}

macro_rules! impl_use_simd_bitfields {
    ($ForType:ty, $SIMDField:ident, $Offset:expr) => {
        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_sse2,
            use_sse2,
            USE_SSE2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_avx2,
            use_avx2,
            USE_AVX2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_avx512,
            use_avx512,
            USE_AVX512_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_use_neon,
            use_neon,
            USE_NEON_MASK << $Offset
        );
    };
}

macro_rules! impl_no_simd_bitfields {
    ($ForType:ty, $SIMDField:ident, $Offset:expr) => {
        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_sse2,
            no_sse2,
            NO_SSE2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_avx2,
            no_avx2,
            NO_AVX2_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_avx512,
            no_avx512,
            NO_AVX512_MASK << $Offset
        );

        impl_bitfield!(
            $ForType,
            $SIMDField,
            set_no_neon,
            no_neon,
            NO_NEON_MASK << $Offset
        );
    };
}

mod base;
mod biquad_filtering;
mod data_conversion;
mod low_pass_filtering;

pub use base::*;
pub use biquad_filtering::*;
pub use data_conversion::*;
pub use low_pass_filtering::*;
