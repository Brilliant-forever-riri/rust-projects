fn main() {
    let number = 2748263483092;
    match factorize(number) {
        Some(factors) => {
            println!("{} is a composite number with factors: \n {:?}", number, factors);
        }
        None => {
            println!("{} is a prime number.", number);
        }
    }
    
}


fn factorize(mut n: u128) -> Option<Vec<u128>> {
    if n <= 1 {
        return None; // 0 and 1 are neither prime nor composite
    }
    let mut p = 2;
    let mut factors: Vec<u128> = Vec::new();
    while n >= p * p {
        if n % p == 0 {
            factors.push(p);
            n /= p;
        } else {
            p += 1;
        }
    }
    factors.push(n);
    factors.sort();

    if factors.len() == 1 {
        return None; // Only one factor, no. Is prime
    } else {
        return Some(factors); // Multiple factors found, number is composite
    }
}
