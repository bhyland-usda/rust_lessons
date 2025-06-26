/*
    There are multiple types of integer types (whole numbers).

    i8:     8-bit integer -   i8::MIN = -128,                                                     i8::MAX = 127
    i16:   16-bit integer -  i16::MIN = -32,768,                                                 i16::MAX = 32,767
    i32:   32-bit integer -  i32::MIN = -2,147,483,648                                          i32::MAX = 2,147,483,647
    i64:   64-bit integer -  i64::MIN = -9,223,372,036,854,775,808                               i64::MAX = 9,223,372,036,854,775,807
    i128: 128-bit integer - i128::MIN = -170,141,183,460,469,231,731,687,303,715,884,105,728    i128::MAX = 170,141,183,460,469,231,731,687,303,715,884,105,727

    Unsigned:
    u8:     8-bit -   u8::MIN = 0   u8::MAX = 255
    u16:   16-bit -  u16::MIN = 0  u16::MAX = 65,535
    u32:   32-bit -  u32::MIN = 0  u32::MAX = 4,294,967,295
    u64:   64-bit -  u64::MIN = 0  u64::MAX = 18,446,744,073,709,551,615
    u128: 128-bit - u128::MIN = 0 u128::MAX = 340,282,366,920,938,463,463,374,607,431,768,211,455
*/

fn signed_ints() -> (i8, i16, i32, i64, i128) {
    (i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX)
}

fn unsigned_ints() -> (u8, u16, u32, u64, u128) {
    (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_ints_test() {
        assert_eq!(signed_ints(), (i8::MAX, i16::MAX, i32::MAX, i64::MAX, i128::MAX));
    }

    #[test]
    fn unsigned_ints_test() {
        assert_eq!(unsigned_ints(), (u8::MAX, u16::MAX, u32::MAX, u64::MAX, u128::MAX));
    }
}
