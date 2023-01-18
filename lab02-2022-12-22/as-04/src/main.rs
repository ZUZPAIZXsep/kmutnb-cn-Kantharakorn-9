use std::io;
fn main() {
    let mut x:i32=0;
    let mut input = String::new();
    
    println!("Enter number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    x = input.trim().parse().expect("Not a valid number");

    print!("{} = ",x);
    for i in 0..=100{
        if x % 2 == 0{
            print! ("2");
            x = x/2;
            if x != 1 {
                print! (" * ");
            }else{
                break;
            }
        }else if x % 3 == 0{
            print! ("3");
            x = x/3;
            if x != 1 {
                print! (" * ");
            }else{
                break;
            }
        }else if x % 5 == 0{
            print! ("5");
            x = x/5;
            if x != 1 {
                print! (" * ");
            }else{
                break;
            }
        }else if x % 7 == 0{
            print! ("7");
            x = x/7;
            if x != 1 {
                print! (" * ");
            }else{
                break;
            }
        }else{
            print! ("{}",x);
            break;
        }
    }
    
}
