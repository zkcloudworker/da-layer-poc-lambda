/**
 * MinimalField trait implementation `Fp` which only depends on an `FpBackend` trait
 *
 * Most of this code was copied over from ark_ff::Fp
 */
use crate::pasta::wasm_friendly::bigint32::BigInt;
use ark_ff::{One, Zero, Field};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Read, SerializationError, Valid, Validate, Flags,
    Write,
};
use derivative::Derivative;
use num_bigint::BigUint;
use std::{
    marker::PhantomData,
    cmp::Ordering,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use super::minimal_field::MinimalField;

pub trait FpBackend<const N: usize>: Send + Sync + 'static + Sized {
    const MODULUS: BigInt<N>;
    const ZERO: BigInt<N>;
    const ONE: BigInt<N>;

    fn add_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);
    fn mul_assign(a: &mut Fp<Self, N>, b: &Fp<Self, N>);

    /// Construct a field element from an integer in the range
    /// `0..(Self::MODULUS - 1)`. Returns `None` if the integer is outside
    /// this range.
    fn from_bigint(x: BigInt<N>) -> Option<Fp<Self, N>>;
    fn to_bigint(x: Fp<Self, N>) -> BigInt<N>;

    fn pack(x: Fp<Self, N>) -> Vec<u64>;
}

/// Represents an element of the prime field F_p, where `p == P::MODULUS`.
/// This type can represent elements in any field of size at most N * 64 bits.
#[derive(Derivative)]
#[derivative(
    Default(bound = ""),
    Hash(bound = ""),
    Copy(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Debug(bound = "")
)]
pub struct Fp<P: FpBackend<N>, const N: usize>(
    pub BigInt<N>,
    #[derivative(Debug = "ignore")]
    #[doc(hidden)]
    pub PhantomData<P>,
);

impl<P: FpBackend<N>, const N: usize> Clone for Fp<P, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: FpBackend<N>, const N: usize> Fp<P, N> {
    pub fn new(bigint: BigInt<N>) -> Self {
        Fp(bigint, Default::default())
    }

    #[inline]
    pub fn from_bigint(r: BigInt<N>) -> Option<Self> {
        P::from_bigint(r)
    }
    #[inline]
    pub fn into_bigint(self) -> BigInt<N> {
        P::to_bigint(self)
    }

    pub fn to_bytes_le(self) -> Vec<u8> {
        let chunks = P::pack(self).into_iter().map(|x| x.to_le_bytes());
        let mut bytes = Vec::with_capacity(chunks.len() * 8);
        for chunk in chunks {
            bytes.extend_from_slice(&chunk);
        }
        bytes
    }
}

// coerce into Fp from either BigInt<N> or [u32; N]

impl<P: FpBackend<N>, const N: usize> From<BigInt<N>> for Fp<P, N> {
    fn from(val: BigInt<N>) -> Self {
        Fp::from_bigint(val).unwrap()
    }
}

impl<P: FpBackend<N>, const N: usize> From<[u32; N]> for Fp<P, N> {
    fn from(val: [u32; N]) -> Self {
        Fp::from_bigint(BigInt(val)).unwrap()
    }
}

// field

impl<P: FpBackend<N>, const N: usize> MinimalField for Fp<P, N> {
    fn square_in_place(&mut self) -> &mut Self {
        // implemented with mul_assign for now
        let self_copy = *self;
        self.mul_assign(&self_copy);
        self
    }
}

// add, zero

impl<P: FpBackend<N>, const N: usize> Zero for Fp<P, N> {
    #[inline]
    fn zero() -> Self {
        Fp::new(P::ZERO)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<'a, P: FpBackend<N>, const N: usize> AddAssign<&'a Self> for Fp<P, N> {
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        P::add_assign(self, other)
    }
}
impl<P: FpBackend<N>, const N: usize> Add<Self> for Fp<P, N> {
    type Output = Self;

    #[inline]
    fn add(mut self, other: Self) -> Self {
        self.add_assign(&other);
        self
    }
}
impl<'a, P: FpBackend<N>, const N: usize> Add<&'a Fp<P, N>> for Fp<P, N> {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self.add_assign(other);
        self
    }
}

// mul, one

impl<P: FpBackend<N>, const N: usize> One for Fp<P, N> {
    #[inline]
    fn one() -> Self {
        Fp::new(P::ONE)
    }

    #[inline]
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}
impl<'a, P: FpBackend<N>, const N: usize> MulAssign<&'a Self> for Fp<P, N> {
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        P::mul_assign(self, other)
    }
}
impl<P: FpBackend<N>, const N: usize> Mul<Self> for Fp<P, N> {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: Self) -> Self {
        self.mul_assign(&other);
        self
    }
}
impl<'a, P: FpBackend<N>, const N: usize> Mul<&'a Fp<P, N>> for Fp<P, N> {
    type Output = Self;

    #[inline]
    fn mul(mut self, other: &Self) -> Self {
        self.mul_assign(other);
        self
    }
}

// (de)serialization

impl<P: FpBackend<N>, const N: usize> CanonicalSerialize for Fp<P, N> {
    #[inline]
    fn serialize_with_mode<W: Write>(
        &self,
        writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        self.0.serialize_with_mode(writer, compress)
    }

    #[inline]
    fn serialized_size(&self, compress: Compress) -> usize {
        self.0.serialized_size(compress)
    }
}

impl<P: FpBackend<N>, const N: usize> Valid for Fp<P, N> {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl<P: FpBackend<N>, const N: usize> CanonicalDeserialize for Fp<P, N> {
    fn deserialize_with_mode<R: Read>(
        reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Self, SerializationError> {
        Self::from_bigint(BigInt::deserialize_with_mode(reader, compress, validate)?)
            .ok_or(SerializationError::InvalidData)
    }
}

// display

impl<P: FpBackend<N>, const N: usize> From<Fp<P, N>> for BigUint {
    #[inline]
    fn from(val: Fp<P, N>) -> BigUint {
        BigUint::from_bytes_le(&val.to_bytes_le())
    }
}

impl<P: FpBackend<N>, const N: usize> std::fmt::Display for Fp<P, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        BigUint::from(*self).fmt(f)
    }
}


////////////////////////////////////////////////////////////////////////////
// Field
////////////////////////////////////////////////////////////////////////////

// TODO one needs to implement these traits:
//
//    + Neg<Output = Self>
//    + UniformRand
//    + Zeroize
//    + CanonicalSerializeWithFlags
//    + CanonicalDeserializeWithFlags
//    + Sub<Self, Output = Self>
//    + Div<Self, Output = Self>
//    + AddAssign<Self>
//    + SubAssign<Self>
//    + MulAssign<Self>
//    + DivAssign<Self>
//    + for<'a> Sub<&'a Self, Output = Self>
//    + for<'a> Div<&'a Self, Output = Self>
//    + for<'a> SubAssign<&'a Self>
//    + for<'a> DivAssign<&'a Self>
//    + for<'a> Add<&'a mut Self, Output = Self>
//    + for<'a> Sub<&'a mut Self, Output = Self>
//    + for<'a> Mul<&'a mut Self, Output = Self>
//    + for<'a> Div<&'a mut Self, Output = Self>
//    + for<'a> AddAssign<&'a mut Self>
//    + for<'a> SubAssign<&'a mut Self>
//    + for<'a> MulAssign<&'a mut Self>
//    + for<'a> DivAssign<&'a mut Self>
//    + core::iter::Sum<Self>
//    + for<'a> core::iter::Sum<&'a Self>
//    + core::iter::Product<Self>
//    + for<'a> core::iter::Product<&'a Self>


impl<P: FpBackend<N>, const N: usize> Ord for Fp<P, N> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.into_bigint().cmp(&other.into_bigint())
    }
}

impl<P: FpBackend<N>, const N: usize> PartialOrd for Fp<P, N> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



impl<P: FpBackend<N>, const N: usize> From<u128> for Fp<P,N> {
    fn from(other: u128) -> Self {
        todo!()
    }
}


impl<P: FpBackend<N>, const N: usize> From<u64> for Fp<P,N> {
    fn from(other: u64) -> Self {
        todo!()
    }
}

impl<P: FpBackend<N>, const N: usize> From<u32> for Fp<P,N> {
    fn from(other: u32) -> Self {
        todo!()
    }
}

impl<P: FpBackend<N>, const N: usize> From<u16> for Fp<P,N> {
    fn from(other: u16) -> Self {
        todo!()
    }
}

impl<P: FpBackend<N>, const N: usize> From<u8> for Fp<P,N> {
    fn from(other: u8) -> Self {
        todo!()
    }
}

impl<P: FpBackend<N>, const N: usize> From<bool> for Fp<P,N> {
    fn from(other: bool) -> Self {
        todo!()
    }
}

impl<P: FpBackend<N>, const N: usize> Field for Fp<P,N> {
    type BasePrimeField = Self;
    fn extension_degree() -> u64 {
        1
    }
    fn from_base_prime_field_elems(elems: &[Self::BasePrimeField]) -> Option<Self> {
        todo!()
        //if elems.len() != (Self::extension_degree() as usize) {
        //    return None;
        //}
        //Some(elems[0])
    }
    #[inline]
    fn double(&self) -> Self {
        todo!()
        //let mut temp = *self;
        //temp.double_in_place();
        //temp
    }
    #[inline]
    fn double_in_place(&mut self) -> &mut Self {
        todo!()
        //self.0.mul2();
        //self.reduce();
        //self
    }
    #[inline]
    fn characteristic() -> [u64; 4] {
        todo!()
        //C::MODULUS.to_64x4()
    }
    #[inline]
    fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)> {
        todo!()
        //if F::BIT_SIZE > 8 {
        //    return None;
        //} else {
        //    let mut result_bytes = [0u8; 4 * 8 + 1];
        //    result_bytes
        //        .iter_mut()
        //        .zip(bytes)
        //        .for_each(|(result, input)| {
        //            *result = *input;
        //        });
        //    let last_limb_mask = (u64::MAX >> C::REPR_SHAVE_BITS).to_le_bytes();
        //    let mut last_bytes_mask = [0u8; 9];
        //    last_bytes_mask[..8].copy_from_slice(&last_limb_mask);
        //    let output_byte_size = buffer_byte_size(C::MODULUS_BITS as usize + F::BIT_SIZE);
        //    let flag_location = output_byte_size - 1;
        //    let flag_location_in_last_limb = flag_location - (8 * (4 - 1));
        //    let last_bytes = &mut result_bytes[8 * (4 - 1)..];
        //    let flags_mask = u8::MAX.checked_shl(8 - (F::BIT_SIZE as u32)).unwrap_or(0);
        //    let mut flags: u8 = 0;
        //    for (i, (b, m)) in last_bytes.iter_mut().zip(&last_bytes_mask).enumerate() {
        //        if i == flag_location_in_last_limb {
        //            flags = *b & flags_mask;
        //        }
        //        *b &= m;
        //    }
        //    Self::deserialize(&result_bytes[..(4 * 8)])
        //        .ok()
        //        .and_then(|f| F::from_u8(flags).map(|flag| (f, flag)))
        //}
    }
    #[inline(always)]
    fn square(&self) -> Self {
        todo!()
        //let mut temp = self.clone();
        //temp.square_in_place();
        //temp
    }
    #[inline(always)]
    fn square_in_place(&mut self) -> &mut Self {
        todo!()
        //self.const_square();
        //self
    }
    #[inline]
    fn inverse(&self) -> Option<Self> {
        todo!()
        //if self.is_zero() {
        //    None
        //} else {
        //    let one = BigInteger256::from(1);
        //    let mut u = self.0;
        //    let mut v = C::MODULUS;
        //    let mut b = Self(C::R2, PhantomData);
        //    let mut c = Self::zero();
        //    while u != one && v != one {
        //        while u.is_even() {
        //            u.div2();
        //            if b.0.is_even() {
        //                b.0.div2();
        //            } else {
        //                b.0.add_nocarry(&C::MODULUS);
        //                b.0.div2();
        //            }
        //        }
        //        while v.is_even() {
        //            v.div2();
        //            if c.0.is_even() {
        //                c.0.div2();
        //            } else {
        //                c.0.add_nocarry(&C::MODULUS);
        //                c.0.div2();
        //            }
        //        }
        //        if v < u {
        //            u.sub_noborrow(&v);
        //            b.sub_assign(&c);
        //        } else {
        //            v.sub_noborrow(&u);
        //            c.sub_assign(&b);
        //        }
        //    }
        //    if u == one {
        //        Some(b)
        //    } else {
        //        Some(c)
        //    }
        //}
    }
    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        if let Some(inverse) = self.inverse() {
            *self = inverse;
            Some(self)
        } else {
            None
        }
    }
    #[inline]
    fn frobenius_map(&mut self, _: usize) {
        todo!()
    }

}
