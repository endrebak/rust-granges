mod rle;

#[cfg(test)]

mod tests {
    use super::rle::Rle;
    use std::collections::HashMap;

    #[test]
    fn add_two_rles1() {

        let rle1 = Rle {
            lengths: vec![1, 1],
            values: vec![0, 1],
            data: HashMap::new() //<String, Vec<T>>,
        };

        let rle2 = Rle {
            lengths: vec![2],
            values: vec![1],
            data: HashMap::new() //<String, Vec<T>>,
        };

        let expected_result = Rle {
            lengths: vec![1, 1],
            values: vec![1, 2],
            data: HashMap::new() //<String, Vec<T>>,
        };

        println!("Test starts");

        let actual_result = rle1.add(&rle2);

        println!("Result struct: {:?}", actual_result);

        assert_eq!(expected_result, actual_result);
        // assert_eq!(rle1.add(&rle2), expected_result);


    }

    #[test]
    fn add_two_rles2() {

        let rle1 = Rle {
            lengths: vec![1, 4, 2],
            values: vec![1, 2, 1],
            data: HashMap::new() //<String, Vec<T>>,
        };

        let rle2 = Rle {
            lengths: vec![3, 3, 1],
            values: vec![1, 0, 1],
            data: HashMap::new() //<String, Vec<T>>,
        };


        let expected_result = Rle {
            lengths: vec![1, 2, 2, 1, 1],
            values: vec![2, 3, 2, 1, 2],
            data: HashMap::new() //<String, Vec<T>>,
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
    fn add_two_rles3() {

        let rle1 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            data: HashMap::new() //<String, Vec<T>>,
        };

        let rle2 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            data: HashMap::new() //<String, Vec<T>>,
        };


        let expected_result = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![2, 4, 6, 8, 10, 12, 14, 16, 18],
            data: HashMap::new() //<String, Vec<T>>,
        };



    }


    #[test]
    fn add_two_rles4() {

        let rle1 = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            data: HashMap::new() //<String, Vec<T>>,
        };

        let rle2 = Rle {
            lengths: vec![45],
            values: vec![10],
            data: HashMap::new() //<String, Vec<T>>,
        };


        let expected_result = Rle {
            lengths: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            values: vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
            data: HashMap::new() //<String, Vec<T>>,
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
            data: HashMap::new() //<String, Vec<T>>,
        };

        let rle2 = Rle {
            lengths: vec![1, 2, 1, 1, 1, 3, 1],
            values: vec![1, 2, 1, 3, 2, 1, 3],
            data: HashMap::new() //<String, Vec<T>>,
        };


        let expected_result = Rle {
            lengths: vec![1, 1, 1, 1, 2, 2, 2],
            values: vec![2, 5, 3, 2, 4, 3, 4],
            data: HashMap::new() //<String, Vec<T>>,
        };
        println!("Test starts");
        let actual_result = rle1.add(&rle2);
        println!("\nExpected lengths: {:?}", expected_result.lengths);
        println!("Actual lengths: {:?}\n", actual_result.lengths);
        println!("Expected values: {:?}", expected_result.values);
        println!("Actual values: {:?}", actual_result.values);
        assert_eq!(actual_result, expected_result);

    }


    // #[test]
    // fn add_two_rles5() {

    //     let rle1 = Rle {
    //         lengths: vec![2, 1, 3, 2, 1, 3, 2, 3, 2, 3, 2, 3, 2, 1, 2, 1, 3, 2, 3, 2, 3, 1, 3, 2, 1, 2, 3, 2, 3, 1, 3, 2, 3, 1, 3, 1, 2, 3, 2, 3, 1, 3, 1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 2, 1, 3, 1, 2, 3, 2, 1, 2, 1, 3, 2, 1, 3, 1, 3, 1, 2, 3, 2, 1, 3, 2, 3, 2, 1, 3, 1, 2, 1, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 1, 3, 1, 3, 1, 2, 3, 1, 2, 3, 1, 3, 1, 2, 1, 3, 2, 3, 2, 1, 3, 1, 3, 2, 1, 3, 2, 3, 1, 2, 3, 1, 2, 1, 2, 3, 2, 1, 2, 3, 1, 2, 3, 2, 3, 2, 3, 2, 1, 2, 1, 3, 2, 1, 2, 1, 3, 1, 2, 3, 1, 2, 3, 2, 1, 2, 3, 1, 2, 1, 2, 3, 2, 3, 1, 2, 1, 2, 3, 2, 3, 1, 2, 3, 1, 2, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 3, 2, 1, 3, 1, 2, 1, 2, 3, 2, 1, 2, 1, 2, 3, 2, 1, 3, 2, 3, 2, 3, 2, 1, 2, 1, 3, 1, 3, 1, 3, 2, 3, 2, 1, 2, 3, 1, 3, 1, 2, 3, 1, 3, 2, 1, 3, 1, 2, 3, 1, 3, 1, 3, 1, 2, 3, 1, 3, 1, 3, 2, 3, 1, 2, 1, 2, 1, 2, 3, 1, 2, 3, 1, 2, 3, 2, 1, 2, 3, 1, 3, 1, 3, 1, 3, 2, 3, 2, 1, 2, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2, 3, 1, 2, 3, 2, 3, 1, 3, 1, 2, 1, 2, 1, 2, 3, 2, 3, 1, 3, 2, 3, 1, 2, 3, 1, 2, 3, 2, 3, 2, 3, 1, 3, 1, 3, 1, 2, 3, 1, 2, 3, 1, 3, 1, 2, 3, 1, 3, 1, 2, 3],
    //         values: vec![1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 3, 4, 3, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 2, 3, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 3, 1, 2, 6, 1, 1, 1, 1, 3, 3, 2, 1, 1, 1, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 3, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 3, 1, 1, 2, 1, 2, 1, 1, 3, 3, 1, 1, 1, 2, 2, 2, 3, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 4, 1, 1, 5, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 2, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 1, 4, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 2, 2, 1, 1, 4, 3, 1, 2, 3, 4, 2, 3, 2, 1, 1, 1, 3, 1, 2, 2, 1, 1, 1, 2, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    //         data: HashMap::new() //<String, Vec<T>>,
    //     };

    //     let rle2 = Rle {
    //         lengths: vec![2, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 2, 1, 1, 1, 5, 1, 1, 4, 1, 1, 1, 1, 1, 1, 3, 1, 4, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 3, 2, 1, 2, 1, 1, 1, 2, 1, 1, 3, 6, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 1, 2, 2, 1, 2, 2, 4, 1, 3, 1, 1, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 1, 1, 3, 4, 1, 1, 1, 3, 1, 3, 1, 2, 1, 3, 3, 1, 3, 1, 2, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 2, 2, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 3, 2, 1, 2, 1, 1, 1, 1, 1, 3, 1, 2, 3, 1, 3, 1, 2, 1, 2, 1, 2, 1, 1, 2, 1, 2, 3, 2, 1, 2, 1, 1, 1, 4, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1, 3, 2, 1, 1, 4, 3, 1, 3, 1, 1, 3, 2, 1, 1, 2, 2, 4, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 2, 8, 1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 2, 1, 1, 2, 1, 3, 2, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 4, 1, 1, 1, 1, 4, 1, 2, 1, 1, 2, 3, 1, 1, 2, 1, 1, 1, 1, 1, 3, 1, 2, 1, 1, 2, 2, 1, 1, 1, 4, 1, 2, 1, 1, 1, 3, 2, 1],
    //         values: vec![1, 3, 1, 2, 3, 2, 1, 3, 1, 3, 1, 2, 3, 1, 3, 2, 1, 3, 1, 3, 1, 2, 1, 3, 1, 2, 3, 2, 3, 1, 3, 1, 2, 3, 1, 3, 2, 1, 2, 3, 2, 3, 2, 3, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 1, 3, 1, 2, 3, 2, 3, 2, 1, 3, 1, 2, 1, 3, 2, 3, 2, 1, 2, 1, 2, 1, 2, 3, 2, 1, 3, 1, 2, 1, 2, 3, 2, 3, 2, 3, 2, 3, 1, 3, 1, 2, 1, 3, 1, 3, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 2, 1, 3, 1, 2, 3, 1, 2, 1, 2, 1, 3, 2, 3, 1, 3, 2, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2, 1, 2, 1, 3, 2, 1, 2, 1, 2, 3, 2, 3, 1, 2, 1, 3, 2, 1, 3, 2, 1, 2, 1, 2, 3, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 2, 1, 2, 1, 3, 2, 3, 1, 3, 2, 1, 2, 1, 2, 3, 2, 3, 1, 3, 2, 1, 3, 2, 3, 1, 3, 1, 2, 1, 2, 3, 1, 2, 3, 1, 3, 1, 2, 3, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 2, 1, 3, 2, 3, 2, 1, 3, 2, 1, 3, 2, 1, 2, 1, 2, 3, 1, 2, 1, 2, 1, 2, 3, 2, 1, 3, 1, 2, 1, 3, 2, 3, 2, 1, 2, 3, 1, 3, 2, 3, 1, 2, 3, 1, 3, 1, 2, 1, 3, 2, 3, 1, 3, 2, 1, 2, 3, 1, 3, 2, 3, 2, 1, 2, 1, 2, 3, 2, 3, 1, 2, 1, 3, 2, 1, 2, 1, 3, 1, 3, 1, 2, 3, 1, 3, 2, 1, 2, 3],
    //         data: HashMap::new() //<String, Vec<T>>,
    //     };


    //     let expected_result = Rle {
    //         lengths: vec![1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 5, 1, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 3, 1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 4, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 3, 1, 1, 6, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 4, 1, 2, 1, 1, 1, 1, 1, 2, 3, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 3, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1],
    //         values: vec![3, 2, 6, 3, 6, 5, 3, 5, 6, 5, 4, 5, 3, 4, 6, 3, 4, 3, 5, 4, 5, 4, 6, 3, 6, 4, 5, 2, 6, 3, 4, 5, 6, 4, 5, 3, 5, 6, 4, 5, 3, 4, 2, 3, 6, 4, 5, 6, 3, 5, 3, 6, 4, 5, 4, 5, 6, 4, 3, 4, 6, 4, 3, 4, 3, 4, 3, 4, 3, 5, 4, 5, 4, 3, 4, 2, 5, 6, 3, 5, 4, 5, 6, 5, 4, 5, 2, 3, 4, 2, 3, 4, 5, 6, 5, 4, 3, 6, 5, 4, 3, 4, 5, 4, 6, 4, 5, 4, 5, 4, 5, 4, 3, 5, 6, 2, 6, 2, 4, 5, 2, 5, 4, 6, 5, 2, 3, 4, 3, 4, 3, 6, 3, 5, 3, 2, 4, 3, 2, 6, 4, 3, 5, 4, 5, 3, 4, 2, 4, 2, 3, 2, 4, 6, 5, 2, 4, 2, 4, 5, 3, 6, 5, 6, 3, 4, 5, 2, 5, 4, 6, 4, 3, 6, 2, 4, 5, 4, 5, 3, 5, 6, 3, 2, 4, 2, 3, 4, 5, 6, 3, 4, 2, 4, 2, 4, 5, 4, 6, 5, 3, 5, 3, 4, 5, 4, 5, 4, 3, 5, 3, 4, 2, 5, 4, 5, 4, 5, 3, 2, 5, 2, 3, 4, 5, 4, 3, 5, 4, 3, 4, 3, 5, 3, 4, 3, 4, 5, 4, 6, 3, 5, 6, 4, 3, 4, 5, 6, 4, 3, 4, 3, 2, 4, 6, 4, 6, 2, 4, 6, 4, 6, 3, 4, 2, 3, 4, 5, 4, 3, 4, 2, 5, 3, 4, 3, 4, 3, 4, 5, 4, 6, 5, 4, 2, 3, 4, 5, 2, 3, 6, 3, 4, 5, 3, 5, 4, 5, 2, 4, 5, 3, 5, 3, 5, 4, 3, 5, 3, 6, 4, 5, 4, 2, 6, 4, 3, 5, 4, 5, 2, 4, 3, 4, 5, 6, 2, 3, 5, 6, 4, 3, 4, 3, 5, 2, 4, 2, 4, 5, 4, 5, 6, 5, 6, 4, 2, 4, 3, 4, 5, 4, 5, 6, 2, 4, 5, 4, 5, 2, 3, 2, 3, 4, 6, 4, 6, 3, 4, 5, 2, 4, 6, 5, 2, 5, 3, 6, 4, 6, 2, 3, 5, 4, 6, 2, 5, 2, 4, 3, 4, 6],
    //         data: HashMap::new() //<String, Vec<T>>,
    //     };
    //     println!("Test starts");
    //     let actual_result = rle1.add(&rle2);
    //     println!("\nExpected lengths: {:?}", expected_result.lengths);
    //     println!("Actual lengths: {:?}\n", actual_result.lengths);
    //     println!("Expected values: {:?}", expected_result.values);
    //     println!("Actual values: {:?}", actual_result.values);
    //     assert_eq!(actual_result, expected_result);

    // }


}



// mod rle; // you probably don't want #[cfg(test)] to apply to rle

// #[cfg(test)] // this makes the tests module run only during cargo test
// mod tests {
// use super::rle::Rle; // now Rle is applicable in the tests module

// #[test]
// fn it_works() {
// let rle1 = Rle {
// lengths: vec![1, 4, 2],
// values: vec![1, 2, 1],
// };
// }

// }
