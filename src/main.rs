use std::collections::HashMap;
#[allow(dead_code)]
#[derive(Debug)]
struct Left {
    val: i32,
}

pub struct Solution;
fn main() {
    Solution::del_repeat_double_pointer();
    Solution::best_oppor();
    Solution::rotate_array();
    Solution::dupli_exist();
    Solution::appear_single_time_num();
    Solution::plus_one_fn();
    Solution::mv_zero();
    Solution::two_sum_fn();
    Solution::del_repeat_double_pointer();
    Solution::intersect_fn();
    println!(
        "{}",
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ])
    );
    Solution::t();
}

#[allow(unused)]
impl Solution {
    fn del_repeat_double_pointer() {
        let mut nums = vec![1, 2, 2, 3, 4, 5, 5, 5, 7, 8, 9, 9, 9];

        let mut fast = 1;
        let mut slow = 1;

        if nums.len() == 0 {
            ()
        }
        while fast < nums.len() {
            if nums[fast] != nums[fast - 1] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        for n in (slow..nums.len()).rev() {
            //println!("{}, slow {}",n, slow);
            nums.remove(n);
            //println!("{}",nums[n]);
        }
        println!("{:?}", nums);
    }

    fn best_oppor() {
        let prices = vec![7, 6, 4, 3, 1];

        let mut prof: i32 = 0;
        for (index, val) in prices.clone().into_iter().enumerate() {
            if index >= 1 {
                let judge = val - prices.get(index - 1).unwrap();
                if judge > 0 {
                    prof += judge;
                }
            }
        }
        println!("{}", prof);
    }

    fn rotate_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;

        for i in 0..k {
            let hand = nums.get(nums.len() - 1).unwrap().clone();
            nums.pop();
            nums.insert(0, hand)
        }

        println!("{:?}", nums);
    }

    fn dupli_exist() {
        let nums = vec![1, 2, 4, 3, 5];

        //nums.sort();
        let mut map: HashMap<usize, i32> = HashMap::new();
        //map = nums.clone().into_iter().enumerate().collect();
        for i in nums.iter() {
            map.insert(*i as usize, 0);
            // println!("{:?}", map);
            if map.len() == nums.len() {
                println!("true");
            }
        }
    }

    fn appear_single_time_num() {
        let nums = vec![1, 1, 2, 3, 3, 4, 4];

        let mut i = 0;
        for j in nums {
            i = i ^ j;
        }
        assert_eq!(2, i);
    }

    fn intersect_fn() {
        let nums1 = vec![1, 2, 4]; //fast
        let nums2 = vec![2, 4]; //slow

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec: Vec<i32> = Vec::new();

        for e in nums1.iter() {
            if map.contains_key(e) {
                map.insert(*e, map[e] + 1); // the v counts times e appears
            } else {
                map.insert(*e, 1);
            }
        }
        for e in nums2.iter() {
            if map.contains_key(e) {
                if map[e] > 0 {
                    vec.push(*e)
                }
                map.insert(*e, map[e] - 1);
            }
        }
        println!("{:?}", vec);
    }

    fn plus_one_fn() {
        let digits = vec![1, 2, 4, 3];

        if !digits.contains(&9) {}
        println!("{:?}", digits);
    }
    fn test() {
        let i = 0 ^ 2;
        println!("{}", i);
    }

    fn mv_zero() {
        let mut nums = vec![0, 1, 4, 0, 0, 3, 12];

        let mut i = 0;
        let mut j = 0;
        while i < nums.len() {
            if nums[i] == 0 && j < nums.len() - i {
                nums.remove(i);
                nums.push(0);
                j += 1;
            } else {
                i += 1;
            }
        }
        println!("{:?}", nums);
    }
    fn two_sum_fn() -> Vec<i32> {
        let nums = vec![2, 7, 11, 15];

        let target = 9;
        let mut map: HashMap<_, _> = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            let other = target - value;
            if let Some(&other_index) = map.get(&other) {
                return vec![other_index, index as i32];
            }
            map.insert(value, index as i32);
        }
        vec![]
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = [0i32; 9];
        let mut col = [0i32; 9];

        let mut cell = [0i32; 9];

        let mut temp = 0i32;

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                temp = 1 << board[i][j] as i32 - '1' as i32;

                let p = i / 3 * 3 + j / 3;

                if (col[i] & temp > 0) || (row[j] & temp) > 0 || (cell[p] & temp) > 0 {
                    return false;
                }

                col[i] |= temp;
                row[j] |= temp;
                cell[p] |= temp;
            }
        }
        true
    }
}
