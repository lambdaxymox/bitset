/*!
# BitSet Library

## Introduction
**bitset** is a crate introducing a data type representing sets of bits.

## Getting Started
To use the library as a dependency in your project, Add **bitset** to your 
`Cargo.toml` file

```ignore
[dependencies]
bitset = "0.2.4"
```

After that, place the crate declaration in your project's `main.rs` 
or `lib.rs` file

```rust
extern crate bitset;
```

## Features
**bitset** is a library adds the `BitSet` data type to a system. This allows
one to use a sequence of arbitrary many bits and apply standard logic operations
to them. One possible use case would be for tracking data dependencies between 
subsystems in a larger software system using the bit set to track what data 
components each data type has a in data-oriented design fashion. One can apply
the standard bitwise logic operations to bit sets such as logical AND, logical 
OR, logical XOR, SHIFT LEFT, SHIFT RIGHT, and logical NEGATION. One can also 
query, test, set and flip individual bits.

## Limitations
The main limitation of the **bitset** crate is that it only supports a bit set 
capacity of 128 bits. This is the largest possible unsigned integer that Rust's
type systems currently allows. This limitation will be removed in the future
when Rust gets const generics.
*/

use std::fmt;
use std::ops;


/// A fixed-size sequence of N bits. Bit sets can be transformed by 
/// standard logic operators and converted to and from integers.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitSet {
    data: u128,
}

impl BitSet {
    /// Construct a new bit set with all bits set to `false`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::new();
    ///
    /// for i in 0..bitset.capacity() {
    ///     assert_eq!(bitset.test(i), false); 
    /// }
    /// ```
    #[inline]
    pub const fn new() -> BitSet {
        BitSet {
            data: 0,
        }
    }

    /// Construct a new bit set from an unsigned integer.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::from_u64(0xFFFF_FFFF_FFFF_FFFF);
    /// for i in 0..64 {
    ///     assert_eq!(bitset.test(i), true);
    /// }
    /// 
    /// for i in 64..bitset.capacity() {
    ///     assert_eq!(bitset.test(i), false);  
    /// }
    /// ```
    #[inline]
    pub const fn from_u64(value: u64) -> BitSet {
        BitSet {
            data: value as u128
        }
    }

    /// Construct a new bit set from an unsigned integer.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::from_u128(0xFFFF_FFFF_FFFF_FFFF);
    /// for i in 0..64 {
    ///     assert_eq!(bitset.test(i), true);
    /// }
    /// 
    /// for i in 64..bitset.capacity() {
    ///     assert_eq!(bitset.test(i), false);  
    /// }
    /// ```
    #[inline]
    pub const fn from_u128(value: u128) -> BitSet {
        BitSet {
            data: value
        }
    }

    /// Test whether the bit in the input position is set.
    ///
    /// If the position `position` exceeds the capacity of the bit set,
    /// the function returns `false`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::from_u64(0b0010);
    ///
    /// assert_eq!(bitset.test(0), false);
    /// assert_eq!(bitset.test(1), true);
    /// assert_eq!(bitset.test(2), false);
    /// assert_eq!(bitset.test(3), false);
    /// ```
    #[inline]
    pub fn test(&self, position: usize) -> bool {
        if position < self.capacity() {
            self.data & (1 << position) != 0
        } else {
            false
        }
    }

    /// Count up the number of bits in the bit set that are set to true.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::from_u64(0x0000_0000_0000_FFFF);
    ///
    /// assert_eq!(bitset.count(), 16);
    /// ```
    #[inline]
    pub fn count(&self) -> usize {
        self.data.count_ones() as usize
    }

    /// Return the maximum number of bits that this bit set can hold.
    #[inline]
    pub const fn capacity(&self) -> usize {
        128
    }

    /// Test whether all the bits in a bit set are set to true.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// 
    #[inline]
    pub fn all(&self) -> bool {
        self.data == 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128
    }

    /// Test whether none of the bits in a bit set are set to true.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset = BitSet::new();
    /// 
    /// assert!(bitset.none());
    /// ```
    #[inline]
    pub fn none(&self) -> bool {
        self.data == 0
    }

    /// Test whether any of the bits in a bit set are set to true.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let bitset1 = BitSet::new();
    /// assert!(!bitset1.any());
    /// let bitset2 = BitSet::from_u64(1);
    /// assert!(bitset2.any());
    /// ```
    #[inline]
    pub fn any(&self) -> bool {
        self.data != 0
    }

    /// Flip all the bits in a bit set.
    ///
    /// If a bit is set to `true`, it will be set to `false` after calling 
    /// `flip_all`. Similarly, if a bit is set to `false`, it will be set to `true`
    /// after calling `flip_all`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let mut result = BitSet::from_u128(0xFFFF_0000_FFFF_0000_FFFF_0000_FFFF_0000);
    /// let expected   = BitSet::from_u128(0x0000_FFFF_0000_FFFF_0000_FFFF_0000_FFFF);
    /// result.flip_all();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn flip_all(&mut self) {
        self.data ^= 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128;
    }

    /// Flip an individual bit in a bit set.
    ///
    /// If the bit at position `position` in the bit set is `true`, it will be
    /// set to `false`. If the bit as position `position` is set to `false`, it
    /// will be set to `true`.
    ///
    /// If the bit position exceeds the capacity of the bit set, the function 
    /// returns `None`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let mut bitset = BitSet::new();
    /// assert_eq!(bitset.get(3), Some(false));
    /// bitset.flip(3); 
    /// assert_eq!(bitset.get(3), Some(true));
    /// ``` 
    pub fn flip(&mut self, position: usize) -> Option<()> {
        if position < self.capacity() {
            self.data ^= 1 << position;
            
            Some(())
        } else {
            None
        }
    }
    
    /// Set all the bits in a bit set to `true` regardless of their current
    /// value.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet,
    /// # };
    /// #
    /// let mut bitset = BitSet::new();
    /// assert!(bitset.none());
    /// bitset.set_all();
    /// assert!(bitset.all());
    /// ```
    #[inline]
    pub fn set_all(&mut self) {
        self.data = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128;
    }

    /// Set all the bits in a bit set to `false` regardless of their current
    /// value.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let data = 0xDEAD_BEEF_DEAD_BEEF;
    /// let mut bitset = BitSet::from_u64(data);
    /// assert!(bitset.any());
    /// bitset.reset_all();
    /// assert!(bitset.none());
    /// ```
    #[inline]
    pub fn reset_all(&mut self) {
        self.data = 0;
    }

    /// Set the bit as position `position` to the value `value`.
    ///
    /// The function returns `None` if `position` is outside the capacity of the
    /// bit set.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet,  
    /// # };
    /// #
    /// let mut bitset = BitSet::new();
    /// bitset.set(1, true);
    /// assert_eq!(bitset.get(1), Some(true));
    /// bitset.set(1, true);
    /// assert_eq!(bitset.get(1), Some(true));
    /// bitset.set(1, false);
    /// assert_eq!(bitset.get(1), Some(false));
    /// ```
    pub fn set(&mut self, position: usize, value: bool) -> Option<()> {
        if position < self.capacity()  {
            let mask: u128 = 1 << position;
            if value {
                self.data |= mask;
            } else {
                self.data &= !mask;
            }

            Some(())
        } else {
            None
        }

    }

    /// Get the current value of the bit at position `position` in the bit set.
    ///
    /// The function returns `None` if `position` is outside the capacity of the
    /// bit set.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet,   
    /// # };
    /// #
    /// let bitset = BitSet::from_u64(0b110);
    /// assert_eq!(bitset.get(0), Some(false));
    /// assert_eq!(bitset.get(1), Some(true));
    /// assert_eq!(bitset.get(2), Some(true));
    /// assert_eq!(bitset.get(3), Some(false));
    /// 
    /// // Return none if the position exceeds the capacity of the bitset.
    /// assert_eq!(bitset.get(bitset.capacity()), None);
    /// ```
    pub fn get(&self, position: usize) -> Option<bool> {
        if position < self.capacity() {
            if (self.data & (1 << position) ) != 0 {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }

    /// Convert a bit set to a 64-bit integer if the bit set will fit inside
    /// the integer.
    ///
    /// The functions returns `None` if the index of the highest bit set the `true`
    /// exceeds the width of a `u64`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// // A bitset that will not fit in a u64.
    /// let bitset1 = BitSet::from_u128(0x0000_0000_0000_0001_0FFF_FFFF_FFFF_FFFF);
    /// assert!(bitset1.to_u64().is_none());
    ///
    /// // A bitset that will fit inside a u64.
    /// let bitset2 = BitSet::from_u128(0x0000_0000_0000_0000_0FFF_FFFF_FFFF_FFFF);
    /// assert_eq!(bitset2.to_u64(), Some(0x0FFF_FFFF_FFFF_FFFF));
    /// ```
    pub fn to_u64(&self) -> Option<u64> {
        if (self.data & (0xFFFF_FFFF_FFFF_FFFF_u128 << 64)) == 0 {
            Some(self.data as u64)
        } else {
            None
        }
    }

    /// Convert a bit set to a 128-bit integer if the bit set will fit inside the integer.
    ///
    /// The functions returns `None` if the index of the highest bit set the `true`
    /// exceeds the width of a `u128`.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet, 
    /// # };
    /// #
    /// let data: u64 = 0xDEAD_BEEF_DEAD_BEEF;
    /// let bitset = BitSet::from_u64(data);
    /// let expected = Some(data as u128);
    /// let result = bitset.to_u128();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn to_u128(&self) -> Option<u128> {
        Some(self.data)
    }

    /// Convert a bit set to a string of ones and zeros.
    ///
    /// ## Example
    ///
    /// ```
    /// # use bitset::{
    /// #     BitSet 
    /// # };
    /// #
    /// let bitset = BitSet::from_u64(0xDEAD_BEEF);
    /// let expected = "\
    ///     00000000000000000000000000000000\
    ///     00000000000000000000000000000000\
    ///     00000000000000000000000000000000\
    ///     11011110101011011011111011101111";
    /// let result = bitset.as_string();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn as_string(&self) -> String {
        let mut st = String::with_capacity(self.capacity());
        for i in (0..self.capacity()).rev() {
            if self.test(i) {
                st.push('1');
            } else {
                st.push('0')
            }
        }

        st
    }
}

impl fmt::Display for BitSet {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "BitSet [{:#X}]", self.data)
    }
}

impl ops::BitAnd<BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitand(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data & other.data;

        bitset
    }
}

impl ops::BitAnd<&BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitand(self, other: &BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data & other.data;

        bitset
    }
}

impl ops::BitAnd<BitSet> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn bitand(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data & other.data;

        bitset
    }
}

impl<'a, 'b> ops::BitAnd<&'a BitSet> for &'b BitSet {
    type Output = BitSet;

    #[inline]
    fn bitand(self, other: &'a BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data & other.data;

        bitset
    }
}

impl ops::BitOr<BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitor(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data | other.data;

        bitset
    }
}

impl ops::BitOr<&BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitor(self, other: &BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data | other.data;

        bitset
    }
}

impl ops::BitOr<BitSet> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn bitor(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data | other.data;

        bitset
    }
}

impl<'a, 'b> ops::BitOr<&'a BitSet> for &'b BitSet {
    type Output = BitSet;

    #[inline]
    fn bitor(self, other: &'a BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data | other.data;

        bitset
    }
}

impl ops::BitXor<BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitxor(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data ^ other.data;

        bitset
    }
}

impl ops::BitXor<&BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn bitxor(self, other: &BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data ^ other.data;

        bitset
    }
}

impl ops::BitXor<BitSet> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn bitxor(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data ^ other.data;

        bitset
    }
}

impl<'a, 'b> ops::BitXor<&'a BitSet> for &'b BitSet {
    type Output = BitSet;

    #[inline]
    fn bitxor(self, other: &'a BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data ^ other.data;

        bitset
    }
}

impl ops::Shl<usize> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, amount: usize) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << amount;

        bitset
    }
}

impl ops::Shl<usize> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, amount: usize) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << amount;

        bitset
    }
}

impl ops::Shr<usize> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, amount: usize) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> amount;

        bitset
    }
}

impl ops::Shr<usize> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, amount: usize) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> amount;

        bitset
    }
}

impl ops::BitAndAssign<BitSet> for BitSet {
    #[inline]
    fn bitand_assign(&mut self, other: BitSet) {
        self.data &= other.data;
    }
}

impl ops::BitAndAssign<&BitSet> for BitSet {
    #[inline]
    fn bitand_assign(&mut self, other: &BitSet) {
        self.data &= other.data;
    }
}

impl ops::BitOrAssign<BitSet> for BitSet {
    #[inline]
    fn bitor_assign(&mut self, other: BitSet) {
        self.data |= other.data;
    }
}

impl ops::BitOrAssign<&BitSet> for BitSet {
    #[inline]
    fn bitor_assign(&mut self, other: &BitSet) {
        self.data |= other.data;
    }
}

impl ops::BitXorAssign<BitSet> for BitSet {
    #[inline]
    fn bitxor_assign(&mut self, other: BitSet) {
        self.data ^= other.data;
    }
}

impl ops::BitXorAssign<&BitSet> for BitSet {
    #[inline]
    fn bitxor_assign(&mut self, other: &BitSet) {
        self.data ^= other.data;
    }
}

impl ops::ShlAssign<usize> for BitSet {
    #[inline]
    fn shl_assign(&mut self, amount: usize) {
        self.data <<= amount;
    }
}

impl ops::ShrAssign<usize> for BitSet {
    #[inline]
    fn shr_assign(&mut self, amount: usize) {
        self.data >>= amount;
    }
}

impl ops::Not for BitSet {
    type Output = BitSet;

    #[inline]
    fn not(self) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = !self.data;

        bitset
    }
}

impl ops::Not for &BitSet {
    type Output = BitSet;

    #[inline]
    fn not(self) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = !self.data;

        bitset
    }
}

