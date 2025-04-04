fn main() {
    if 2 > 4 {
        println!("yay");
    } else if 4 < 2 {
        println!("boom");
    } else {
        println!("nice");
    }

    // using if else in a variable
    let high = true;
    let num = if high {9} else {7};
    println!("{}", num)
}
