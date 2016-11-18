use std::slice;
use self::libc::int32_t;

pub mod int_rle;
pub use int_rle::int_rle_new;
pub use int_rle::int_rle_free;

pub use int_rle::int_rle_values;
pub use int_rle::int_rle_values_size;

pub use int_rle::int_rle_lengths;
pub use int_rle::int_rle_lengths_size;

extern crate libc;

#[no_mangle]
pub extern "C" fn rle_find_zero_lengths(array: *const int32_t, size: int32_t) -> int32_t {

    let lengths = unsafe {
        !array.is_null();

        slice::from_raw_parts(array, size as usize)
    };

    return find_zero_lengths(lengths, size);

}


pub fn find_zero_lengths(lengths: &[i32], size: int32_t) -> i32 {

    for i in 0..(size) {
        if lengths[i as usize] < 1 {
            return 1;
        };
    }

    return 0;

}



pub extern "C" fn lengths_equal(array1: *const int32_t,
                                size1: int32_t,
                                array2: *const int32_t,
                                size2: int32_t)
                                -> int32_t {

    let lengths1: &[i32] = unsafe {
        !array1.is_null();

        slice::from_raw_parts(array1, size1 as usize)
    };

    let lengths2: &[i32] = unsafe {
        !array2.is_null();

        slice::from_raw_parts(array2, size2 as usize)
    };

    let sum1: i32 = lengths1.iter().sum();
    let sum2 = lengths2.iter().sum();
    return (sum1 == sum2) as i32;

}

// let sum1 = lengths1.iter().fold(0, |mut sum, &x| {
//     sum += x;
//     sum
// });

#[cfg(test)]
mod rle_tests {
    use super::find_zero_lengths;

    #[test]
    fn test_find_zero_lengths() {

        let lengths = [0; 3];
        // println!("{:?}", lengths);

        let actual_result = find_zero_lengths(&lengths, 3);

        assert_eq!(1, actual_result);
    }


    #[test]
    fn test_find_zero_lengths2() {

        let lengths = [1, 1, 2, 3];
        // println!("{:?}", lengths);

        let actual_result = find_zero_lengths(&lengths, 3);

        assert_eq!(0, actual_result);

    }
}
