extern crate num;
use std::collections::HashMap;
use std::cmp::min;


#[derive(Debug, PartialEq)]
pub struct Rle {
    pub lengths: Vec<i32>,
    pub values: Vec<i32>,
    pub data: HashMap<String, Vec<i32>>,
}


impl Rle {
    pub fn add(&self, other: &Rle) -> Rle {

        fn unpack(n: Option<i32>) -> i32 {
            match n {
                Some(x) => x,
                None => 0,
            }

        }

        let mut ls1 = self.lengths.clone();
        let mut ls2 = other.lengths.clone();

        let mut vs1 = self.values.clone();
        let mut vs2 = other.values.clone();

        let mut new_values = Vec::<i32>::new();
        let mut new_lengths = Vec::<i32>::new();

        while ls1.len() + ls2.len() > 0 {

            let l1 = unpack(ls1.pop());
            let l2 = unpack(ls2.pop());

            let v1 = unpack(vs1.pop());
            let v2 = unpack(vs2.pop());

            // println!("New iteration:");
            // println!("\tv1: {:?}, l1: {:?}", v1, l1);
            // println!("\tv2: {:?}, l2: {:?}", v2, l2);

            if l1 < l2 {

                new_values.push(v1 + v2);
                new_lengths.push(l1);

                let mut left_l2 = l2 - l1;

                while left_l2 > 0 {
                    let v1 = unpack(vs1.pop());
                    let l1 = unpack(ls1.pop());

                    if l1 == 0 {
                        break;
                    }

                    let new_length = min(l1, left_l2);

                    left_l2 -= l1;

                    new_lengths.push(new_length);
                    new_values.push(v1 + v2);

                    if left_l2 < 0 {
                        ls1.push(left_l2.abs());
                        vs2.push(v2);
                    }

                }

                // println!("First case");
                // println!("\t{:?} new_lengths.", new_lengths);
                // println!("\t{:?} new_values.", new_values);


            } else if l2 < l1 {

                new_values.push(v2 + v1);
                new_lengths.push(l2);

                let mut left_l1 = l1 - l2;

                while left_l1 > 0 {
                    let v2 = unpack(vs2.pop());
                    let l2 = unpack(ls2.pop());

                    let new_length = min(l2, left_l1);

                    if l2 == 0 {
                        break;
                    }

                    left_l1 -= l2;

                    new_lengths.push(new_length);
                    new_values.push(v2 + v1);

                    if left_l1 < 0 {
                        ls2.push(left_l1.abs());
                        vs2.push(v2);
                    }
                    // println!("{:?} Second case left_l1:", left_l1);
                    // println!("{:?} Second case l2:", l2);
                    // println!("{:?} Second case v2:", v2);
                }


                // println!("Second case");
                // println!("\t{:?} new_lengths.", new_lengths);
                // println!("\t{:?} new_values.", new_values);

            } else if l2 == l1 {

                new_values.push(v2 + v1);
                new_lengths.push(l2);

                println!("Third case:");
                println!("\t{:?} new_lengths.", new_lengths);
                println!("\t{:?} new_values.", new_values);
            }

            // println!("{:?} new_lengths.", new_lengths);
            // println!("{:?} new_values.", new_values);

        }

        // let mut dedup_vals_values = Vec::<i32>::new();
        // let mut dedup_vals_lengths = Vec::<i32>::new();

        // let mut v1 = unpack(new_values.pop());
        // let mut l1 = unpack(new_lengths.pop());

        // for (v2, l2) in new_values.iter().zip(new_lengths.iter()) {

        //     if v1 != *v2 {
        //         dedup_vals_lengths.push(l1);
        //         dedup_vals_values.push(v1);
        //         v1 = *v2;
        //         l1 = *l2;
        //     } else {
        //         l1 += *l2
        //     }
        // }


        new_values.reverse();
        new_lengths.reverse();

        Rle {
            lengths: new_lengths,
            values: new_values,
            data: HashMap::new(),
        }


        // dedup_vals_values.reverse();
        // dedup_vals_lengths.reverse();

        // Rle {
        //     lengths: dedup_vals_lengths,
        //     values: dedup_vals_values,
        //     data: HashMap::new(),
        // }
    }
}
