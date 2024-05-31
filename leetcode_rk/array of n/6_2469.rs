// https://leetcode.com/problems/convert-the-temperature/

// took me a minute to solve 
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let mut kelvin = celsius + 273.15;
        let mut fahrenheit = celsius * 1.80 +32.00;
        return vec![kelvin, fahrenheit];
    }
}

// could be returned in one line as
// return vec![celsius + 273.15, celsius * 1.80 +32.00];