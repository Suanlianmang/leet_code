struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
    let length=  s.len();
    let (first, last) = s.split_at(length / 2);
    let mut first = first.chars().rev().collect::<Vec<char>>();
    let mut last = last.chars().collect::<Vec<char>>();
    let mut new_val: Vec<char> = vec![];
    let mut append_first: bool = true;
    let mut just_double: bool = true;
    loop {
        let from_first = first.pop();
        let from_last = last.pop();
        if from_first == None && from_last == None {
            break;
        }
        if from_first == None {
            new_val.push(from_last.unwrap());
            just_double = false;
            continue;
        }
        if from_last == None {
            new_val.push(from_first.unwrap());
            just_double = false;
            continue;
        }
        let from_first = from_first.unwrap();
        let from_last = from_last.unwrap();

        if from_first != from_last {
            if append_first {
                new_val.push(from_first);
                last.push(from_last);   
            }
            else {
                new_val.push(from_last);
                first.push(from_first);
            }
        }else{
            new_val.push(from_first);
        }
        append_first = !append_first;

    }
    let mut last_len = new_val.len();
    if !just_double {
        last_len = last_len - 1;
    }
    last_len = last_len * 2;
    println!("{:?} ", new_val);
    println!("{} ", last_len - length);
    let x = (last_len - length) as i32;
    return x;
}
}

fn main() {

    println!("Result: {}", Solution::min_insertions("leetcode".to_string()));
}
 

// zjveiiwvc

// leetcode

//  leedtocotdeel

// zcjvewiiwevjcz