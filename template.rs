use std::io; 

pub fn parse_n() -> usize{
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

fn main(){
    
}