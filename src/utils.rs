pub fn tup2<T: Copy>(arr: Vec<T>) -> (T, T) {
    (arr[0], arr[1])
}

pub fn tup3<T: Copy>(arr: Vec<T>) -> (T, T, T) {
    (arr[0], arr[1], arr[2])
}

pub fn tup4<T: Copy>(arr: Vec<T>) -> (T, T, T, T) {
    (arr[0], arr[1], arr[2], arr[3])
}

pub fn hcf(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        hcf(b, a % b)
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / hcf(a, b)
}

pub fn digits(mut num: i64, base: i64) -> Vec<i64> {
    let mut digis = Vec::new();
    while num > 0 {
        let digit = num % base;
        num /= base;
        digis.push(digit);
    }
    digis.reverse();
    digis
}

pub fn exp_mod(num: i64, exp: i64, m: i64) -> i64 {
    let digis = digits(exp, 2);
    let mut rem = 1;
    for &dig in digis.iter() {
        rem *= rem;
        if dig == 1 {
            rem *= num;
        }
        rem %= m;
    }
    rem
}
