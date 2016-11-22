extern crate ranges;


pub use ranges::float_rle::FloatRle;
pub use ranges::float_rle::extend_floatrle;

#[cfg(test)]

mod tests {
    use super::FloatRle;
    use super::extend_floatrle;


    #[test]
    fn create_new_rle1() {

        let lengths = vec![1];
        let values = vec![0.0];
        println!("Test starting");
        let actual_result = FloatRle::new(lengths, values);

        let expected_result = FloatRle {
            lengths: vec![1],
            values: vec![0.0],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_rle2() {

        let lengths = vec![1, 1];
        let values = vec![0.0, 1.1];
        println!("Test starting");
        let actual_result = FloatRle::new(lengths, values);

        let expected_result = FloatRle {
            lengths: vec![1, 1],
            values: vec![0.0, 1.1],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }


    #[test]
    fn create_new_rle3() {

        let lengths = vec![1, 1, 2];
        let values = vec![1.1, 1.1, 2.0];
        println!("Test starting");
        let actual_result = FloatRle::new(lengths, values);

        let expected_result = FloatRle {
            lengths: vec![2, 2],
            values: vec![1.1, 2.0],
        };

        println!("Actual {:?}", actual_result);
        println!("Expected {:?}", expected_result);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_rle4() {

        let lengths = vec![100, 1, 1, 2];
        let values = vec![0.5, 1.0, 1.0, 2.0];
        println!("Test starting");
        let actual_result = FloatRle::new(lengths, values);

        let expected_result = FloatRle {
            lengths: vec![100, 2, 2],
            values: vec![0.5, 1.0, 2.0],
        };
    }


        #[test]
        fn add_two_rles1() {

            let rle1 = FloatRle {
                lengths: vec![1, 1],
                values: vec![0.0, 1.0],
            };

            let rle2 = FloatRle {
                lengths: vec![2],
                values: vec![1.0],
            };

            let expected_result = FloatRle {
                lengths: vec![1, 1],
                values: vec![1.0, 2.0],
            };

            println!("Test starts");

            let actual_result = rle1.op_float(&rle2, |x, y| x + y, 0 as f64);

            println!("Result struct: {:?}", actual_result);

            assert_eq!(expected_result, actual_result);


        }


        #[test]
        fn add_two_rles2() {

            let rle1 = FloatRle {
                lengths: vec![1, 4, 2],
                values: vec![1.0, 2.0, 1.0],
            };

            let rle2 = FloatRle {
                lengths: vec![3, 3, 1],
                values: vec![1.0, 0.0, 1.0],
            };


            let expected_result = FloatRle {
                lengths: vec![1, 2, 2, 1, 1],
                values: vec![2.0, 3.0, 2.0, 1.0, 2.0],
            };

            println!("Test starts");
            let actual_result = rle1.op_float(&rle2, |x, y| x + y, 0 as f64);
            println!("\nExpected lengths: {:?}", expected_result.lengths);
            println!("Actual lengths: {:?}\n", actual_result.lengths);
            println!("Expected values: {:?}", expected_result.values);
            println!("Actual values: {:?}", actual_result.values);
            assert_eq!(actual_result, expected_result);

        }


    #[test]
    fn add_two_rles6() {

        let rle1 = FloatRle {
            lengths: vec![1, 1, 1, 1, 2, 3, 2, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1,
                          1, 2, 1, 2, 2, 1, 4, 1, 3, 2, 1, 1, 2, 1, 1, 2, 2, 3, 2, 1, 1, 2, 2, 2,
                          3, 1, 2, 1, 4, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1],
            values: vec![2.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 3.0,
                         2.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 1.0, 3.0, 1.0, 2.0, 3.0, 1.0, 2.0, 1.0, 2.0, 1.0, 3.0, 2.0, 3.0, 2.0, 3.0, 2.0, 3.0,
                         2.0, 3.0, 1.0, 3.0, 2.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 3.0, 1.0, 2.0],
        };

        let rle2 = FloatRle {
            lengths: vec![1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2,
                          2, 1, 1, 1, 4, 2, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 5, 2, 2, 2, 2, 1, 2,
                          3, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1],
            values: vec![1.0, 3.0, 1.0, 3.0, 2.0, 1.0, 3.0, 2.0, 1.0, 3.0, 2.0, 3.0, 1.0, 2.0, 1.0, 3.0, 1.0, 2.0, 1.0, 3.0, 2.0, 1.0, 3.0, 2.0,
                         1.0, 3.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 1.0, 3.0, 1.0, 3.0, 1.0, 2.0, 3.0, 2.0, 1.0, 2.0, 3.0, 2.0, 1.0, 3.0, 1.0, 3.0,
                         2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 3.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 3.0, 2.0, 3.0, 2.0, 3.0],
        };

        let expected_result = FloatRle {
            lengths: vec![1, 1, 1, 1, 4, 1, 1, 1, 2, 2, 1, 2, 2, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 3,
                          1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 6, 3, 2, 1, 1,
                          1, 2, 1, 1, 1, 3, 1, 3, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1],
            values: vec![3.0, 4.0, 5.0, 2.0, 5.0, 4.0, 5.0, 4.0, 2.0, 4.0, 6.0, 4.0, 3.0, 6.0, 5.0, 4.0, 3.0, 4.0, 3.0, 4.0, 5.0, 4.0, 5.0, 4.0,
                         5.0, 4.0, 5.0, 6.0, 5.0, 4.0, 3.0, 2.0, 5.0, 4.0, 6.0, 2.0, 4.0, 3.0, 5.0, 3.0, 5.0, 4.0, 3.0, 5.0, 4.0, 5.0, 4.0, 6.0,
                         5.0, 4.0, 5.0, 2.0, 3.0, 4.0, 3.0, 5.0, 3.0, 2.0, 4.0, 2.0, 5.0, 3.0, 4.0, 5.0, 3.0, 5.0],
        };

        println!("Test starts");
        let actual_result = rle1.op_float(&rle2, |x, y| x + y, 0 as f64);
        println!("\nExpected lengths: {:?}", expected_result.lengths);
        println!("Actual lengths: {:?}\n", actual_result.lengths);
        println!("Expected values: {:?}", expected_result.values);
        println!("Actual values: {:?}", actual_result.values);
        assert_eq!(actual_result, expected_result);

        let actual_result2 = rle2.op_float(&rle1, |x, y| x + y, 0 as f64);
        assert_eq!(actual_result2, expected_result);

    }


    #[test]
    fn add_two_rles7_one_is_wrong_length() {

        let rle1 = FloatRle {
            lengths: vec![1, 1, 1],
            values: vec![0.0, 1.0, 2.0],
        };

        let rle2 = FloatRle {
            lengths: vec![2],
            values: vec![1.0],
        };

        let expected_result = FloatRle {
            lengths: vec![1, 2],
            values: vec![1.0, 2.0],
        };

        println!("Test starts");

        let actual_result = rle1.op_float(&rle2, |x, y| x + y, 0.0);

        println!("Result struct: {:?}", actual_result);

        assert_eq!(expected_result, actual_result);
        // assert_eq!(rle1.op_int((&rle2, |x, y|  x + y, 0), expected_result);


    }



    #[test]
    fn extend_intrle1() {

        let mut rle1 = FloatRle {
            lengths: vec![1, 1],
            values: vec![1.0, 2.0],
        };

        let mut rle2 = FloatRle {
            lengths: vec![2, 1, 1],
            values: vec![3.0, 1.0, 2.0],
        };


        let expected_rle2 = FloatRle {
            lengths: vec![2, 1, 1],
            values: vec![3.0, 1.0, 2.0],
        };

        let expected_result = (FloatRle {
            lengths: vec![1, 1, 2],
            values: vec![1.0, 2.0, 0.0],
        },
                               expected_rle2);

        extend_floatrle(&mut rle1.lengths,
                      &mut rle1.values,
                      &mut rle2.lengths,
                      &mut rle2.values,
                      0.0);

        assert_eq!((rle1, rle2), expected_result);
    }

}


//     #[test]
//     fn add_two_rles3() {

//         let rle1 = IntRle {
//             lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//             values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//         };

//         let rle2 = IntRle {
//             lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//             values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//         };


//         let expected_result = IntRle {
//             lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//             values: vec![2, 4, 6, 8, 10, 12, 14, 16, 18],
//         };

//         println!("Test starts");
//         let actual_result = rle1.op_int(&rle2, |x, y| x + y, 0);
//         println!("\nExpected lengths: {:?}", expected_result.lengths);
//         println!("Actual lengths: {:?}\n", actual_result.lengths);
//         println!("Expected values: {:?}", expected_result.values);
//         println!("Actual values: {:?}", actual_result.values);
//         assert_eq!(actual_result, expected_result);


//     }


//     #[test]
//     fn add_two_rles4() {

//         let rle1 = IntRle {
//             lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//             values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//         };

//         let rle2 = IntRle {
//             lengths: vec![45],
//             values: vec![10],
//         };


//         let expected_result = IntRle {
//             lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
//             values: vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
//         };

//         println!("Test starts");
//         let actual_result = rle1.op_int(&rle2, |x, y| x + y, 0);
//         println!("\nExpected lengths: {:?}", expected_result.lengths);
//         println!("Actual lengths: {:?}\n", actual_result.lengths);
//         println!("Expected values: {:?}", expected_result.values);
//         println!("Actual values: {:?}", actual_result.values);
//         assert_eq!(actual_result, expected_result);

//     }

//     #[test]
//     fn add_two_rles5() {

//         let rle1 = IntRle {
//             lengths: vec![1, 1, 3, 3, 1, 1],
//             values: vec![1, 3, 1, 2, 3, 1],
//         };

//         let rle2 = IntRle {
//             lengths: vec![1, 2, 1, 1, 1, 3, 1],
//             values: vec![1, 2, 1, 3, 2, 1, 3],
//         };

//         let expected_result = IntRle {
//             lengths: vec![1, 1, 1, 1, 2, 2, 2],
//             values: vec![2, 5, 3, 2, 4, 3, 4],
//         };

//         println!("Test starts");
//         let actual_result = rle1.op_int(&rle2, |x, y| x + y, 0);
//         println!("\nExpected lengths: {:?}", expected_result.lengths);
//         println!("Actual lengths: {:?}\n", actual_result.lengths);
//         println!("Expected values: {:?}", expected_result.values);
//         println!("Actual values: {:?}", actual_result.values);
//         assert_eq!(actual_result, expected_result);

//     }




//     #[test]
//     fn deduct_two_rles1() {

//         let rle1 = IntRle {
//             lengths: vec![1, 1, 1, 1, 2, 3, 2, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1,
//                           1, 2, 1, 2, 2, 1, 4, 1, 3, 2, 1, 1, 2, 1, 1, 2, 2, 3, 2, 1, 1, 2, 2, 2,
//                           3, 1, 2, 1, 4, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1],
//             values: vec![2, 1, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 3, 1, 2, 1, 2, 3, 2, 1, 3,
//                          2, 3, 2, 3, 2, 3, 2, 1, 3, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 3, 2, 3, 2, 3,
//                          2, 3, 1, 3, 2, 3, 2, 1, 2, 1, 2, 1, 2, 3, 1, 2],
//         };

//         let rle2 = IntRle {
//             lengths: vec![1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2,
//                           2, 1, 1, 1, 4, 2, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 5, 2, 2, 2, 2, 1, 2,
//                           3, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1],
//             values: vec![1, 3, 1, 3, 2, 1, 3, 2, 1, 3, 2, 3, 1, 2, 1, 3, 1, 2, 1, 3, 2, 1, 3, 2,
//                          1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 1, 3, 1, 2, 3, 2, 1, 2, 3, 2, 1, 3, 1, 3,
//                          2, 1, 2, 1, 2, 1, 3, 2, 3, 1, 2, 3, 1, 3, 2, 3, 2, 3],
//         };

//         let expected_result = IntRle {
//             lengths: vec![1, 1, 1, 1, 2, 2, 1, 1, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1,
//                           1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1, 2,
//                           3, 2, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 2,
//                           1, 1, 1, 2],
//             values: vec![1, -2, -1, 0, -1, 1, 2, -1, 0, -2, 0, 2, 0, -1, 1, 0, -1, 2, -1, 1, -2,
//                          -1, 1, 2, -1, -2, 1, 0, 2, -1, 2, 1, -1, 0, -1, 0, 1, 0, 1, 2, 0, -2, 1,
//                          -1, -2, 0, 1, -1, 1, -1, 1, 0, 2, -1, 2, 0, -1, 0, 1, 0, -1, 2, 0, 1, -1,
//                          1, -1, 1, 0, -2, 0, -1, -2, 0, 1, -1],
//         };

//         println!("Test starts");
//         let actual_result = rle1.op_int(&rle2, |x, y| x - y, 0);
//         println!("\nExpected lengths: {:?}", expected_result.lengths);
//         println!("Actual lengths: {:?}\n", actual_result.lengths);
//         println!("Expected values: {:?}", expected_result.values);
//         println!("Actual values: {:?}", actual_result.values);
//         assert_eq!(actual_result, expected_result);

//     }


//     #[test]
//     fn multiply_two_rles1() {

//         let rle1 = IntRle {
//             lengths: vec![1, 1, 1, 1, 2, 3, 2, 3, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1,
//                           1, 2, 1, 2, 2, 1, 4, 1, 3, 2, 1, 1, 2, 1, 1, 2, 2, 3, 2, 1, 1, 2, 2, 2,
//                           3, 1, 2, 1, 4, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1],
//             values: vec![2, 1, 2, 1, 2, 3, 2, 1, 2, 3, 2, 1, 2, 3, 2, 3, 1, 2, 1, 2, 3, 2, 1, 3,
//                          2, 3, 2, 3, 2, 3, 2, 1, 3, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 3, 2, 3, 2, 3,
//                          2, 3, 1, 3, 2, 3, 2, 1, 2, 1, 2, 1, 2, 3, 1, 2],
//         };

//         let rle2 = IntRle {
//             lengths: vec![1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 1, 2, 2, 2,
//                           2, 1, 1, 1, 4, 2, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 5, 2, 2, 2, 2, 1, 2,
//                           3, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1],
//             values: vec![1, 3, 1, 3, 2, 1, 3, 2, 1, 3, 2, 3, 1, 2, 1, 3, 1, 2, 1, 3, 2, 1, 3, 2,
//                          1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 1, 3, 1, 2, 3, 2, 1, 2, 3, 2, 1, 3, 1, 3,
//                          2, 1, 2, 1, 2, 1, 3, 2, 3, 1, 2, 3, 1, 3, 2, 3, 2, 3],
//         };

//         let expected_result = IntRle {
//             lengths: vec![1, 1, 1, 1, 4, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 1, 1, 1,
//                           1, 1, 2, 1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 6,
//                           1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
//                           1, 1],
//             values: vec![2, 3, 6, 1, 6, 3, 6, 4, 1, 3, 4, 9, 3, 4, 2, 9, 6, 3, 2, 3, 2, 3, 6, 3,
//                          6, 4, 3, 6, 3, 6, 9, 6, 4, 2, 1, 6, 3, 9, 1, 3, 2, 6, 2, 6, 3, 4, 2, 6,
//                          4, 3, 6, 3, 9, 6, 4, 6, 1, 2, 3, 4, 2, 6, 2, 1, 4, 3, 1, 6, 2, 3, 4, 6,
//                          2, 6],
//         };

//         println!("Test starts");
//         let actual_result = rle1.op_int(&rle2, |x, y| x * y, 1);
//         println!("\nExpected lengths: {:?}", expected_result.lengths);
//         println!("Actual lengths: {:?}\n", actual_result.lengths);
//         println!("Expected values: {:?}", expected_result.values);
//         println!("Actual values: {:?}", actual_result.values);
//         assert_eq!(actual_result, expected_result);

//     }


// }
