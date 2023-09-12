use std::io; 

pub fn parse_n() -> usize{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num = num_string.trim().parse().expect("invalid number"); 
    return num; 
}

fn main(){
    let n = parse_n();
    if n == 1{
        print!("1"); 
    } 
    else if n <= 3{
        print!("NO SOLUTION"); 
    }
    else if n == 4{
        print!("2 4 1 3"); 
    }
    else{
        let mut perm = vec![0; n]; 
        let mut ctr = 1; 
        let mut even_ptr = 0;
        let mut odd_ptr = 1;  

        while even_ptr < n{
            perm[even_ptr] = ctr; 
            even_ptr+=2; 
            ctr+=1; 
        }

        while odd_ptr < n{
            perm[odd_ptr] = ctr; 
            odd_ptr += 2; 
            ctr+=1;
        }
        
        for i in perm{
            print!("{} " , i); 
        }
    }
}