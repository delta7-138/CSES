use std::io; 

pub fn parse_n() -> usize{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

pub fn parse_array() -> Vec<u64>{
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
    let n = parse_n(); 
    let mut arr = parse_array(); 

    let mut ans:u64 = 0; 
    for i in 1..n{
        if arr[i] < arr[i-1]{
            ans = ans + (arr[i-1] - arr[i]); 
            arr[i] = arr[i-1]; 
        }
    }

    print!("{}", ans); 
}