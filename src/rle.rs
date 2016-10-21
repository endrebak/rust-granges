extern crate libc;
use self::libc::{size_t, int32_t};

extern crate num;

use std::cmp::min;
use std::slice;

#[no_mangle]
pub extern "C" fn rle_new(lengths_data: *const int32_t,
                          lengths_length: size_t,
                          values_data: *const int32_t,
                          values_length: size_t)
                          -> *mut Rle {
    let lengths = unsafe { slice::from_raw_parts(lengths_data, lengths_length as usize).to_vec() };
    let values = unsafe { slice::from_raw_parts(values_data, values_length as usize).to_vec() };

    return Box::into_raw(Box::new(Rle::new(lengths, values)));

}

#[no_mangle]
pub extern "C" fn rle_free(ptr: *mut Rle) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}



#[no_mangle]
pub extern "C" fn rle_values_size(rle: *mut Rle) -> int32_t {
    unsafe { (*rle).values.len() as i32 }
}

#[no_mangle]
pub extern "C" fn rle_values(rle: *mut Rle) -> *mut int32_t {
    unsafe { &mut (*rle).values[0] }
}


#[no_mangle]
pub extern "C" fn rle_lengths_size(rle: *mut Rle) -> int32_t {
    unsafe { (*rle).lengths.len() as i32 }
}

#[no_mangle]
pub extern "C" fn rle_lengths(rle: *mut Rle) -> *mut int32_t {
    unsafe { &mut (*rle).lengths[0] }
}


#[derive(Debug, PartialEq)]
pub struct Rle {
    pub lengths: Vec<i32>,
    pub values: Vec<i32>,
}


fn unpack(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x,
        None => 0,
    }

}

impl Rle {
    pub fn new(lengths: Vec<i32>, values: Vec<i32>) -> Self {

        if lengths.len() == 1 {
            return Rle {
                lengths: lengths,
                values: values,
            };
        }

        let mut new_lengths = Vec::<i32>::new();
        let mut new_values = Vec::<i32>::new();

        let mut prev_v = values[0];
        let mut sum_l = lengths[0];


        for (i, (l, v)) in lengths.iter().skip(1).zip(values.iter().skip(1)).enumerate() {
            // println!("Iteration: {:?}", i);
            if *v == prev_v {
                // println!("\t If v: {:?}, prev_v: {:?}, l: {:?}", *v, prev_v, l);
                sum_l += *l;
            } else {
                // println!("\t Else v: {:?}, prev_v: {:?}, l: {:?}", *v, prev_v, l);
                new_lengths.push(sum_l);
                new_values.push(prev_v);
                sum_l = *l;
                prev_v = *v;
            }
            // println!("New lengths: {:?}", new_lengths);
            // println!("New values: {:?}", new_values);
        }

        // println!("prev_v: {:?}", prev_v);
        // println!("new_values[0]: {:?}", new_values[0]);
        if prev_v != new_values[new_values.len() - 1] {
            new_values.push(prev_v);
            new_lengths.push(sum_l);
        } else {
            let last_idx = new_lengths.len() - 1;
            new_lengths[last_idx] += sum_l;
        }

        Rle {
            lengths: new_lengths,
            values: new_values,
        }
    }

    pub fn add(&self, other: &Rle) -> Rle {


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

                    // println!("l1: {:?}, left_l2: {:?}", l1, left_l2);
                    let new_length = min(l1, left_l2);
                    // println!("new_length: {:?}", new_length);

                    left_l2 -= l1;
                    // println!("left_l2: {:?}", left_l2);

                    new_lengths.push(new_length);
                    new_values.push(v1 + v2);

                    if left_l2 < 0 {
                        ls1.push(left_l2.abs());
                        vs1.push(v1);
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
                }


                // println!("Second case");
                // println!("\t{:?} new_lengths.", new_lengths);
                // println!("\t{:?} new_values.", new_values);

            } else if l2 == l1 {

                new_values.push(v2 + v1);
                new_lengths.push(l2);

                // println!("Third case:");
                // println!("\t{:?} new_lengths.", new_lengths);
                // println!("\t{:?} new_values.", new_values);
            }

        }

        new_values.reverse();
        new_lengths.reverse();

        // println!("New values = {:?}", new_values);
        // println!("New lengths = {:?}", new_lengths);
        Rle::new(new_lengths, new_values)
    }
}
