use std::io; 

pub fn parse_n() -> i64{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

pub fn parse_array() -> Vec<usize>{
    let mut line = String::new(); 
    io::stdin().read_line(&mut line).expect("invalid parse"); 
    let numbers = line.split(" "); 
    let mut nums = Vec::new(); 
    for num_string in numbers{
        nums.push(num_string.trim().parse().expect("array numbers")); 
    }
    return nums; 
}
fn solve(n: usize , k: usize) -> usize{
    
}

fn main(){
    let mut t = parse_n(); 
    
    while t >= 0{
        let args = parse_array(); 
        t-=1;
    }
}