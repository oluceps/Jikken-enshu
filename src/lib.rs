pub mod algo {


    pub fn del_repeat(mut nums: Vec<i32>) -> Vec<i32> {

        let mut fast= 1;
        let mut slow= 1;

        if nums.len() == 0{
            ()
        }
        while fast < nums.len() {
            if nums[fast] != nums[fast - 1] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        for n in (slow..nums.len()).rev(){
            //println!("{}, slow {}",n, slow);
            nums.remove(n);
            //println!("{}",nums[n]);
        }

        return nums
    }

    pub fn best_opportunity(nums: &Vec<i32>) -> i32 {

        let mut prof :i32 = 0;
        for (index,val) in nums.into_iter().enumerate(){
            if index >= 1{
                let judge = val - nums.get(index - 1).unwrap();
                if  judge > 0{
                    prof += judge;
                }
            }
        }
        prof
    }

    use std::fmt::Display;


    pub fn rotate(nums: &mut Vec<i32>, k: i32) -> &mut Vec<i32> {
        for i in 0..k{
            let hand = nums.get(nums.len() - 1).unwrap().clone();
            nums.pop();
            nums.insert(0, hand)
        }
        nums
    }

    use std::collections::HashMap;

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        //nums.sort();
        let mut map:HashMap<usize, i32> = HashMap::new();
        //map = nums.clone().into_iter().enumerate().collect();
        for i in nums.iter(){
            map.insert(*i as usize, 0);
            // println!("{:?}", map);
            if map.len() == nums.len(){
                return true
            }
        }

        false
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        for j in nums{
            i = i ^ j;
        }
        return i

    }


    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {


        use std::collections::HashMap;
        let mut map:HashMap<i32,i32> = HashMap::new();
        let mut vec:Vec<i32> = Vec::new();

        for e in nums1.iter(){
            if map.contains_key(e){
                map.insert(*e, map[e]+1); // the v counts times e appears
            }else{
                map.insert(*e, 1);
            }
        }
        for e in nums2.iter(){

            if map.contains_key(e){
                if  map[e] > 0 {
                    vec.push(*e)
                }
                map.insert(*e,map[e]-1);
            }
        }

        return vec;

    }

    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {

        if !digits.contains(&9){
            ;
        }
        digits

    }

    pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        while i < nums.len(){

            if nums[i] == 0 && j < nums.len() - i{
                nums.remove(i);
                nums.push(0);
                j += 1;
            }else {
                i += 1;
            }
        }
        nums

    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<_,_> = HashMap::new();

        for (index, value) in nums.iter().enumerate(){
            let other = target - value;
            if let Some(&other_index) = map.get(&other){
                return vec![other_index, index as i32]
            }
            map.insert(value, index as i32);
        }
        vec![]
    }



}


