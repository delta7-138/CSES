use std::io; 

pub fn parse_n() -> u64{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num: u64 = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

fn main(){
    let mut n = parse_n();

    while n > 1{
        print!("{} " , n); 
        if n % 2 == 0{
            n = n/2;
        }
        else{ 
            n = 3 * n + 1; 
        }
    }

    print!("1"); 
}