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

/// 位操作
/// 
/// [unset_rightmost_one](`Self::unset_rmo`)    
/// [set_rightmost_zero](`Self::set_rmz`)    
/// [unset_trailing_ones](`Self::unset_tos`)    
/// [set_trailing_zeros](`Self::set_tzs`)    
/// [set_rightmost_zero_unset_rest_bits](`Self::set_rmz_unset_rest_bs`)    
/// [unset_rightmost_one_set_rest_bits](`Self::unset_rmo_set_rest_bs`)    
/// [set_trailing_zeros_unset_rest_bits](`Self::set_tzs_unset_rest_bs`)    
/// [unset_trailing_ones_set_rest_bits](`Self::unset_tos_set_rest_bs`)    
/// [isolate_rightmost_one_unset_rest_bits](`Self::isolate_rmo_unset_rest_bs`)    
/// [set_rightmost_one_rightside_bits_unset_leftside_bits](`Self::set_rmo_rside_bs_unset_lside_bs`)    
/// [set_rightmost_zero_rightside_bits_unset_leftside_bits](`Self::set_rmz_rside_bs_unset_lside_bs`)    
pub trait Bits {
    /// $ x \ \\& \ (x-1) $：最右边1置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_0000, 0b_0000_0001u8.unset_rmo());
    /// assert_eq!(0b_0000_0000, 0b_1000_0000u8.unset_rmo());
    /// assert_eq!(0b_1010_1000, 0b_1010_1010u8.unset_rmo());
    /// assert_eq!(0b_0000_0000, 0b_0000_0000u8.unset_rmo()); // 无1不变
    /// ```
    fn unset_rmo(self) -> Self;

    /// $ x \ | \ (x+1) $：最右边0置1
    /// ````rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_0001, 0b_0000_0000u8.set_rmz());
    /// assert_eq!(0b_1111_1111, 0b_0111_1111u8.set_rmz());
    /// assert_eq!(0b_0101_0111, 0b_0101_0101u8.set_rmz());
    /// assert_eq!(0b_1111_1111, 0b_1111_1111u8.set_rmz()); // 无0不变
    /// ````
    fn set_rmz(self) -> Self;

    /// $ x \ \\& \ (x+1) $：尾部（右边）1s置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0001_0000, 0b_0001_0111u8.unset_tos());
    /// assert_eq!(0b_1111_0000, 0b_1111_0000u8.unset_tos());
    /// assert_eq!(0b_0000_0000, 0b_1111_1111u8.unset_tos());
    /// assert_eq!(0b_0000_0000, 0b_0000_0000u8.unset_tos()); // 无1不变
    /// ```
    fn unset_tos(self) -> Self;

    /// $ x \ | \ (x-1) $：尾部（右边）0s置1
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_0111, 0b_0000_0100u8.set_tzs());
    /// assert_eq!(0b_0000_1111, 0b_0000_1111u8.set_tzs());
    /// assert_eq!(0b_1111_1111, 0b_0000_0000u8.set_tzs());
    /// assert_eq!(0b_1111_1111, 0b_1111_1111u8.set_tzs()); // 无0不变
    /// ```
    fn set_tzs(self) -> Self;

    /// $ \neg x \ \\& \ (x+1) $：最右边0置1，剩余位置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_0001, 0b_1111_1110u8.set_rmz_unset_rest_bs());
    /// assert_eq!(0b_1000_0000, 0b_0111_1111u8.set_rmz_unset_rest_bs());
    /// assert_eq!(0b_0001_0000, 0b_1010_1111u8.set_rmz_unset_rest_bs());
    /// assert_eq!(0b_0000_0000, 0b_1111_1111u8.set_rmz_unset_rest_bs()); // 无0全置0
    /// ```
    fn set_rmz_unset_rest_bs(self) -> Self;

    /// $ \neg x \ | \ (x-1) $：最右边1置0，剩余位置1
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_1111_1110u8, 0b_0000_0001u8.unset_rmo_set_rest_bs());
    /// assert_eq!(0b_0111_1111u8, 0b_1000_0000u8.unset_rmo_set_rest_bs());
    /// assert_eq!(0b_1110_1111u8, 0b_0101_0000u8.unset_rmo_set_rest_bs());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.unset_rmo_set_rest_bs()); // 无1全置1
    /// ```
    fn unset_rmo_set_rest_bs(self) -> Self;

    /// $ \neg x \ \\& \ (x-1) $：尾部（右边）0s置1，剩余位置0
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
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_0101_0000u8.set_tzs_unset_rest_bs());
    /// assert_eq!(0b_0000_0000u8, 0b_0101_0101u8.set_tzs_unset_rest_bs());
    /// assert_eq!(0b_0000_0000u8, 0b_1111_1111u8.set_tzs_unset_rest_bs()); // 无0全置0
    /// ```
    fn set_tzs_unset_rest_bs(self) -> Self;

    /// $ \neg x \ | \ (x+1) $：尾部（右边）1s置0，剩余位置1
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_1111_0000u8, 0b_1010_1111u8.unset_tos_set_rest_bs());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.unset_tos_set_rest_bs()); // 无1全置1
    /// ```
    fn unset_tos_set_rest_bs(self) -> Self;

    /// $ x \ \\& \ (-x) $：保留最右边1，剩余位置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_0001u8, 0b_0101_0101u8.isolate_rmo_unset_rest_bs());
    /// assert_eq!(0b_1000_0000u8, 0b_1000_0000u8.isolate_rmo_unset_rest_bs());
    /// assert_eq!(0b_0000_1000u8, 0b_1010_1000u8.isolate_rmo_unset_rest_bs());
    /// assert_eq!(0b_0000_0000u8, 0b_0000_0000u8.isolate_rmo_unset_rest_bs()); // 无1全置0
    /// ```
    fn isolate_rmo_unset_rest_bs(self) -> Self;

    /// $ x \ \oplus \ (x-1) $：最右边1及其右边位置1，其左边位置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_1010_1000u8.set_rmo_rside_bs_unset_lside_bs());
    /// assert_eq!(0b_1111_1111u8, 0b_0000_0000u8.set_rmo_rside_bs_unset_lside_bs()); // 无1全置1
    /// assert_eq!(0b_0000_0001u8, 0b_0000_1111u8.set_rmo_rside_bs_unset_lside_bs()); // 尾部（右边）有1，结果为1
    /// ```
    fn set_rmo_rside_bs_unset_lside_bs(self) -> Self;

    /// $ x \ \oplus \ (x+1) $：最右边0及其右边位置1，其左边位置0
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(0b_0000_1111u8, 0b_0101_0111u8.set_rmz_rside_bs_unset_lside_bs());
    /// assert_eq!(0b_1111_1111u8, 0b_1111_1111u8.set_rmz_rside_bs_unset_lside_bs()); // 无0全置1
    /// assert_eq!(0b_0000_0001u8, 0b_1111_0000u8.set_rmz_rside_bs_unset_lside_bs()); // 尾部（右边）无1，结果为1
    /// ```
    fn set_rmz_rside_bs_unset_lside_bs(self) -> Self;

    /// 有效位是否均为1
    ///
    /// $ 7_{10} \ = \ 111_2 $ 的有效位为3位
    ///
    /// ```rust
    /// use rustiny_bs::Bits;
    ///
    /// assert_eq!(false, 0b_0000_0000u8.is_bits_set());
    /// assert_eq!(true, 0b_0000_0001u8.is_bits_set());
    /// assert_eq!(true, 0b_0000_0111u8.is_bits_set());
    /// assert_eq!(true, 0b_1111_1111u8.is_bits_set());
    /// ```
    fn is_bits_set(self) -> bool
    where
        Self: Ord + Sized + Zero,
    {
        Self::zero() != self && Self::zero() == self.unset_tos()
    }
}

pub trait UnsignedBits: Bits {
    /// 是否为 $ 2^n $
    /// ```rust
    /// use rustiny_bs::UnsignedBits;
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
        Self::zero() != self && Self::zero() == self.unset_rmo()
    }

    /// 是否为 $ 2^n \ - \ 1 $
    /// ```rust
    /// use rustiny_bs::UnsignedBits;
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
        Self::zero() == self.unset_tos()
    }
}

pub trait SignedBits: Bits {}

impl<T: Integer> Bits for T
where
    T: BitAnd<T, Output = T>,
    T: BitOr<T, Output = T>,
    T: BitXor<T, Output = T>,
    T: Not<Output = T>,
    T: WrappingAdd,
    T: WrappingNeg,
    T: WrappingSub,
    T: One,
{
    fn unset_rmo(self) -> Self {
        self & self.wrapping_sub(T::one())
    }

    fn set_rmz(self) -> Self {
        self | self.wrapping_add(T::one())
    }

    fn unset_tos(self) -> Self {
        self & self.wrapping_add(T::one())
    }

    fn set_tzs(self) -> Self {
        self | self.wrapping_sub(T::one())
    }

    fn set_rmz_unset_rest_bs(self) -> Self {
        !self & self.wrapping_add(T::one())
    }

    fn unset_rmo_set_rest_bs(self) -> Self {
        !self | self.wrapping_sub(T::one())
    }

    fn set_tzs_unset_rest_bs(self) -> Self {
        !self & self.wrapping_sub(T::one())
    }

    fn unset_tos_set_rest_bs(self) -> Self {
        !self | self.wrapping_add(T::one())
    }

    fn isolate_rmo_unset_rest_bs(self) -> Self {
        self & self.wrapping_neg()
    }

    fn set_rmo_rside_bs_unset_lside_bs(self) -> Self {
        self ^ self.wrapping_sub(T::one())
    }

    fn set_rmz_rside_bs_unset_lside_bs(self) -> Self {
        self ^ self.wrapping_add(T::one())
    }
}

impl<T: Unsigned + Bits> UnsignedBits for T {}

impl<T: Signed + Bits> SignedBits for T {}
