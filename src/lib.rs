#[inline(always)]
pub fn std_sort<T: Ord>(slice: &mut [T; 3]) {
    slice.sort()
}

#[inline(always)]
pub fn std_sort_unstable<T: Ord>(slice: &mut [T; 3]) {
    slice.sort_unstable()
}

pub fn sort_if_else<T: Ord>(slice: &mut [T; 3]) {
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

pub fn sort_match<T: Ord>(slice: &mut [T; 3]) {
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
    use crate::{sort_if_else, sort_match};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sort_triple_if_else(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            sort_if_else(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }

        #[test]
        fn test_sort_triple_match(mut slice: [u8; 3]) {
            let mut test = slice.clone();
            sort_match(&mut test);
            slice.sort();
            prop_assert_eq!(test, slice);
        }
    }
}
