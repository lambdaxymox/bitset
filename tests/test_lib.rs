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