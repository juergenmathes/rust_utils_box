fn main(){
    println!("Hello, this a collection of some functions I found useful to write them down.")
}

fn is_palindrom(x: i32) -> bool {
    let mut y = x.to_string();

    if x.to_string() == y.chars().rev().collect::<String>() {
        return true;
    }
    else {
        return false;
    }
}


pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut r = 0;
    match nums.binary_search(&target) {
        Ok(v) => r = v,
        Err(e) => r = e
    };
    
    r as i32
    
    }
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, v) in nums.iter().enumerate() {
        if target <= *v {
            return i as i32
        }
    }        
    nums.len() as i32
}


use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        let mut row = HashSet::new();
        let mut column = HashSet::new();
        let mut cube = HashSet::new();
        for j in 0..9 {
            if board[i][j] != '.' {
                if row.contains(&(board[i][j])) {
                    return false;
                }
                row.insert(board[i][j]);
            }
            
            if board[j][i] != '.' {
                if column.contains(&(board[j][i])) {
                    return false;
                }
                column.insert(board[j][i]);
            }
            
            let row_idx = 3 * (i / 3) + j / 3;
            let col_idx = 3 * (i% 3) + j % 3;
            if board[row_idx][col_idx] != '.' {
                if cube.contains(&(board[row_idx][col_idx])){
                    return false;
                }
                cube.insert(board[row_idx][col_idx]);
            }
        } 
    }
    true
}