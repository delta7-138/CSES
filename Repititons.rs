use std::io; 

pub fn parse_string() -> String{
    let mut str_inp = String::new(); 
    io::stdin().read_line(&mut str_inp).expect("String not correct");
    return str_inp.trim().to_string();  
}

fn main(){
    let seq_str: String = parse_string();
    let seq: Vec<char> = seq_str.chars().collect(); 
    let mut max_win = 1; 
    let mut left = 0; 
    let mut right = 1; 
    let n = seq.len(); 

    while right < n{
        while right < n && seq[right] == seq[right-1]{
            right+=1; 
        }

        if right - left > max_win{
            max_win = right - left; 
        }

        left = right; 
        right = right+1;
    }

    print!("{}" , max_win); 
}