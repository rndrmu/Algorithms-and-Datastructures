use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Instant;


/// TIME COMPLEXITY: O(n)
/// SPACE COMPLEXITY: O(1)
fn fibonacci_iterative(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 1..n {
        let c = a.checked_add(b);

        match c {
            Some(sum) => {
                a = b;
                b = sum;
            }
            None => {
                // return previous value 
                return b;
            }
        }
    }

    b
}

/// TIME COMPLEXITY: O(2^n)
/// SPACE COMPLEXITY: O(n)
fn fibonacci_recursive(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

fn calculate_time_for_n_numbers(fibonacci_fn: T, max_time_in_secs: u64) -> u128 
    where T: Fn(u128) -> u128 + Send + Copy + 'static
{
    let (tx, rx) = channel();

    let start_time = Instant::now();

    let handle = thread::spawn(move || {
        let mut n = 0;

        loop {
            let result = fibonacci_fn(n);

            if result == 0 && n > 0 {
                break;
            }

            if start_time.elapsed().as_secs() > max_time_in_secs {
                break;
            }

            tx.send(result).unwrap();
            n += 1;
        }
    });

    let mut numbers_calculated: u128 = 0;

    for _ in rx {
        numbers_calculated += 1;

        if start_time.elapsed().as_secs() > max_time_in_secs {
            break;
        }
    }

    handle.join().unwrap();

    numbers_calculated
}

fn main() {
    
    // own task for each function (iterative and recursive) and for each interval (1, 10, 60 seconds)

    println!("Iterative: {} numbers calculated in 1 second", calculate_time_for_n_numbers(fibonacci_iterative, 1));
    println!("Recursive: {} numbers calculated in 1 second", calculate_time_for_n_numbers(fibonacci_recursive, 1));

    println!("Iterative: {} numbers calculated in 10 seconds", calculate_time_for_n_numbers(fibonacci_iterative, 10));
    println!("Recursive: {} numbers calculated in 10 seconds", calculate_time_for_n_numbers(fibonacci_recursive, 10));

    println!("Iterative: {} numbers calculated in 60 seconds", calculate_time_for_n_numbers(fibonacci_iterative, 60));
    println!("Recursive: {} numbers calculated in 60 seconds", calculate_time_for_n_numbers(fibonacci_recursive, 60));


    /**
     * Result on a 10400F (6 cores) with 16GB RAM
     * Iterative: 559351 numbers calculated in 1 second
     * Recursive: 40 numbers calculated in 1 second
     * Iterative: 3032809 numbers calculated in 10 seconds
     * Recursive: 43 numbers calculated in 10 seconds
     * Iterative: 16366178 numbers calculated in 60 seconds
     * Recursive: 47 numbers calculated in 60 seconds
     */

}
