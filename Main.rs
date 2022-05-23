use std :: env :: {args , Args};
fn main() {
    let mut args : Args = args();
    
    if args.len() < 3{
        panic!("Not the good number of arguments");
    }
    let first = args.nth(1).unwrap();
    let second = args.nth(0).unwrap();
    let third = args.nth(0).unwrap();
    println!("{} {} {}", first , second , third);
}