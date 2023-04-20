//! recursive defined function 
//! f(n) = 1 if n <= 2
//! f(n) = n * f(n-1) - f(n-2) if n > 2

// recursive function
fn f_rec(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        n * f_rec(n-1) - f_rec(n-2)
    }
}

// iterative function
fn f_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 1;
    for _ in 3..n+1 {
        c = a + b;
        a = b;
        b = c;
    }
    c
}


fn main() {
    // calculate for n = 1,2,3,4,5
        for i in 1..6 {
            println!("f_rec({}) = {}", i, f_rec(i));
            println!("f_iter({}) = {}", i, f_iter(i));
    }
}
