//!

#[macro_export(local_inner_macros)]
#[rustfmt::skip]
macro_rules! range_generic_type {
    (signed) => {isize};
    (unsigned) => {usize};
}

#[macro_export(local_inner_macros)]
#[rustfmt::skip]
macro_rules! define_int_fn_ptr_sized_value {
    ($vis:vis signed) => {
        $vis const fn value_isize(self) -> isize {
            self.0 as isize
        }
    };
    ($vis:vis unsigned) => {
        $vis const fn value_usize(self) -> usize {
            self.0 as usize
        }
    };
}

#[macro_export(local_inner_macros)]
#[rustfmt::skip]
macro_rules! define_arrayvec_fn_ops {
    ($vis:vis $item_ty:ty, $len_ty:ty, $idx_ty:ty, $capacity:expr) => {
        pub const fn len(&self) -> $len_ty {
            self.len as $len_ty
        }

        pub fn push(&mut self, v: $item_ty) {
            ::core::assert!(self.len < $capacity);
            self.array[self.len].write(v);
            self.len += 1;
        }
        
        pub fn pop(&mut self) -> Option<$item_ty> {
            if self.len == 0 {
                None
            } else {
                self.len -= 1;
                Some(unsafe {
                    self.array[self.len].assume_init_read()
                })
            }
        }

        pub fn clear(&mut self) {
            self.truncate(0);
        }

        pub fn truncate(&mut self, new_len: $len_ty) {
            let new_len = new_len as usize;
            while self.len > new_len {
                self.pop();
            }
        }

        pub fn get(&self, idx: $idx_ty) -> Option<&$item_ty> {
            let offset = idx.offset();
            if offset as usize >= self.len {
                None
            } else {
                Some(unsafe {
                    &*self.array[offset as usize].as_ptr()
                })
            }            
        }

        pub fn get_mut(&mut self, idx: $idx_ty) -> Option<&mut $item_ty> {
            let offset = idx.offset();
            if offset as usize >= self.len {
                None
            } else {
                Some(unsafe {
                    &mut *self.array[offset as usize].as_mut_ptr()
                })
            }            
        }

        pub fn as_slice(&self) -> &[$item_ty] {
            unsafe {
                &*((&self.array[..self.len])
                    as * const [::core::mem::MaybeUninit<$item_ty>]
                    as * const [$item_ty])
            }
        }

        pub fn as_mut_slice(&mut self) -> &mut[$item_ty] {
            unsafe {
                &mut *((&mut self.array[..self.len])
                    as * mut [::core::mem::MaybeUninit<$item_ty>]
                    as * mut [$item_ty])
            }
        }

        pub fn iter(&self) -> ::core::slice::Iter<'_, $item_ty> {
            self.as_slice().iter()
        }

        pub fn iter_mut(&mut self) -> ::core::slice::IterMut<'_, $item_ty> {
            self.as_mut_slice().iter_mut()
        }
    }
}

#[rustfmt::skip]
macro_rules! define_ranged_types {
    ($ranged_int_macro:ident $ranged_int_ty:ident $ranged_array_macro:ident $ranged_array_ty:ident
        $ranged_arrayvec_macro:ident $ranged_arrayvec_ty:ident : $dollar:tt $ty:ident $len_ty:ident $signed:ident) => {
        #[macro_export]
        macro_rules! $ranged_int_macro {
            ($dollar m:expr, $dollar n:expr) => {
                $dollar crate::$ranged_int_ty<
                    {($dollar m) as range_generic_type!($signed)},
                    {($dollar n - $dollar m) as usize}>}
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ranged_int_ty<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>($ty);

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            $ranged_int_ty<RANGE_START, RANGE_LEN> {

            pub const fn new(v: $ty) -> Self {
                assert!((v as range_generic_type!($signed)) >= RANGE_START);
                assert!((((v as range_generic_type!($signed)) - RANGE_START) as usize) < RANGE_LEN);
                $ranged_int_ty(v)
            }

            pub const fn checked_new(v: $ty) -> Option<Self> {
                if((v as range_generic_type!($signed)) >= RANGE_START && 
                    (((v as range_generic_type!($signed)) - RANGE_START) as usize) < RANGE_LEN)
                {
                    Some($ranged_int_ty(v))
                } else {
                    None
                }
            }
            
            pub const fn value(self) -> $ty {
                self.0
            }
            
            define_int_fn_ptr_sized_value!(pub $signed);

            pub const fn offset(self) -> $len_ty {
                ((self.0 as range_generic_type!($signed)) - RANGE_START) as $len_ty
            }

            pub fn checked_add(self, v: $ty) -> Option<Self> {
                let v = self.0.checked_add(v)?;
                $ranged_int_ty::checked_new(v)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            Default for $ranged_int_ty<RANGE_START, RANGE_LEN> {
            fn default() -> Self {
                $ranged_int_ty(RANGE_START as $ty)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            PartialEq<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN> {
            fn eq(&self, other: &$ty) -> bool {
                ::core::cmp::PartialEq::eq(&self.0, other)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            PartialOrd<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN> {
            fn partial_cmp(&self, other: &$ty) -> Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, other)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            ::core::convert::From<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN> {
            fn from(v: $ty) -> Self {
                $ranged_int_ty::new(v)
            }
        }

        #[macro_export]
        macro_rules! $ranged_arrayvec_macro {
            ($item_ty:path, $dollar m:expr, $dollar n:expr) => {
                $dollar crate::$ranged_arrayvec_ty<
                    $item_ty,
                    {($dollar m) as $dollar crate::range_generic_type!($signed)},
                    {($dollar n - $dollar m) as usize}>}
        }

        pub struct $ranged_arrayvec_ty<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize> {
            array: [::core::mem::MaybeUninit<T>; RANGE_LEN],
            len: usize 
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            define_arrayvec_fn_ops!(T, $len_ty, $ranged_int_ty<RANGE_START, RANGE_LEN>, RANGE_LEN);
        }
        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            Drop for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            fn drop(&mut self) {
                self.clear();
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            Default for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            fn default() -> Self {
                $ranged_arrayvec_ty {
                    array: ::core::array::from_fn(|_| ::core::mem::MaybeUninit::uninit()),
                    len: 0
                }
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            ::core::ops::Index<$ranged_int_ty<RANGE_START, RANGE_LEN>>
            for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            
            type Output = T;

            fn index(&self, idx: $ranged_int_ty<RANGE_START, RANGE_LEN>) -> &T {
                let offset = idx.offset() as usize;
                &self.as_slice()[offset]
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            ::core::ops::IndexMut<$ranged_int_ty<RANGE_START, RANGE_LEN>>
            for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            
            fn index_mut(&mut self, idx: $ranged_int_ty<RANGE_START, RANGE_LEN>) -> &mut T {
                let offset = idx.offset() as usize;
                &mut self.as_mut_slice()[offset]
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            ::core::ops::Index<$ty> for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            
            type Output = T;

            fn index(&self, idx: $ty) -> &T {
                ::core::ops::Index::index(self, $ranged_int_ty::new(idx))
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize>
            ::core::ops::IndexMut<$ty> for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN> {
            
            fn index_mut(&mut self, idx: $ty) -> &mut T {
                ::core::ops::IndexMut::index_mut(self, $ranged_int_ty::new(idx))
            }
        }

    };
    ($ranged_int_macro:ident $ranged_int_ty:ident $ranged_array_macro:ident $ranged_array_ty:ident
        $ranged_arrayvec_macro:ident $ranged_arrayvec_ty:ident : $dollar:tt $ty:ident $len_ty:ident $signed:ident inclusive) => {
        #[macro_export]
        macro_rules! $ranged_int_macro {
            ($dollar m:expr, $dollar n:expr) => {
                $dollar crate::$ranged_int_ty<
                    {($dollar m) as $dollar crate::range_generic_type!($signed)},
                    {($dollar n - $dollar m + 1) as usize},
                    {($dollar n - $dollar m) as usize}>}
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ranged_int_ty<const RANGE_START: range_generic_type!($signed),
            const RANGE_LEN: usize,
            const RANGE_LAST_OFFSET: usize>($ty);

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            
            pub const fn new(v: $ty) -> Self {
                assert!((v as range_generic_type!($signed)) >= RANGE_START);
                assert!((((v as range_generic_type!($signed)) - RANGE_START) as usize) < RANGE_LEN);
                $ranged_int_ty(v)
            }

            pub const fn offset(self) -> $len_ty {
                ((self.0 as range_generic_type!($signed)) - RANGE_START) as $len_ty
            }

            pub const fn checked_new(v: $ty) -> Option<Self> {
                if((v as range_generic_type!($signed)) >= RANGE_START && 
                    (((v as range_generic_type!($signed)) - RANGE_START) as usize) < RANGE_LEN)
                {
                    Some($ranged_int_ty(v))
                } else {
                    None
                }
            }

            pub const fn value(self) -> $ty {
                self.0
            }
            
            define_int_fn_ptr_sized_value!(pub $signed);

            pub fn checked_add(self, v: $ty) -> Option<Self> {
                let v = self.0.checked_add(v)?;
                $ranged_int_ty::checked_new(v)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            Default for $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            #[allow(unused_comparisons)]
            fn default() -> Self {
                let def_value = if RANGE_START >= 0 {
                    RANGE_START as $ty
                } else {
                    let neg_start = (-(RANGE_START as isize)) as usize;
                    if neg_start > RANGE_LAST_OFFSET {
                        (-((neg_start - RANGE_LAST_OFFSET) as isize)) as $ty
                    } else {
                        0
                    }
                };
                $ranged_int_ty(def_value)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            PartialEq<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            fn eq(&self, other: &$ty) -> bool {
                ::core::cmp::PartialEq::eq(&self.0, other)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            PartialOrd<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            fn partial_cmp(&self, other: &$ty) -> Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, other)
            }
        }

        impl<const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            ::core::convert::From<$ty> for $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            fn from(v: $ty) -> Self {
                $ranged_int_ty::new(v)
            }
        }

        #[macro_export]
        macro_rules! $ranged_arrayvec_macro {
            ($item_ty:path, $dollar m:expr, $dollar n:expr) => {
                $dollar crate::$ranged_arrayvec_ty<
                    $item_ty,
                    {($dollar m) as $dollar crate::range_generic_type!($signed)},
                    {($dollar n - $dollar m + 1) as usize},
                    {($dollar n - $dollar m) as usize}>}
        }

        pub struct $ranged_arrayvec_ty<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize> {
            array: [::core::mem::MaybeUninit<T>; RANGE_LEN],
            len: usize 
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            define_arrayvec_fn_ops!(T, $len_ty, $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET>, RANGE_LEN);
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            Drop for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            fn drop(&mut self) {
                self.clear();
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            Default for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            fn default() -> Self {
                $ranged_arrayvec_ty {
                    array: ::core::array::from_fn(|_| ::core::mem::MaybeUninit::uninit()),
                    len: 0
                }
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            ::core::ops::Index<$ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET>>
            for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            
            type Output = T;

            fn index(&self, idx: $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET>) -> &T {
                let offset = idx.offset() as usize;
                &self.as_slice()[offset]
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            ::core::ops::IndexMut<$ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET>>
            for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            
            fn index_mut(&mut self, idx: $ranged_int_ty<RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET>) -> &mut T {
                let offset = idx.offset() as usize;
                &mut self.as_mut_slice()[offset]
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            ::core::ops::Index<$ty> for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            
            type Output = T;

            fn index(&self, idx: $ty) -> &T {
                ::core::ops::Index::index(self, $ranged_int_ty::new(idx))
            }
        }

        impl<T, const RANGE_START: range_generic_type!($signed), const RANGE_LEN: usize, const RANGE_LAST_OFFSET: usize>
            ::core::ops::IndexMut<$ty> for $ranged_arrayvec_ty<T, RANGE_START, RANGE_LEN, RANGE_LAST_OFFSET> {
            
            fn index_mut(&mut self, idx: $ty) -> &mut T {
                ::core::ops::IndexMut::index_mut(self, $ranged_int_ty::new(idx))
            }
        }
    };
}

#[rustfmt::skip]
macro_rules! define_range_to_types {
    ($ranged_int_macro:ident($ranged_int_full_macro:ident) $ranged_array_macro:ident ($ranged_array_full_macro:ident)
        $ranged_arrayvec_macro:ident ($ranged_arrayvec_full_macro:ident) : $dollar:tt) => {
        #[macro_export]
        macro_rules! $ranged_int_macro {
            ($dollar n:expr) => {$dollar crate::$ranged_int_full_macro!(0, $dollar n)}
        }

        #[macro_export]
        macro_rules! $ranged_arrayvec_macro {
            ($dollar item_ty:ty, $dollar n:expr) => {$dollar crate::$ranged_arrayvec_full_macro!($dollar item_ty, 0, $dollar n)}
        }
    };
}

define_ranged_types!(u8_MtoN  RangedU8  u8_MtoN_array RangedU8Array  u8_MtoN_arrayvec RangedU8ArrayVec
    : $ u8  u8  unsigned);
define_ranged_types!(u16_MtoN RangedU16 u16_MtoN_array RangedU16Array u16_MtoN_arrayvec RangedU16ArrayVec
    : $ u16 u16 unsigned);
define_ranged_types!(u32_MtoN RangedU32 u32_MtoN_array RangedU32Array u32_MtoN_arrayvec RangedU32ArrayVec
    : $ u32 u32 unsigned);

define_ranged_types!(u8_MtoNinc  RangedU8Inc  u8_MtoNinc_array  RangedU8IncArray  u8_MtoNinc_arrayvec  RangedU8IncArrayVec 
    : $ u8  u8  unsigned inclusive);
define_ranged_types!(u16_MtoNinc RangedU16Inc u16_MtoNinc_array RangedU16IncArray u16_MtoNinc_arrayvec RangedU16IncArrayVec
    : $ u16 u16 unsigned inclusive);
define_ranged_types!(u32_MtoNinc RangedU32Inc u32_MtoNinc_array RangedU32IncArray u32_MtoNinc_arrayvec RangedU32IncArrayVec
    : $ u32 u32 unsigned inclusive);

define_ranged_types!(i8_MtoN  RangedI8  i8_MtoN_array  RangedI8Array  i8_MtoN_arrayvec  RangedI8ArrayVec 
    : $ i8  u8  signed);
define_ranged_types!(i16_MtoN RangedI16 i16_MtoN_array RangedI16Array i16_MtoN_arrayvec RangedI16ArrayVec
    : $ i16 u16 signed);
define_ranged_types!(i32_MtoN RangedI32 i32_MtoN_array RangedI32Array i32_MtoN_arrayvec RangedI32ArrayVec
    : $ i32 u32 signed);

define_ranged_types!(i8_MtoNinc  RangedI8Inc  i8_MtoNinc_array  RangedI8IncArray  i8_MtoNinc_arrayvec  RangedI8IncArrayVec 
    : $ i8  u8  signed inclusive);
define_ranged_types!(i16_MtoNinc RangedI16Inc i16_MtoNinc_array RangedI16IncArray i16_MtoNinc_arrayvec RangedI16IncArrayVec
    : $ i16 u16 signed inclusive);
define_ranged_types!(i32_MtoNinc RangedI32Inc i32_MtoNinc_array RangedI32IncArray i32_MtoNinc_arrayvec RangedI32IncArrayVec
    : $ i32 u32 signed inclusive);

define_range_to_types!(u8_0toN (u8_MtoN)  u8_0toN_array (u8_MtoN_array)  u8_0toN_arrayvec (u8_MtoN_arrayvec) : $);
define_range_to_types!(u16_0toN(u16_MtoN) u16_0toN_array(u16_MtoN_array) u16_0toN_arrayvec(u16_MtoN_arrayvec): $);
define_range_to_types!(u32_0toN(u32_MtoN) u32_0toN_array(u32_MtoN_array) u32_0toN_arrayvec(u32_MtoN_arrayvec): $);

define_range_to_types!(u8_0toNinc (u8_MtoNinc)  u8_0toNinc_array (u8_MtoNinc_array)  u8_0toNinc_arrayvec (u8_MtoNinc_arrayvec) : $);
define_range_to_types!(u16_0toNinc(u16_MtoNinc) u16_0toNinc_array(u16_MtoNinc_array) u16_0toNinc_arrayvec(u16_MtoNinc_arrayvec): $);
define_range_to_types!(u32_0toNinc(u32_MtoNinc) u32_0toNinc_array(u32_MtoNinc_array) u32_0toNinc_arrayvec(u32_MtoNinc_arrayvec): $);
