use std::io; 

pub fn parse_n() -> usize{
    let mut num_string = String::new(); 
    io::stdin().read_line(&mut num_string).expect("invalid parse");
    let num = num_string.trim().parse().expect("invalid number"); 
    return num; 
}


pub fn parse_string() -> Vec<char>{
    let mut inp_string = String::new(); 
    io::stdin().read_line(&mut inp_string).expect("invalid parse");
    let strinp_arr = inp_string.trim().to_string();
    return strinp_arr.chars().collect();
}

fn solve(grid: Vec<Vec<char>> , n: usize) -> u64{
    let modular: u64 = 1000000007; 
    let mut dp = vec![vec![0; n]; n]; 
    if grid[0][0] == '*'{
        return 0; 
    }

    dp[0][0] = 1; 

    for i in 0..n{
        for j in 0..n{
            //print!("{}" , grid[i][j]); 
            if i == 0 && j == 0 || grid[i][j] == '*'{
                continue; 
            }

            if i == 0{
                if grid[i][j-1] == '*'
                {
                    dp[i][j] = 0; 
                }
                else
                {
                    dp[i][j] = dp[i][j-1] % modular;
                }
            }
            else if j == 0{
                if grid[i-1][j] == '*'
                {
                    dp[i][j] = 0; 
                }
                else
                {
                    dp[i][j] = dp[i-1][j] % modular; 
                }
            }
            else{
                dp[i][j] = (dp[i][j-1] + dp[i-1][j]) % modular; 
            }
        }
    }
    return dp[n-1][n-1]; 
}

fn main(){
    let grid_size = parse_n(); 
    let mut final_grid = Vec::new(); 

    for _i in 0..grid_size{
        final_grid.push(parse_string()); 
    }
    
    print!("{}" , solve(final_grid , grid_size)); 
}