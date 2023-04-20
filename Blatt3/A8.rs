fn main() {
    let n = 5;
    println!("n = {}", n);

    let h_iter = harmonic_iter(n);
    println!("Iteratively calculated harmonic series: {}", h_iter);

    let h_rec = harmonic_rec(n);
    println!("Recursively calculated harmonic series: {}", h_rec);
}

fn harmonic_iter(n: i32) -> f64 {
    let mut h = 0.0;
    for i in 1..=n {
        h += 1.0 / i as f64;
    }
    h
}

fn harmonic_rec(n: i32) -> f64 {
    if n == 1 {
        1.0
    } else {
        harmonic_rec(n - 1) + 1.0 / n as f64
    }
}
