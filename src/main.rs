use std::collections::BTreeSet;
use std::env;

fn main() {
    println!("Max input value: {}", usize::max_value());
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let n = args[1].parse::<usize>().unwrap();
            let factors = factorize(n);
            println!("{:?}", factors);
        }
        _ => {
            println!("Program takes exactly one argument.")
        }
    }
}

fn factorize(n: usize) -> Vec<usize> {
    let max_prime = (n as f64).sqrt() as usize;
    let mut primes: BTreeSet<usize> = BTreeSet::from([2]);
    let mut not_primes: BTreeSet<usize> = BTreeSet::new();
    for i in (3..=max_prime).step_by(2) {
        if not_primes.contains(&i) {
            continue;
        } else {
            primes.insert(i);
            for j in (2*i..=max_prime).step_by(i) {
                not_primes.insert(j);
            }
        }
    }

    let mut temp = n;
    let mut result = vec![];
    for i in primes {
        if i > temp{
            break;
        }
        while temp % i == 0 {
            result.push(i);
            temp = temp / i;
        }
    }
    if temp != 1 {
        result.push(temp);
    }
    return result;
}
