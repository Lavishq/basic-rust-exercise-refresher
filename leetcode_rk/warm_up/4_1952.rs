// https://leetcode.com/problems/three-divisors/

// this took just a little more time but the catch here is "exactly"
impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut divisors = 1;
        for i in 2..=n {
            if n%i==0 {
                divisors += 1;
            }
        }
        if divisors == 3 {
            return true;
        }
        false
    }
}