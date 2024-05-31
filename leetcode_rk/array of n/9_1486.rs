// https://leetcode.com/problems/xor-operation-in-an-array/

// quick ez
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut xor_ed = 0;
        for i in 0..n { 
            xor_ed ^= start+2*i;
        }
        xor_ed
    }
}

// tested this too
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut xor_ed = start;
        for i in 1..n { 
            xor_ed ^= start+2*i;
        }
        xor_ed
    }
}

// it occured to me to use a vec![] store and then reloop but xor would work the same way in any case when conv. to binary and i suspected my ans will be correct so i tried and passed