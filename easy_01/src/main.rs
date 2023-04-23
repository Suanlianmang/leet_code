use std::collections::VecDeque;

struct Solution;



impl Solution {
    
    pub fn is_palindrome(x: i32) -> bool {
        let rev = x.to_string().chars().rev().collect::<String>();
        rev == x.to_string()
    }



    fn map_roman(c: char) -> i32 {
     match c {
        'I' => return 1,
        'V' => return 5,
        'X' => return 10,
        'L' => return 50,
        'C' => return 100,
        'D' => return 500,
        'M' => return 1000,
        _ => return 0
     }
    }
    pub fn roman_to_int(s: &str) -> i32 {
        let num: Vec<i32> = s.chars().map(|x| Self::map_roman(x)).collect::<Vec<i32>>();


        let num_to_sub: Vec<i32> = num.iter().enumerate().map(|(i, x)| {if i < (&num.len() - 1) && num[i + 1] > *x{*x}else{0}}).collect::<Vec<i32>>();

        num.into_iter().sum::<i32>() - (num_to_sub.into_iter().sum::<i32>() * 2)
    }


    fn matching_parentheses(open: char, close: char) -> bool {
        match open{
            '(' => return close == ')',
            '{' => return close == '}',
            '[' => return close == ']',
            _ => return false
        }

    }

    pub fn is_valid(s: &str) -> bool {
        let mut queue: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            let front = queue.pop_back();
            if front == None {
                queue.push_back(c);
                continue;
            }
            let front = front.unwrap();
            if Self::matching_parentheses(front, c) {
                continue;
            }
            queue.push_back(front);
            queue.push_back(c);
            
        }
        queue.len() == 0
        
    }

}
fn main() {     
     let x = 123454321;

     println!("{} is a palindrome: {}", x, Solution::is_palindrome(x));
     
     let x = String::from("MCMXCIV");

     println!("{} in int is {}", x, Solution::roman_to_int(&x));

     
    let x = String::from("([)]");
    println!("{} is valid: {:?}", x, Solution::is_valid(&x));
}
