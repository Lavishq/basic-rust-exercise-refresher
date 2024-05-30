// https://leetcode.com/problems/count-of-matches-in-tournament/
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 { 
        // n-1 // this works but i think the expectation is in explanation}
        // let mut teams = n;
        // let mut total_nm = 0;
        // while teams > 1 {
        //     total_nm += teams / 2;
        //     teams = (teams + 1) / 2;
        // }
        // total_nm

        // my soln
        let mut teams = n;
        let mut total_nm = 0;
        while teams > 1 {
            if teams%2==0 {
                total_nm += teams/2;
                teams = teams/2;

            } else {
                total_nm += (teams-1) / 2;
                teams = ((teams - 1) / 2)+1;

            }
        }
        total_nm
    }
}