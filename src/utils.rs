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
