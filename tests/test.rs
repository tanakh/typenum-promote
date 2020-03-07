use typenum::Unsigned;
use typenum_promote::*;

#[test]
fn test() {
    assert_eq!(<promote!(0) as Unsigned>::to_u64(), 0);
    assert_eq!(<promote!(12345) as Unsigned>::to_u64(), 12345);
    assert_eq!(<promote!(0x12345) as Unsigned>::to_u64(), 0x12345);
    assert_eq!(
        <promote!(0xffff_ffff_ffff_ffff) as Unsigned>::to_u64(),
        0xffff_ffff_ffff_ffff
    );
}
