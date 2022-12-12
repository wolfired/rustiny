use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::BitXor;
use std::ops::Not;

use rustiny_number::Integer;
use rustiny_number::One;
use rustiny_number::Signed;
use rustiny_number::Unsigned;
use rustiny_number::WrappingAdd;
use rustiny_number::WrappingNeg;
use rustiny_number::WrappingSub;
use rustiny_number::Zero;

pub trait Bits {
    /// $ x \\& (x-1) $：把最右边的1置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_0000, 0b_0000_0001u8.unset_rightmost_one());
    /// assert_eq!(0b_0000_0000, 0b_1000_0000u8.unset_rightmost_one());
    /// assert_eq!(0b_1010_1000, 0b_1010_1010u8.unset_rightmost_one());
    /// assert_eq!(0b_0000_0000, 0b_0000_0000u8.unset_rightmost_one()); // 无1不变
    /// ```
    fn unset_rightmost_one(self) -> Self;

    /// $ x | (x+1) $：把最右边的0置1
    /// ````rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_0001, 0b_0000_0000u8.set_rightmost_zero());
    /// assert_eq!(0b_1111_1111, 0b_0111_1111u8.set_rightmost_zero());
    /// assert_eq!(0b_0101_0111, 0b_0101_0101u8.set_rightmost_zero());
    /// assert_eq!(0b_1111_1111, 0b_1111_1111u8.set_rightmost_zero()); // 无0不变
    /// ````
    fn set_rightmost_zero(self) -> Self;

    /// $ x \\& (x+1) $：把尾部（右边）的1置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0001_0000, 0b_0001_0111u8.unset_trailing_ones());
    /// assert_eq!(0b_1111_0000, 0b_1111_0000u8.unset_trailing_ones());
    /// assert_eq!(0b_0000_0000, 0b_1111_1111u8.unset_trailing_ones());
    /// assert_eq!(0b_0000_0000, 0b_0000_0000u8.unset_trailing_ones()); // 无1不变
    /// ```
    fn unset_trailing_ones(self) -> Self;

    /// $ x | (x-1) $：把尾部（右边）的0置1
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_0111, 0b_0000_0100u8.set_trailing_zeros());
    /// assert_eq!(0b_0000_1111, 0b_0000_1111u8.set_trailing_zeros());
    /// assert_eq!(0b_1111_1111, 0b_0000_0000u8.set_trailing_zeros());
    /// assert_eq!(0b_1111_1111, 0b_1111_1111u8.set_trailing_zeros()); // 无0不变
    /// ```
    fn set_trailing_zeros(self) -> Self;

    /// $ \neg x \\& (x+1) $：把最右边的0置1，剩余位置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_0001, 0b_1111_1110u8.set_rightmost_zero_unset_rest_bits());
    /// assert_eq!(0b_1000_0000, 0b_0111_1111u8.set_rightmost_zero_unset_rest_bits());
    /// assert_eq!(0b_0001_0000, 0b_1010_1111u8.set_rightmost_zero_unset_rest_bits());
    /// assert_eq!(0b_0000_0000, 0b_1111_1111u8.set_rightmost_zero_unset_rest_bits()); // 无0全置0
    /// ```
    fn set_rightmost_zero_unset_rest_bits(self) -> Self;

    /// $ \neg x | (x-1) $：把最右边的1置0，剩余位置1
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_1111_1110u8, 0b_0000_0001u8.unset_rightmost_one_set_rest_bits());
    /// assert_eq!(0b_0111_1111u8, 0b_1000_0000u8.unset_rightmost_one_set_rest_bits());
    /// assert_eq!(0b_1110_1111u8, 0b_0101_0000u8.unset_rightmost_one_set_rest_bits());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.unset_rightmost_one_set_rest_bits()); // 无1全置1
    /// ```
    fn unset_rightmost_one_set_rest_bits(self) -> Self;

    /// $ \neg x \\& (x-1) $：把尾部（右边）的0置1，剩余位置0
    ///
    /// 有三种实现：
    /// ```rust
    /// let x = 0b_0101_0000i8;
    ///
    /// assert_eq!(0b_0000_1111, !x & (x-1)); // 本库使用此现实
    /// assert_eq!(0b_0000_1111, !(x | -x));
    /// assert_eq!(0b_0000_1111, (x & -x) - 1);
    /// ```
    ///
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_0101_0000u8.set_trailing_zeros_unset_rest_bits());
    /// assert_eq!(0b_0000_0000u8, 0b_0101_0101u8.set_trailing_zeros_unset_rest_bits());
    /// assert_eq!(0b_0000_0000u8, 0b_1111_1111u8.set_trailing_zeros_unset_rest_bits()); // 无0全置0
    /// ```
    fn set_trailing_zeros_unset_rest_bits(self) -> Self;

    /// $ \neg x | (x+1) $：把尾部（右边）的1置0，剩余位置1
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_1111_0000u8, 0b_1010_1111u8.unset_trailing_ones_set_rest_bits());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.unset_trailing_ones_set_rest_bits()); // 无1全置1
    /// ```
    fn unset_trailing_ones_set_rest_bits(self) -> Self;

    /// $ x \\& (-x) $：保留最右边的1，剩余位置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_0001u8, 0b_0101_0101u8.isolate_rightmost_one_unset_rest_bits());
    /// assert_eq!(0b_1000_0000u8, 0b_1000_0000u8.isolate_rightmost_one_unset_rest_bits());
    /// assert_eq!(0b_0000_1000u8, 0b_1010_1000u8.isolate_rightmost_one_unset_rest_bits());
    /// assert_eq!(0b_0000_0000u8, 0b_0000_0000u8.isolate_rightmost_one_unset_rest_bits()); // 无1全置0
    /// ```
    fn isolate_rightmost_one_unset_rest_bits(self) -> Self;

    /// $ x \oplus (x-1) $：把最右边的1及其右边位置1，其左边位置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_1010_1000u8.set_rightmost_one_and_rightside_bits_unset_leftside_bits());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.set_rightmost_one_and_rightside_bits_unset_leftside_bits()); // 无1全置1
    /// assert_eq!(0b_0000_0001u8, 0b_0000_1111u8.set_rightmost_one_and_rightside_bits_unset_leftside_bits()); // 如果尾部（右边）有1，结果为1
    /// ```
    fn set_rightmost_one_and_rightside_bits_unset_leftside_bits(self) -> Self;

    /// $ x \oplus (x+1) $：把最右边的0及其右边位置1，其左边位置0
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_0101_0111u8.set_rightmost_zero_and_rightside_bits_unset_leftside_bits());
    /// assert_eq!(0b_1111_1111u8, 0b_1111_1111u8.set_rightmost_zero_and_rightside_bits_unset_leftside_bits()); // 无0全置1
    /// assert_eq!(0b_0000_0001u8, 0b_1111_0000u8.set_rightmost_zero_and_rightside_bits_unset_leftside_bits()); // 如果尾部（右边）无1，结果为1
    /// ```
    fn set_rightmost_zero_and_rightside_bits_unset_leftside_bits(self) -> Self;

    /// 有效位是否均为1
    ///
    /// $ 7_{10} = 111_2 $ 的有效位为3位
    ///
    /// ```rust
    /// use rustiny_bits::Bits;
    ///
    /// assert_eq!(false, 0b_0000_0000u8.are_all_bits_set());
    /// assert_eq!(true, 0b_0000_0001u8.are_all_bits_set());
    /// assert_eq!(true, 0b_0000_0111u8.are_all_bits_set());
    /// assert_eq!(true, 0b_1111_1111u8.are_all_bits_set());
    /// ```
    fn are_all_bits_set(self) -> bool
    where
        Self: Ord + Sized + Zero,
    {
        Self::zero() != self && Self::zero() == self.unset_trailing_ones()
    }
}

pub trait UnsignedBits: Bits {
    /// 是否为 $ 2^n $
    /// ```rust
    /// use rustiny_bits::UnsignedBits;
    ///
    /// assert_eq!(false, <u8 as UnsignedBits>::is_power_of_two(0b_0000_0000u8));
    /// assert_eq!(true, <u8 as UnsignedBits>::is_power_of_two(0b_0000_0001u8));
    /// assert_eq!(true, <u8 as UnsignedBits>::is_power_of_two(0b_1000_0000u8));
    /// assert_eq!(false, <u8 as UnsignedBits>::is_power_of_two(0b_1000_0001u8));
    /// ```
    fn is_power_of_two(self) -> bool
    where
        Self: Ord + Sized + Zero,
    {
        Self::zero() != self && Self::zero() == self.unset_rightmost_one()
    }

    /// 是否为 $ 2^n - 1 $
    /// ```rust
    /// use rustiny_bits::UnsignedBits;
    ///
    /// assert_eq!(true, 0b_0000_0000u8.is_power_of_two_minus_one());
    /// assert_eq!(true, 0b_0000_0001u8.is_power_of_two_minus_one());
    /// assert_eq!(true, 0b_0000_0011u8.is_power_of_two_minus_one());
    /// assert_eq!(true, 0b_1111_1111u8.is_power_of_two_minus_one());
    /// assert_eq!(false, 0b_0000_0010u8.is_power_of_two_minus_one());
    /// ```
    fn is_power_of_two_minus_one(self) -> bool
    where
        Self: Ord + Sized + Zero,
    {
        Self::zero() == self.unset_trailing_ones()
    }
}

pub trait SignedBits: Bits {}

impl<T: Integer> Bits for T
where
    T: BitAnd<T, Output = T>,
    T: BitOr<T, Output = T>,
    T: BitXor<T, Output = T>,
    T: Not<Output = T>,
    T: WrappingAdd<T>,
    T: WrappingNeg,
    T: WrappingSub<T>,
    T: One,
{
    fn unset_rightmost_one(self) -> Self {
        self & self.wrapping_sub(T::one())
    }

    fn set_rightmost_zero(self) -> Self {
        self | self.wrapping_add(T::one())
    }

    fn unset_trailing_ones(self) -> Self {
        self & self.wrapping_add(T::one())
    }

    fn set_trailing_zeros(self) -> Self {
        self | self.wrapping_sub(T::one())
    }

    fn set_rightmost_zero_unset_rest_bits(self) -> Self {
        !self & self.wrapping_add(T::one())
    }

    fn unset_rightmost_one_set_rest_bits(self) -> Self {
        !self | self.wrapping_sub(T::one())
    }

    fn set_trailing_zeros_unset_rest_bits(self) -> Self {
        !self & self.wrapping_sub(T::one())
    }

    fn unset_trailing_ones_set_rest_bits(self) -> Self {
        !self | self.wrapping_add(T::one())
    }

    fn isolate_rightmost_one_unset_rest_bits(self) -> Self {
        self & self.wrapping_neg()
    }

    fn set_rightmost_one_and_rightside_bits_unset_leftside_bits(self) -> Self {
        self ^ self.wrapping_sub(T::one())
    }

    fn set_rightmost_zero_and_rightside_bits_unset_leftside_bits(self) -> Self {
        self ^ self.wrapping_add(T::one())
    }
}

impl<T: Unsigned + Bits> UnsignedBits for T {}

impl<T: Signed + Bits> SignedBits for T {}
