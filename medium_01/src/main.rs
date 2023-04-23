struct Solution;


impl Solution {
    pub fn reverse(x: i32) -> i32 {

        match x.abs().to_string().chars().rev().collect::<String>().parse::<i32>(){
            Ok(i) => return i * x.signum(),
            Err(_) => return 0
        };
    }
}


fn main() {
    let x: i32 = -120;
    println!("{}", Solution::reverse(x));
}
