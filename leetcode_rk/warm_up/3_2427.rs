// https://leetcode.com/problems/number-of-common-factors/

// prev q took me a lot of time, but this took just 5 mins
impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut sum_factors = 0;
        let max = if a>b {a} else {b};
        for i in 1..=max {
            if a%i == 0 && b%i==0 {
                sum_factors +=1;
            } 
        }
        sum_factors
    }
}