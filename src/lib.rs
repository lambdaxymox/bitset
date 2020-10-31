#![allow(clippy::redundant_field_names)]
#![no_std]
extern crate core;


use core::ops;


pub struct BitSet {
    data: u128,
}

impl BitSet {
    #[inline]
    pub const fn new() -> BitSet {
        BitSet {
            data: 0,
        }
    }

    #[inline]
    pub const fn from_u64(value: u64) -> BitSet {
        BitSet {
            data: value as u128
        }
    }

    #[inline]
    pub const fn from_u128(value: u128) -> BitSet {
        BitSet {
            data: value
        }
    }

    #[inline]
    pub fn test(&self, position: usize) -> Option<bool> {
        if position < self.capacity() {
            Some(self.data & (1 << position) != 0)
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.data.count_ones() as usize
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        128
    }

    #[inline]
    pub fn all(&self) -> bool {
        self.data == 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128
    }

    #[inline]
    pub fn none(&self) -> bool {
        self.data == 0
    }

    #[inline]
    pub fn any(&self) -> bool {
        self.data != 0
    }

    pub fn flip_all(&mut self) {
        self.data ^= 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128;
    }

    pub fn flip(&mut self, position: usize) -> Option<()> {
        if position < self.capacity() {
            self.data ^= 1 << position;
            
            Some(())
        } else {
            None
        }
        
    }
    
    pub fn set_all(&mut self) {
        self.data = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_u128;
    }

    pub fn set(&mut self, position: usize, value: bool) -> Option<()> {
        if position < self.capacity()  {
            let mask: u128 = 1 << position;
            if value {
                self.data |= mask;
            } else {
                self.data &= mask.reverse_bits();
            }

            Some(())
        } else {
            None
        }

    }

    pub fn get(&self, position: usize) -> bool {
        if position < self.capacity() {
            if (self.data & (1 << position) ) != 0 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        if (self.data & (0xFFFF_FFFF_FFFF_FFFF_u128 << 64)) == 0 {
            Some(self.data as u64)
        } else {
            None
        }
    }

    pub fn to_u128(&self) -> Option<u128> {
        Some(self.data)
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

impl ops::Shl<BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << other.data;

        bitset
    }
}

impl ops::Shl<&BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, other: &BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << other.data;

        bitset
    }
}

impl ops::Shl<BitSet> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << other.data;

        bitset
    }
}

impl<'a, 'b> ops::Shl<&'a BitSet> for &'b BitSet {
    type Output = BitSet;

    #[inline]
    fn shl(self, other: &'a BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data << other.data;

        bitset
    }
}

impl ops::Shr<BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> other.data;

        bitset
    }
}

impl ops::Shr<&BitSet> for BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, other: &BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> other.data;

        bitset
    }
}

impl ops::Shr<BitSet> for &BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, other: BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> other.data;

        bitset
    }
}

impl<'a, 'b> ops::Shr<&'a BitSet> for &'b BitSet {
    type Output = BitSet;

    #[inline]
    fn shr(self, other: &'a BitSet) -> Self::Output {
        let mut bitset = BitSet::new();
        bitset.data = self.data >> other.data;

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

impl ops::ShlAssign<BitSet> for BitSet {
    #[inline]
    fn shl_assign(&mut self, other: BitSet) {
        self.data <<= other.data;
    }
}

impl ops::ShlAssign<&BitSet> for BitSet {
    #[inline]
    fn shl_assign(&mut self, other: &BitSet) {
        self.data <<= other.data;
    }
}

impl ops::ShrAssign<BitSet> for BitSet {
    #[inline]
    fn shr_assign(&mut self, other: BitSet) {
        self.data <<= other.data;
    }
}

impl ops::ShrAssign<&BitSet> for BitSet {
    #[inline]
    fn shr_assign(&mut self, other: &BitSet) {
        self.data <<= other.data;
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

