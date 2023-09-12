use std::io; 

pub fn parse_n() -> u64{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num: u64 = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

pub fn parse_array() -> Vec<u64>{
    let mut line = String::new(); 
    io::stdin().read_line(&mut line).expect("invalid parse"); 
    let numbers = line.split(" "); 
    let mut nums: Vec<u64> = Vec::new(); 
    for num_string in numbers{
        nums.push(num_string.trim().parse().expect("array numbers")); 
    }
    return nums; 
}

fn main(){
    let n = parse_n(); 
    let arr = parse_array(); 

    let mut xor_ans = 0; 
    for i in 1..n+1{
        xor_ans = xor_ans ^ i; 
    }

    for i in arr{
        xor_ans = xor_ans ^ i; 
    }

    print!("{}" , xor_ans); 
}

