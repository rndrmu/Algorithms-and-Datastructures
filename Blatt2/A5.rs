//! recursive defined function f
//! f(n) = 1 if n <= 2
//! f(n) = n + f(n-2) if n > 2

fn f_rec(n: i32) -> i32 {
    if n <= 2 {
        1
    } else {
        n + f_rec(n - 2)
    }
}

fn f_iter(n: i32) -> i32 {
    let mut n1 = 1;
    let mut n2 = 1;
    let mut i = 2;
    while i < n {
        let tmp = n1 + n2;
        n1 = n2;
        n2 = tmp;
        i += 1;
    }
    n2
}

// calc for n = 1, 2, 3, 4, 5
fn main() {
    for i in 1..5 {
        println!("f_rec({}) = {}", i, f_rec(i));
        println!("f_iter({}) = {}", i, f_iter(i));
    }
}
