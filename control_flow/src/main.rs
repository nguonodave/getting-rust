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
    let num = if high { 9 } else { 7 };
    println!("{}", num);

    // loops
    let mut c = 1;
    let res = loop {
        c += 1;
        println!("wow");
        if c == 10 {
            break c;
        }
    };

    println!("{}", res);

    // while loop
    let mut dig = 3;
    while dig > 0 {
        dig -= 1;
        println!("the dig is {}", dig);
    }

    println!("done");

    let ss = [10, 2, 4, 66, 44, 77, 99];

    for s in ss.iter() {
        println!("{}", s)
    }

    /*
    ... "in ss" implicitly calls the .into_iter() function
    since the ss is a primitive type, which is a copy, hence ownership won't ba transfered, it will not be consumed
    if ss would have been a collection, eg a vector then it would have been consumed by the into_inter fiunction
    */
    for n in ss {
        println!("{}", n)
    }

    println!("{:?}", ss)
}
