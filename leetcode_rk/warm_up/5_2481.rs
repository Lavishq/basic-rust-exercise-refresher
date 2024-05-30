// https://leetcode.com/problems/minimum-cuts-to-divide-a-circle/solutions/2850494/very-simple-easy-to-understand-solution-2-liner/

// i still dont understand the accurate objective of this problem but i tried to solve for tests and within 10 mins my solution looked like below
// it can be optimised heavily as n%2==0 always has n/2, else has n and n==1 has 0
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        } 

        if n == 2 {
            return 1;
        }
        

        if n == 3 {
            return 3;
        } else if n%3 ==0 && n%2 ==0 {
            return n/2;
        } else if n%2 == 0 {
            return n/2;
        } else {
            return n;
        }
    }
}

// i looked the solutions for better stuff since mine looked like above

// class Solution {
//     public:
//         int numberOfCuts(int n) {
//             if(n == 1) return 0;
//             return n%2 ? n: n/2;
//         }
//     };

// i optimized my soln 
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }    
        if n%2 == 0 {
            return n/2;
        } else {
            return n;
        }
    }
}