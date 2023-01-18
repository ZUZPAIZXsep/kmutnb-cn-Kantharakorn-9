use std::io;
fn main() {
    let mut x:i32=0;
    let mut input = String::new();
    
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    x = input.trim().parse().expect("Not a valid number");
    
    for i in 1..=x{
        for j in x-(i-2)..=x{
            print!("  ");
        }
        let y = ((x-(i-1))*2)-1;
        for k in 1..=y{
            if k == y {
                println!("*");
            }else{
                print!("* ");
            }
        }
    }
}