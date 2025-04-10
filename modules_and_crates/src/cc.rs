// cc is already a module and there is no need to do mod or pub mod in the fibo func

// pub mod cc {
pub fn fibonacci(n: u32) -> u32 {
    // n = n-1 + n-2
    if n == 0 {
        return 0;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
// }
