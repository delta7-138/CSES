use std::io; 

pub fn gcd<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
    // for our required types, default evalutaes to 0 at compile time.
    // and thus have 0 cost abstraction.
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn parse_n() -> i32{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

pub fn parse_string() -> String{
    let mut inp_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    return inp_string.trim().to_string(); 
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
    let mut t = parse_n(); 
    while t > 0{
        //enter code here
        t-=1; 
    }
}