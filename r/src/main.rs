use std::collections::HashMap;
#[allow(dead_code)]
#[derive(Debug)]
struct Left {
    val: i32,
}

pub struct Solution;
fn main() {
    // Solution::del_repeat_double_pointer();
    // Solution::best_oppor();
    // Solution::rotate_array();
    // Solution::dupli_exist();
    // Solution::appear_single_time_num();
    // Solution::plus_one_fn();
    // Solution::mv_zero();
    // Solution::two_sum_fn();
    // Solution::del_repeat_double_pointer();
    // Solution::intersect_fn();
    // println!(
    //     "{}",
    //     Solution::is_valid_sudoku(vec![
    //         vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    //         vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    //         vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    //         vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    //         vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    //         vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    //         vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    //         vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    //         vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    //     ])
    // );

    // let mut rotate_matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // Solution::rotate(&mut rotate_matrix);
    // println!("{:?}", rotate_matrix)

    // let mut charl = vec!['h', 'e', 'l', 'l', 'o'];
    // Solution::reverse_string(&mut charl);
    // println!("{:?}", charl)

    let mut raw_vec = vec![3, 1, 5, 4, 8, 31, 931, 13399, 113, 0];
    let len = raw_vec.len() as i32;
    Solution::quick_sort(&mut raw_vec, 0, len - 1);
    println!("{:?}", raw_vec);
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
        let mut row = [0u16; 9];
        let mut col = [0u16; 9];

        let mut cell = [0u16; 9];

        let mut temp = 0u16;

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }

                temp = 1 << board[i][j] as u16 - '1' as u16;

                if (col[i] & temp > 0) || (row[j] & temp) > 0 {
                    return false;
                }

                let p = (i / 3 * 3 + j / 3);

                if (cell[p] & temp) > 0 {
                    return false;
                }

                col[i] |= temp;
                row[j] |= temp;
                cell[p] |= temp;
            }
        }
        true
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn floor(x: &i32, y: &i32) -> i32 {
            let mut quotient = x / y;
            let remainder = x % y;
            if remainder > 0 && (x < &0) != (y < &0) {
                quotient -= 1;
            }
            quotient
        }

        let n = matrix.len() as i32;

        for i in 0..(floor(&n, &2)) {
            for j in i..(n - 1 - i) {
                let i = i as usize;
                let j = j as usize;
                let n = n as usize;
                (
                    matrix[i][j],
                    matrix[j][n - 1 - i],
                    matrix[n - 1 - i][n - 1 - j],
                    matrix[n - 1 - j][i],
                ) = (
                    matrix[n - 1 - j][i],
                    matrix[i][j],
                    matrix[j][n - 1 - i],
                    matrix[n - 1 - i][n - 1 - j],
                )
            }
        }
    }

    pub fn reverse_string(s: &mut Vec<char>) {
        let l = 0usize;
        let r = s.len() - 1;

        for i in 0..s.len() / 2 {
            let mut temp: char;
            let r = r - i;

            temp = s.get(r).unwrap().clone();
            *s.get_mut(r).unwrap() = *s.get(i).unwrap();
            *s.get_mut(i).unwrap() = temp;
        }
    }
    pub fn reverse(x: i32) -> i32 {
        if x < 0 {
            let mut v = x.abs().to_string().into_bytes();
            v.reverse();
            let i: i32 = String::from_utf8(v.to_vec())
                .unwrap()
                .parse()
                .unwrap_or_default();
            -i
        } else if x > 0 {
            let mut v = x.to_string().into_bytes();
            v.reverse();
            let i: i32 = String::from_utf8(v.to_vec())
                .unwrap()
                .parse()
                .unwrap_or_default();
            i
        } else {
            0
        }
    }

    pub fn quick_sort(raw: &mut Vec<i32>, left: i32, right: i32) {
        println!("{:?}", raw);
        if left > right {
            return;
        }

        let (left, right) = (left as usize, right as usize);

        let tmp = raw[left];

        let (mut i, mut j) = (left, right);
        while i != j {
            while raw[j] >= tmp && i < j {
                j -= 1;
            }

            while raw[i] <= tmp && i < j {
                i += 1;
            }

            if i < j {
                let t = raw[i];
                raw[i] = raw[j];
                raw[j] = t;
            }
        }
        raw[left] = raw[i];
        raw[i] = tmp;
        Self::quick_sort(raw, left as i32, (i as i32 - 1));
        Self::quick_sort(raw, (i + 1) as i32, right as i32);
    }

    // pub fn quick_sort_sub(raw: &mut Vec<i32>, left: usize, right: usize) {
    //     if left > right {
    //         return;
    //     }
    // }
}
