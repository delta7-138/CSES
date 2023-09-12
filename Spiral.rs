use std::io; 

pub fn parse_n() -> i32{
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

fn solve(x: usize , y: usize) -> i64{
    let mut max_num: i64 = x as i64;
    let mut min_num: i64 = y as i64; 
    let mut row_flag = 0; 

    if y > x{
        max_num = y as i64; 
        min_num = x as i64; 
        row_flag = 1; 
    }

    if max_num % 2 == row_flag {
        return max_num * max_num - min_num + 1; 
    }
    return max_num * max_num - (2 * max_num) + min_num + 1; 
}



fn main(){
    let mut t = parse_n(); 
    while t > 0{
        let row_col_pair = parse_array(); 
        let x = row_col_pair[0]; 
        let y = row_col_pair[1];

        println!("{}" , solve(x , y)); 
        t-=1; 
    }
}