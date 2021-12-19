# rs-factorize
Simple CLI tool for prime factorization.

The purpose of writing this program was 50% to play around with rust for learning purposes, and 50% because I on rare occasions find myself in need for an easily accessible tool for prime factorization.

## Usage

`cargo run --release <integer to factorize>`

```
rs-factorize on  main [!?] is 📦 v0.1.0 via 🦀 v1.57.0 took 17s 
❯ cargo run --release 1844674407370912
   Compiling rs-factorize v0.1.0 (/Users/kriops/rs-factorize)
    Finished release [optimized] target(s) in 0.46s
     Running `target/release/rs-factorize 1844674407370912`
Max input value: 18446744073709551615
[2, 2, 2, 2, 2, 13, 523, 1259, 6734401]
```

## Implementation
The algorithm used is based on the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes), which is my go-to thing to implement when learning a new language. 

- For input n, the max prime candidate is set to max_prime = sqrt(n)
- All primes up to max_primes are generated
- Attempt to divide n by the generated primes

It's not very efficient, but it is more than performant enough for my personal use.
