use std::io;
fn main() {
    let mut x:i32=0;
    let mut input = String::new();
    
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    x = input.trim().parse().expect("Not a valid number");

    for i in 1..=x{
        for j in 1..=x{
            if j == 1 || j == i {
                print!("X ");
            }else if j == x{
                println!("X");
            }else{
                print!("O ");
            }
        }
    }
}
