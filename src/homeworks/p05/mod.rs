use std::cmp::min;

#[test]
fn test() {
    let data =
        [
            ((24,  60), 12),
            ((15,   9),  3),
            ((15,   6),  3),
            ((140, 40), 20),
            ((24,  16),  8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37,  11),  1),
            ((120, 90), 30),
        ];


    for ((a, b), exp) in data.iter() {
        assert_eq!(*exp, gcd(*a, *b));
    }
}


fn gcd(mut a: u32, mut b: u32) -> u32 {                     //алгоритм Евклида
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}


/*
fn gcd(a: u32, b: u32) -> u32 {           //повільна версія
    let mut res: u32 = 0;
    for i in 1..=min(a, b) {
        if ((a % i) == 0  && (b % i) == 0) {
            res = i;
        }
    }
    return res
}
*/
