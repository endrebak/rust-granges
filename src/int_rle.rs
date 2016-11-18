extern crate libc;
use self::libc::{size_t, int32_t};

extern crate num;

use std::cmp::min;
use std::slice;


#[no_mangle]
pub extern "C" fn int_rle_new(lengths_data: *const int32_t,
                              lengths_length: size_t,
                              values_data: *const int32_t,
                              values_length: size_t)
                              -> *mut IntRle {
    let lengths = unsafe { slice::from_raw_parts(lengths_data, lengths_length as usize).to_vec() };
    let values = unsafe { slice::from_raw_parts(values_data, values_length as usize).to_vec() };

    return Box::into_raw(Box::new(IntRle::new(lengths, values)));

}


#[no_mangle]
pub extern "C" fn int_rle_free(ptr: *mut IntRle) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}



#[no_mangle]
pub extern "C" fn int_rle_values_size(rle: *mut IntRle) -> int32_t {
    unsafe { (*rle).values.len() as i32 }
}

#[no_mangle]
pub extern "C" fn int_rle_values(rle: *mut IntRle) -> *mut int32_t {
    unsafe { &mut (*rle).values[0] }
}


#[no_mangle]
pub extern "C" fn int_rle_lengths_size(rle: *mut IntRle) -> int32_t {
    unsafe { (*rle).lengths.len() as i32 }
}

#[no_mangle]
pub extern "C" fn int_rle_lengths(rle: *mut IntRle) -> *mut int32_t {
    unsafe { &mut (*rle).lengths[0] }
}


#[no_mangle]
pub extern "C" fn int_rle_add(rle_self: *mut IntRle,
                              rle_other: *mut IntRle,
                              identity: i32)
                              -> *mut IntRle {
    return Box::into_raw(Box::new(unsafe { &*rle_self }
        .op_int(unsafe { &*rle_other }, |x, y| x + y, identity)));
}




#[derive(Debug, PartialEq)]
pub struct IntRle {
    pub lengths: Vec<i32>,
    pub values: Vec<i32>,
}


fn unpack(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x,
        None => 0,
    }

}


pub fn extend_intrle(lengths1: &mut Vec<i32>,
                     values1: &mut Vec<i32>,
                     lengths2: &mut Vec<i32>,
                     values2: &mut Vec<i32>,
                     identity: i32) {

    let sum_l1: i32 = lengths1.iter().sum();
    let sum_l2: i32 = lengths2.iter().sum();

    if sum_l1 < sum_l2 {
        let diff = sum_l2 - sum_l1;

        lengths1.push(diff);
        values1.push(identity);
    } else if sum_l2 < sum_l1 {
        let diff = sum_l1 - sum_l2;

        lengths2.push(diff);
        values2.push(identity);
    }


}


impl IntRle {
    pub fn new(lengths: Vec<i32>, values: Vec<i32>) -> Self {

        if lengths.len() == 1 {
            return IntRle {
                lengths: lengths,
                values: values,
            };
        }

        let mut new_lengths = Vec::<i32>::new();
        let mut new_values = Vec::<i32>::new();

        let mut prev_v = values[0];
        let mut sum_l = lengths[0];


        for (l, v) in lengths.iter().skip(1).zip(values.iter().skip(1)) {
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

        if new_values.len() == 0 {
            new_values.push(prev_v);
            new_lengths.push(sum_l);
        } else if prev_v != new_values[new_values.len() - 1] {
            new_values.push(prev_v);
            new_lengths.push(sum_l);
        } else {
            let last_idx = new_lengths.len() - 1;
            new_lengths[last_idx] += sum_l;
        }

        IntRle {
            lengths: new_lengths,
            values: new_values,
        }
    }

    pub fn op_int<F>(&self, other: &IntRle, op: F, identity: i32) -> IntRle
        where F: Fn(i32, i32) -> i32
    {

        let mut ls1 = self.lengths.clone();
        let mut ls2 = other.lengths.clone();

        let mut vs1 = self.values.clone();
        let mut vs2 = other.values.clone();

        extend_intrle(&mut ls1, &mut vs1, &mut ls2, &mut vs2, identity);


        let mut new_values = Vec::<i32>::new();
        let mut new_lengths = Vec::<i32>::new();

        while ls1.len() + ls2.len() > 0 {

            let l1 = unpack(ls1.pop());
            let l2 = unpack(ls2.pop());

            let v1 = unpack(vs1.pop());
            let v2 = unpack(vs2.pop());

            if l1 < l2 {

                new_values.push(op(v1, v2));
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
                    new_values.push(op(v1, v2));

                    if left_l2 < 0 {
                        ls1.push(left_l2.abs());
                        vs1.push(v1);
                    }

                }

            } else if l2 < l1 {

                new_values.push(op(v1, v2));
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
                    new_values.push(op(v1, v2));

                    if left_l1 < 0 {
                        ls2.push(left_l1.abs());
                        vs2.push(v2);
                    }
                }

            } else if l2 == l1 {

                new_values.push(op(v1, v2));
                new_lengths.push(l2);

            }

        }

        new_values.reverse();
        new_lengths.reverse();

        IntRle::new(new_lengths, new_values)
    }
}
