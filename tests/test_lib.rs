extern crate bitset;


use bitset::{
    BitSet,
};


#[test]
fn test_new() {
    let bitset = BitSet::new();

    assert_eq!(bitset.to_u128(), Some(0));
}

#[test]
fn test_from_u64_to_u64() {
    let data = 0xFFFF_FFFF_FFFF_FFFF;
    let bitset = BitSet::from_u64(data);
    let expected = Some(data);
    let result = bitset.to_u64();

    assert_eq!(result, expected);
}

#[test]
fn test_from_u128_to_u128() {
    let data = 0xFFFF_FFFF_FFFF_FFFF;
    let bitset = BitSet::from_u128(data);
    let expected = Some(data);
    let result = bitset.to_u128();

    assert_eq!(result, expected);
}

#[test]
fn test_from_u128_to_u64_fits_inside_u64() {
    let data = 0x0000_0000_0000_0000_8FFF_FFFF_FFFF_FFFF;
    let bitset = BitSet::from_u128(data);
    let expected = Some(0x8FFF_FFFF_FFFF_FFFF);
    let result = bitset.to_u64();

    assert_eq!(result, expected);
}

#[test]
fn test_from_u128_to_u64_too_big() {
    let data = 0x0000_0000_0000_000F_FFFF_FFFF_FFFF_FFFF;
    let bitset = BitSet::from_u128(data);

    assert!(bitset.to_u64().is_none());
}

#[test]
fn test_bitset_test1() {
    let data = 0b11001010;
    let bitset = BitSet::from_u64(data);
    
    assert_eq!(bitset.test(0), false);
    assert_eq!(bitset.test(1), true);
    assert_eq!(bitset.test(2), false);
    assert_eq!(bitset.test(3), true);
    assert_eq!(bitset.test(4), false);
    assert_eq!(bitset.test(5), false);
    assert_eq!(bitset.test(6), true);
    assert_eq!(bitset.test(7), true);
}

#[test]
fn test_bitset_test2() {
    let data = 0b11001010;
    let bitset = BitSet::from_u64(data);
    
    for i in 8..bitset.capacity() {
        assert_eq!(bitset.test(i), false);
    }
}

#[test]
fn test_bitset_test_out_of_bounds() {
    let data = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
    let bitset = BitSet::from_u128(data);

    assert_eq!(bitset.test(bitset.capacity()), false);
}

#[test]
fn test_bitset_count() {
    let data = 0xFFFF_FF00;
    let bitset = BitSet::from_u64(data);

    assert_eq!(bitset.count(), 24);
}

#[test]
fn test_bitset_count_all_zeros() {
    let bitset = BitSet::new();

    assert_eq!(bitset.count(), 0);
}

#[test]
fn test_bitset_count_all_set() {
    let mut bitset = BitSet::new();
    bitset.set_all();

    assert_eq!(bitset.count(), bitset.capacity());
}

#[test]
fn test_bitset_all1() {
    let bitset = BitSet::new();

    assert!(!bitset.all());
}

#[test]
fn test_bitset_all2() {
    let data = 0xFFFF;
    let bitset = BitSet::from_u64(data);

    assert!(!bitset.all());
}

#[test]
fn test_bitset_all3() {
    let mut bitset = BitSet::new();
    bitset.set_all();

    assert!(bitset.all());
}

#[test]
fn test_bitset_none1() {
    let bitset = BitSet::new();

    assert!(bitset.none());
}

#[test]
fn test_bitset_none2() {
    let data = 0xFFFF;
    let bitset = BitSet::from_u64(data);

    assert!(!bitset.none());
}

#[test]
fn test_bitset_any1() {
    let data = 0xF000;
    let bitset = BitSet::from_u64(data);

    assert!(bitset.any());
}

#[test]
fn test_bitset_any2() {
    let bitset = BitSet::new();

    assert!(!bitset.any());
}

#[test]
fn test_bitset_flip_all() {
    let mut result = BitSet::from_u128(0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0);
    let expected = BitSet::from_u128(0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F);
    result.flip_all();

    assert_eq!(result, expected);
}

/// Flipping all the bits twice in a row should return the bits to their
/// values.
#[test]
fn test_bitset_flip_all_twice() {
    let mut result = BitSet::from_u128(0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0);
    let expected = result;
    result.flip_all();
    result.flip_all();

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_flip() {
    let expected = BitSet::from_u64(0x0F);
    let mut result = BitSet::from_u64(0xF0);
    result.flip(0);
    result.flip(1);
    result.flip(2);
    result.flip(3);
    result.flip(4);
    result.flip(5);
    result.flip(6);
    result.flip(7);

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_flip_bits_order_should_not_matter() {
    let data = 0xF0;
    let expected = BitSet::from_u64(0x0F);
    let mut result1 = BitSet::from_u64(data);
    result1.flip(0);
    result1.flip(1);
    result1.flip(2);
    result1.flip(3);
    result1.flip(4);
    result1.flip(5);
    result1.flip(6);
    result1.flip(7);

    let mut result2 = BitSet::from_u64(data);
    result2.flip(3);
    result2.flip(0);
    result2.flip(2);
    result2.flip(7);
    result2.flip(4);
    result2.flip(5);
    result2.flip(6);
    result2.flip(1);

    assert_eq!(result1, expected);
    assert_eq!(result2, expected);
}

/// Flipping a bit twice should return the bit to its original value.
#[test]
fn test_bitset_flip_twice_return_original_value() {
    let data = 0b10;
    let expected = BitSet::from_u64(data);
    let mut result = BitSet::from_u64(data);
    result.flip(1);
    result.flip(1);

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_set_all1() {
    let mut bitset = BitSet::new();
    assert!(bitset.none());

    bitset.set_all();
    assert!(bitset.all());
}

#[test]
fn test_bitset_set_all2() {
    let data = 0xDEAD_BEEF;
    let mut bitset = BitSet::from_u64(data);
    bitset.set_all();

    assert!(bitset.all());
}

#[test]
fn test_bitset_reset_all1() {
    let data = 0;
    let mut bitset = BitSet::from_u128(data);
    assert!(bitset.none());

    bitset.reset_all();
    assert!(bitset.none());
}

#[test]
fn test_bitset_reset_all2() {
    let data = 0xDEAD_BEEF_DEAD_BEEF;
    let mut bitset = BitSet::from_u64(data);
    assert!(bitset.any());
    bitset.reset_all();
    assert!(bitset.none());
}

#[test]
fn test_bitset_set_out_of_bounds() {
    let data = 0b1101;
    let mut bitset = BitSet::from_u64(data);
    let result = bitset.set(bitset.capacity(), true);

    assert!(result.is_none());
}

#[test]
fn test_bitset_set() {
    let mut result = BitSet::from_u64(0b0000);

    result.set(0, true);
    assert_eq!(result, BitSet::from_u64(0b0001));
    result.set(1, true);
    assert_eq!(result, BitSet::from_u64(0b0011));
    result.set(2, true);
    assert_eq!(result, BitSet::from_u64(0b0111));
    result.set(3, true);
    assert_eq!(result, BitSet::from_u64(0b1111));

    result.set(0, false);
    assert_eq!(result, BitSet::from_u64(0b1110));
    /*
    result.set(1, false);
    assert_eq!(result, BitSet::from_u64(0b1100));
    result.set(2, false);
    assert_eq!(result, BitSet::from_u64(0b1000));
    result.set(3, false);
    assert_eq!(result, BitSet::from_u64(0b0000));
    */
}

/// After setting a bit in a bitset with a call to the `set` function, successive
/// called to `set` on the same bit with the same value should not change the 
/// value of that bit until it is set to a new value.
#[test]
fn test_bitset_set_bit_twice() {
    let mut bitset = BitSet::from_u64(0b1101);

    assert!(!bitset.test(1));
    bitset.set(1, true);
    assert!(bitset.test(1));
    bitset.set(1, true);
    assert!(bitset.test(1));

    bitset.set(1, false);
    assert!(!bitset.test(1));
    bitset.set(1, false);
    assert!(!bitset.test(1));
}

#[test]
fn test_bitset_get_out_of_bounds() {
    let bitset = BitSet::from_u64(0b1101);

    assert!(bitset.get(bitset.capacity()).is_none());
}

#[test]
fn test_bitset_get1() {
    let bitset = BitSet::from_u64(0b1101);

    assert_eq!(bitset.get(0), Some(true));
    assert_eq!(bitset.get(1), Some(false));
    assert_eq!(bitset.get(2), Some(true));
    assert_eq!(bitset.get(3), Some(true));
}

#[test]
fn test_bitset_get2() {
    let bitset = BitSet::from_u64(0b1101);

    for i in 4..bitset.capacity() {
        assert_eq!(bitset.get(i), Some(false));
    }
}

#[test]
fn test_bitset_set_and_get() {
    let mut bitset = BitSet::from_u64(0xDEAD_BEEF_DEAD_BEEF);

    for i in 0..bitset.capacity() {
        bitset.set(i, true);
        assert_eq!(bitset.get(i), Some(true));
        bitset.set(i, true);
        assert_eq!(bitset.get(i), Some(true));
        bitset.set(i, false);
        assert_eq!(bitset.get(i), Some(false));
        bitset.set(i, false);
        assert_eq!(bitset.get(i), Some(false));
    }
}

#[test]
fn test_bitset_bitwise_and_pointers_and_values() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);
    let bitset2 = BitSet::from_u64(0xCAFE_BABE_DEAD_BEEF);

    assert_eq!(bitset1 & bitset2, &bitset1 & bitset2);
    assert_eq!(bitset1 & bitset2, bitset1  & &bitset2);
    assert_eq!(bitset1 & bitset2, &bitset1 & &bitset2);
}

#[test]
fn test_bitset_bitwise_or_pointers_and_values() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);
    let bitset2 = BitSet::from_u64(0xCAFE_BABE_DEAD_BEEF);

    assert_eq!(bitset1 | bitset2, &bitset1 | bitset2);
    assert_eq!(bitset1 | bitset2, bitset1  | &bitset2);
    assert_eq!(bitset1 | bitset2, &bitset1 | &bitset2);
}

#[test]
fn test_bitset_bitwise_xor_pointers_and_values() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);
    let bitset2 = BitSet::from_u64(0xCAFE_BABE_DEAD_BEEF);

    assert_eq!(bitset1 ^ bitset2, &bitset1 ^ bitset2);
    assert_eq!(bitset1 ^ bitset2, bitset1  ^ &bitset2);
    assert_eq!(bitset1 ^ bitset2, &bitset1 ^ &bitset2);
}

#[test]
fn test_bitset_shl_pointers_and_values() {
    let bitset = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);

    for i in 0..bitset.capacity() {
        assert_eq!(bitset << i, &bitset << i);
    }
}

#[test]
fn test_bitset_shr_pointers_and_values() {
    let bitset = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);

    for i in 0..bitset.capacity() {
        assert_eq!(bitset >> i, &bitset >> i);
    }
}

#[test]
fn test_bitset_not_pointers_and_values() {
    let bitset = BitSet::from_u64(0xDEAD_BEEF_CAFE_BABE);

    assert_eq!(!bitset, !&bitset);
}

#[test]
fn test_bitset_bitwise_and1() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF);
    let bitset2 = BitSet::from_u64(0xDEA0_0EEF);
    let expected = BitSet::from_u64(0xDEA0_0EEF);
    let result = bitset1 & bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_and2() {
    let bitset1 = BitSet::from_u64(0xF0F0_FF0F);
    let bitset2 = BitSet::from_u64(0xFFFF_F0F0);
    let expected = BitSet::from_u64(0xF0F0_F000);
    let result = bitset1 & bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_and_bitset_is_bitset() {
    let bitset = BitSet::from_u64(0xDEAF_BEEF);
    let result = bitset & bitset;
    let expected = bitset;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_or1() {
    let bitset1 = BitSet::from_u64(0x0000_0000);
    let bitset2 = BitSet::from_u64(0x0000_BEEF);
    let expected = BitSet::from_u64(0x0000_BEEF);
    let result = bitset1 | bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_or2() {
    let bitset1 = BitSet::from_u64(0xF0F0_FF0F);
    let bitset2 = BitSet::from_u64(0x000F_0000);
    let expected = BitSet::from_u64(0xF0FF_FF0F);
    let result = bitset1 | bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_or3() {
    let bitset1 = BitSet::from_u64(0xDEAD_0000);
    let bitset2 = BitSet::from_u64(0x0000_BEEF);
    let expected = BitSet::from_u64(0xDEAD_BEEF);
    let result = bitset1 | bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_or_bitset_is_bitset() {
    let bitset = BitSet::from_u64(0xDEAF_BEEF);
    let result = bitset | bitset;
    let expected = bitset;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_xor1() {
    let bitset1 = BitSet::from_u64(0xDEAD_0000);
    let bitset2 = BitSet::from_u64(0x0000_BEEF);
    let expected = BitSet::from_u64(0xDEAD_BEEF);
    let result = bitset1 ^ bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_bitwise_xor2() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF);
    let bitset2 = BitSet::from_u64(0xCAFE_BABE);
    let expected = BitSet::from_u64(0x1453_0451);
    let result = bitset1 ^ bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_xor_bitset_is_zero() {
    let bitset1 = BitSet::from_u64(0xDEAD_BEEF);
    let bitset2 = BitSet::from_u64(0xDEAD_BEEF);
    let expected = BitSet::from_u64(0x0000_0000);
    let result = bitset1 ^ bitset2;

    assert_eq!(result, expected);
}

#[test]
fn test_bitset_shl1() {
    let bitset = BitSet::from_u64(0b0000_0001);
   
    assert_eq!(bitset << 0, BitSet::from_u64(0b0000_0001));
    assert_eq!(bitset << 1, BitSet::from_u64(0b0000_0010));
    assert_eq!(bitset << 2, BitSet::from_u64(0b0000_0100));
    assert_eq!(bitset << 3, BitSet::from_u64(0b0000_1000));
    assert_eq!(bitset << 4, BitSet::from_u64(0b0001_0000));
    assert_eq!(bitset << 5, BitSet::from_u64(0b0010_0000));
    assert_eq!(bitset << 6, BitSet::from_u64(0b0100_0000));
    assert_eq!(bitset << 7, BitSet::from_u64(0b1000_0000));
}

#[test]
fn test_bitset_shl_capacity_minus_one() {
    let bitset = BitSet::from_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    let expected = BitSet::from_u64(0);
    let result = (bitset << (bitset.capacity() - 1)) << 1;
    
    assert_eq!(result, expected);
}

#[test]
fn test_bitset_shr1() {
    let bitset = BitSet::from_u64(0b1000_0000);
   
    assert_eq!(bitset >> 0, BitSet::from_u64(0b1000_0000));
    assert_eq!(bitset >> 1, BitSet::from_u64(0b0100_0000));
    assert_eq!(bitset >> 2, BitSet::from_u64(0b0010_0000));
    assert_eq!(bitset >> 3, BitSet::from_u64(0b0001_0000));
    assert_eq!(bitset >> 4, BitSet::from_u64(0b0000_1000));
    assert_eq!(bitset >> 5, BitSet::from_u64(0b0000_0100));
    assert_eq!(bitset >> 6, BitSet::from_u64(0b0000_0010));
    assert_eq!(bitset >> 7, BitSet::from_u64(0b0000_0001));
    assert_eq!(bitset >> 8, BitSet::from_u64(0b0000_0000));
}

#[test]
fn test_bitset_shr_capacity() {
    let bitset = BitSet::from_u128(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
    let expected = BitSet::from_u64(0);
    let result = (bitset >> (bitset.capacity() - 1)) >> 1;
    
    assert_eq!(result, expected);
}