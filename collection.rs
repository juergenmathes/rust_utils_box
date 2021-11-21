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