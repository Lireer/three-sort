#[inline(always)]
pub fn std_sort<T: Ord>(slice: &mut [T; 3]) {
    slice.sort()
}

#[inline(always)]
pub fn std_sort_unstable<T: Ord>(slice: &mut [T; 3]) {
    slice.sort_unstable()
}

/// Return 0xFF if `b == true` and 0x00 if `b == false`.
#[inline(always)]
fn bool_mask_u8(positive: bool) -> u8 {
    0u8.wrapping_sub(positive as u8)
}

#[inline(always)]
fn min_u8(a: u8, b: u8) -> u8 {
    a & bool_mask_u8(a <= b) | b & bool_mask_u8(b < a)
}

#[inline(always)]
fn max_u8(a: u8, b: u8) -> u8 {
    a & bool_mask_u8(a >= b) | b & bool_mask_u8(b > a)
}

#[inline(always)]
fn branchless_sort_two_u8(slice: &mut [u8]) {
    let min = min_u8(slice[0], slice[1]);
    slice[1] = max_u8(slice[0], slice[1]);
    slice[0] = min;
}

pub fn min_max_sort_u8(slice: &mut [u8; 3]) {
    branchless_sort_two_u8(&mut slice[0..=1]);
    branchless_sort_two_u8(&mut slice[1..=2]);
    branchless_sort_two_u8(&mut slice[0..=1]);
}

#[inline(always)]
fn bool_mask_u64(positive: bool) -> u64 {
    0u64.wrapping_sub(positive as u64)
}

#[inline(always)]
fn min_u64(a: u64, b: u64) -> u64 {
    a & bool_mask_u64(a <= b) | b & bool_mask_u64(b < a)
}

#[inline(always)]
fn max_u64(a: u64, b: u64) -> u64 {
    a & bool_mask_u64(a >= b) | b & bool_mask_u64(b > a)
}

#[inline(always)]
fn branchless_sort_two_u64(slice: &mut [u64]) {
    let min = min_u64(slice[0], slice[1]);
    slice[1] = max_u64(slice[0], slice[1]);
    slice[0] = min;
}

pub fn min_max_sort_u64(slice: &mut [u64; 3]) {
    branchless_sort_two_u64(&mut slice[0..=1]);
    branchless_sort_two_u64(&mut slice[1..=2]);
    branchless_sort_two_u64(&mut slice[0..=1]);
}

pub fn if_else_sort<T: Ord>(slice: &mut [T; 3]) {
    if slice[2] >= slice[1] {
        // slice[2] >= slice[1]
        if slice[0] > slice[2] {
            // slice[0] > slice[2] >= slice[1]
            slice.swap(2, 1);
            slice.swap(2, 0);
        } else if slice[0] > slice[1] {
            // slice[2] >= slice[0] > slice[1]
            slice.swap(1, 0);
        } // else do nothing
    } else {
        // slice[1] > slice[2]
        if slice[2] >= slice[0] {
            // slice[1] > slice[2] >= slice[0]
            slice.swap(2, 1);
        } else if slice[0] > slice[1] {
            // slice[0] > slice[1] > slice[2]
            slice.swap(2, 0);
        } else {
            // slice[1] >= slice[0] >= slice[2]
            slice.swap(2, 0);
            slice.swap(2, 1);
        }
    }
}

pub fn match_sort<T: Ord>(slice: &mut [T; 3]) {
    match (
        slice[0] < slice[1],
        slice[0] < slice[2],
        slice[1] < slice[2],
    ) {
        // 0 < 1 < 2
        (true, true, true) => (),
        // 0 < 2 <= 1
        (true, true, false) => slice.swap(1, 2),
        // 2 <= 0 < 1 < 2 ERROR
        (true, false, true) => unreachable!(),
        // 2 <= 0 < 1
        (true, false, false) => {
            slice.swap(0, 1);
            slice.swap(0, 2)
        }
        // 1 <= 0 < 2
        (false, true, true) => slice.swap(0, 1),
        // 1 <= 0 < 2 <= 1 ERROR
        (false, true, false) => unreachable!(),
        // 1 < 2 <= 0
        (false, false, true) => {
            slice.swap(0, 1);
            slice.swap(1, 2)
        }
        // 2 <= 1 <= 0
        (false, false, false) => slice.swap(0, 2),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bool_mask_u8, bubble_sort, if_else_sort, match_sort, max_u8, min_max_sort_u128,
        min_max_sort_u64, min_max_sort_u8, min_u8,
    };
    use proptest::prelude::*;

    #[test]
    fn bool_mask_positive() {
        assert_eq!(bool_mask_u8(true), 0xFF)
    }

    #[test]
    fn bool_mask_negative() {
        assert_eq!(bool_mask_u8(false), 0x00)
    }

    proptest! {
        #[test]
        fn test_min(a: u8, b: u8) {
            assert_eq!(a.min(b), min_u8(a,b))
        }

        #[test]
        fn test_max(a: u8, b: u8) {
            assert_eq!(a.max(b), max_u8(a,b))
        }

        #[test]
        fn test_sort_triple_if_else(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            if_else_sort(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }

        #[test]
        fn test_sort_triple_match(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            match_sort(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }

        #[test]
        fn test_sort_triple_bubble(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            bubble_sort(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }

        #[test]
        fn test_sort_triple_min_max_u8(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            min_max_sort_u8(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }

        #[test]
        fn test_sort_triple_min_max_u64(mut slice: [u64; 3]) {
            let mut test = slice.clone();
            min_max_sort_u64(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }
    }
}
