
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
