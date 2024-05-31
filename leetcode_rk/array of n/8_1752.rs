// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/description/

// i wrote 20 lines of code after a lot of time and one test case was failing etc etc. so i cleared the code and started from scratch
// cheated a little for answer 
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut count = 0;
        if nums[nums.len()-1] > nums[0] {
            count +=1;
        }
        for index in 0..nums.len()-1 {
             if nums[index] > nums[index+1] {
                count += 1;
            } 
            if count == 2 {
                return false;
            }
        }
        return true;
    }
}

// this only passes 62/109 cases, this fail for things like [6, 10, 6]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut expected_next_index = 0;
        for index in 0..nums.len() {
            expected_next_index = index + 1;
            // println!("{}", )
            if expected_next_index == nums.len() {
                return true;
            } else if nums[index] + 1 == nums[expected_next_index] {
                continue;                
            } else if nums[index] + 1 != nums[expected_next_index] && nums[expected_next_index] == 1 {
                continue;
            }
            else {
                return false;
            }
        }
        return true;
    }
}

// this is the only problem that took more than 30 mins, well more than 1 h