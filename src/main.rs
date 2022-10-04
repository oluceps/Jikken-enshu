use std::ops::RangeFrom;
use rustprac::algo::*;
#[allow(dead_code)]
#[derive(Debug)]
struct Left {
    val: i32,
}
fn main() {

    //del_repeat_double_pointer()
    //best_oppor();
    //rotate_array()
    //dupli_exist()
    //single_num()
    //intersect_fn()
    //plus_one_fn()
    //mv_zero()
    //two_sum_fn()


}


fn del_repeat_double_pointer(){
    let nums = vec![1,2,2,3,4,5,5,5,7,8,9,9,9];
    println!("{:?}",del_repeat(nums));
}

fn best_oppor(){
    let prices = vec![7,6,4,3,1];
    println!("{}",best_opportunity(&prices));
}

fn rotate_array(){
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    println!("{:?}",rotate(&mut nums, k));
}

fn dupli_exist(){
    let nums = vec![1,2,4,3,5];
    assert_eq!(contains_duplicate(nums), false);
}

fn single_num(){
    let nums = vec![1,1,2,3,3,4,4];
    assert_eq!(2,single_number(nums));
}

fn intersect_fn(){
    let nums1 = vec![1,2,4]; //fast
    let nums2 = vec![2,4]; //slow
    println!("{:?}", intersect(nums1, nums2));

}

fn plus_one_fn(){
    let nums = vec![1,2,4,3];
    println!("{:?}", plus_one(nums));
}
fn test(){
    let i = 0^2;
    println!("{}", i);
}

fn mv_zero(){
    let mut nums = vec![0, 1,4,0, 0, 3, 12];
    println!("{:?}", move_zeroes(&mut nums));
}
fn two_sum_fn(){
    let nums = vec![2,7,11,15];
    println!("{:?}", two_sum(nums, 9));
}