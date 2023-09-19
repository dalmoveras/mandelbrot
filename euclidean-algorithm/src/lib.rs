pub fn gcd(a: i32, b: i32) -> i32 {
    return euclidean_algorithm(a, b);
}

pub fn euclidean_algorithm(mut a: i32, mut b: i32) -> i32 {
    let mut r: i32;
    if a < b {
        let temp: i32 = a;
        a = b;
        b = temp;
    }
    loop {
        r = a % b;
        println!("Euclidean algorithm: r={}, a={}, b={}", &r, &a, &b);
        if r == 0 {
            return b;
        } else {
            a = b;
            b = r;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = gcd(212, 138);
        assert_eq!(result, 2);
        println!(" -----------------");

        let result2 = gcd(710, 310);
        assert_eq!(result2, 10);
        println!(" -----------------");

        let result3 = gcd(1160718174, 316258250);
        assert_eq!(result3, 1078);
        println!(" -----------------");
    }
}
