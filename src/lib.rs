pub mod rle;
use rle::Rle;
pub use rle::rle_new;
pub use rle::rle_free;

use std::slice;
extern crate libc;
use self::libc::{size_t, int32_t};

// #[no_mangle]
// pub extern "C" fn rle_new(lengths_data: *const int32_t,
//                           lengths_length: size_t,
//                           values_data: *const int32_t,
//                           values_length: size_t)
//                           -> *mut Rle {
//     let lengths = unsafe { slice::from_raw_parts(lengths_data, lengths_length as usize).to_vec() };
//     let values = unsafe { slice::from_raw_parts(values_data, values_length as usize).to_vec() };

//     return Box::into_raw(Box::new(Rle::new(lengths, values)));

// }


// #[no_mangle]
// pub extern "C" fn rle_free(ptr: *mut Rle) {
//     if ptr.is_null() {
//         return;
//     }
//     unsafe {
//         Box::from_raw(ptr);
//     }
// }



#[cfg(test)]

mod tests {
    use super::rle::Rle;

    #[test]
    fn create_new_rle1() {

        let lengths = vec![1];
        let values = vec![0];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![1],
            values: vec![0],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_rle2() {

        let lengths = vec![1, 1];
        let values = vec![0, 1];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![1, 1],
            values: vec![0, 1],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }


    #[test]
    fn create_new_rle3() {

        let lengths = vec![1, 1, 2];
        let values = vec![1, 1, 2];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![2, 2],
            values: vec![1, 2],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    // #[test]
    // fn create_new_rle4() {

    //     let lengths = vec![100, 1, 1, 2];
    //     let values = vec![0, 1, 1, 2];
    //     println!("Test starting");
    //     let actual_result = Rle::new(lengths, values);

    //     let expected_result = Rle {
    //         lengths: vec![100, 2, 2],
    //         values: vec![0, 1, 2],
    //     };

    //     println!("Actual {:?}", actual_result);
    //     println!("Expected {:?}", expected_result);
    //     assert_eq!(expected_result, actual_result);
    // }


    #[test]
    fn create_new_rle5() {

        let lengths = vec![1, 1, 1, 2];
        let values = vec![1, 1, 1, 2];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![3, 2],
            values: vec![1, 2],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }


    #[test]
    fn create_new_rle6() {

        let lengths = vec![2, 1, 1];
        let values = vec![2, 1, 1];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![2, 2],
            values: vec![2, 1],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_rle7() {

        let lengths = vec![1, 1, 1, 1, 1, 1, 2, 1, 1];
        let values = vec![2, 5, 3, 2, 4, 4, 3, 4, 4];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![1, 1, 1, 1, 2, 2, 2],
            values: vec![2, 5, 3, 2, 4, 3, 4],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_rle8() {

        let lengths = vec![1, 2, 2, 1, 1];
        let values = vec![2, 3, 2, 1, 2];
        println!("Test starting");
        let actual_result = Rle::new(lengths, values);

        let expected_result = Rle {
            lengths: vec![1, 2, 2, 1, 1],
            values: vec![2, 3, 2, 1, 2],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }


    #[test]
    fn add_two_rles1() {

        let rle1 = Rle {
            lengths: vec![1, 1],
            values: vec![0, 1],
        };

        let rle2 = Rle {
            lengths: vec![2],
            values: vec![1],
        };

        let expected_result = Rle {
            lengths: vec![1, 1],
            values: vec![1, 2],
        };

        println!("Test starts");

        let actual_result = rle1.add(&rle2);

        println!("Result struct: {:?}", actual_result);

        assert_eq!(expected_result, actual_result);
        // assert_eq!(rle1.add(&rle2), expected_result);


    }

    // #[test]
    // fn add_two_rles2() {

    //     let rle1 = Rle {
    //         lengths: vec![1, 4, 2],
    //         values: vec![1, 2, 1],
    //     };

    //     let rle2 = Rle {
    //         lengths: vec![3, 3, 1],
    //         values: vec![1, 0, 1],
    //     };


    //     let expected_result = Rle {
    //         lengths: vec![1, 2, 2, 1, 1],
    //         values: vec![2, 3, 2, 1, 2],
    //     };

    //     println!("Test starts");
    //     let actual_result = rle1.add(&rle2);
    //     println!("\nExpected lengths: {:?}", expected_result.lengths);
    //     println!("Actual lengths: {:?}\n", actual_result.lengths);
    //     println!("Expected values: {:?}", expected_result.values);
    //     println!("Actual values: {:?}", actual_result.values);
    //     assert_eq!(actual_result, expected_result);

    // }


    #[test]
    fn add_two_rles3() {

        let rle1 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let rle2 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };


        let expected_result = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![2, 4, 6, 8, 10, 12, 14, 16, 18],
        };



    }


    #[test]
    fn add_two_rles4() {

        let rle1 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let rle2 = Rle {
            lengths: vec![45],
            values: vec![10],
        };


        let expected_result = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
        };
        println!("Test starts");
        let actual_result = rle1.add(&rle2);
        println!("\nExpected lengths: {:?}", expected_result.lengths);
        println!("Actual lengths: {:?}\n", actual_result.lengths);
        println!("Expected values: {:?}", expected_result.values);
        println!("Actual values: {:?}", actual_result.values);
        assert_eq!(actual_result, expected_result);

    }

    #[test]
    fn add_two_rles5() {

        let rle1 = Rle {
            lengths: vec![1, 1, 3, 3, 1, 1],
            values: vec![1, 3, 1, 2, 3, 1],
        };

        let rle2 = Rle {
            lengths: vec![1, 2, 1, 1, 1, 3, 1],
            values: vec![1, 2, 1, 3, 2, 1, 3],
        };

        let expected_result = Rle {
            lengths: vec![1, 1, 1, 1, 2, 2, 2],
            values: vec![2, 5, 3, 2, 4, 3, 4],
        };

        println!("Test starts");
        let actual_result = rle1.add(&rle2);
        println!("\nExpected lengths: {:?}", expected_result.lengths);
        println!("Actual lengths: {:?}\n", actual_result.lengths);
        println!("Expected values: {:?}", expected_result.values);
        println!("Actual values: {:?}", actual_result.values);
        assert_eq!(actual_result, expected_result);

    }


    #[test]
    fn add_two_rles6() {

        let rle1 = Rle {
            lengths: vec![1, 1, 1, 1, 2, 3, 2, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1,
                          1, 2, 1, 2, 2, 1, 4, 1, 3, 2, 1, 1, 2, 1, 1, 2, 2, 3, 2, 1, 1, 2, 2, 2,
                          3, 1, 2, 1, 4, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1],
            values: vec![2, 1, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 3, 1, 2, 1, 2, 3, 2, 1, 3,
                         2, 3, 2, 3, 2, 3, 2, 1, 3, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 3, 2, 3, 2, 3,
                         2, 3, 1, 3, 2, 3, 2, 1, 2, 1, 2, 1, 2, 3, 1, 2],
        };

        let rle2 = Rle {
            lengths: vec![1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2,
                          2, 1, 1, 1, 4, 2, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 5, 2, 2, 2, 2, 1, 2,
                          3, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1],
            values: vec![1, 3, 1, 3, 2, 1, 3, 2, 1, 3, 2, 3, 1, 2, 1, 3, 1, 2, 1, 3, 2, 1, 3, 2,
                         1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 1, 3, 1, 2, 3, 2, 1, 2, 3, 2, 1, 3, 1, 3,
                         2, 1, 2, 1, 2, 1, 3, 2, 3, 1, 2, 3, 1, 3, 2, 3, 2, 3],
        };

        let expected_result = Rle {
            lengths: vec![1, 1, 1, 1, 4, 1, 1, 1, 2, 2, 1, 2, 2, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 3,
                          1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 6, 3, 2, 1, 1,
                          1, 2, 1, 1, 1, 3, 1, 3, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1],
            values: vec![3, 4, 5, 2, 5, 4, 5, 4, 2, 4, 6, 4, 3, 6, 5, 4, 3, 4, 3, 4, 5, 4, 5, 4,
                         5, 4, 5, 6, 5, 4, 3, 2, 5, 4, 6, 2, 4, 3, 5, 3, 5, 4, 3, 5, 4, 5, 4, 6,
                         5, 4, 5, 2, 3, 4, 3, 5, 3, 2, 4, 2, 5, 3, 4, 5, 3, 5],
        };

        println!("Test starts");
        let actual_result = rle1.add(&rle2);
        println!("\nExpected lengths: {:?}", expected_result.lengths);
        println!("Actual lengths: {:?}\n", actual_result.lengths);
        println!("Expected values: {:?}", expected_result.values);
        println!("Actual values: {:?}", actual_result.values);
        assert_eq!(actual_result, expected_result);

    }

}
