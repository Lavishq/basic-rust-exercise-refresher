// https://leetcode.com/problems/running-sum-of-1d-array

// i solved it quickly in 2-3 tries by comparing op and expected output
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = vec![];
        let mut prev = nums[0];
        for i in 1..nums.len() {
            sum.push(prev);
            prev += nums[i];
        }
        sum.push(prev);
        sum
    }
}

// i was trying to solve in this way like pushing once but gave more priority to working soln
// this was in comments
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    let mut retValue:Vec<i32> = Vec::new();
    for num in nums {
        sum += num;
        retValue.push(sum);
    }
    retValue
}

// another interesting soln i found was this
// Create a mutable copy of the input vector
{
    let mut nums = nums;
            
    for i in 1..nums.len() {
        nums[i] = nums[i - 1] + nums[i];
    }
    
    nums
}

