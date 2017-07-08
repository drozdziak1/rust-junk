extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;



const BIG_PRIME: u64 = 15485867;

fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 { return false }
    }
    true
}

fn main() {
    let pool = CpuPool::new_num_cpus();

    let prime_future = pool.spawn_fn(|| {
        let prime = is_prime(BIG_PRIME);

        let res: Result<bool, ()> = Ok(prime);
        res
    });

    println!("Created the future!");

    if prime_future.wait().unwrap() {
        println!("PRIME");
    } else {
        println!("NOT PRIME");
    }
}
