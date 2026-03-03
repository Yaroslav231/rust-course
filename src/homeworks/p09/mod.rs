#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];


    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.to_string(), *n),
                exp.to_string()
            )
        );
}


fn rotate(s: String, n: isize) -> String {
    let mut res: String = s;
    
    for i in 0..n.abs() as usize {
        if (n > 0) {
            let y = (&res).chars().last().unwrap();
            res.insert(0, y);
            res.pop();
        } else if (n < 0) {
            let first_char = (&res).chars().next().unwrap();
            let first_len = first_char.len_utf8();
            res.push(first_char);
            res.drain(..first_len);
        }
    }
    return res
}

/* стандартний спосіб
fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let k = n.rem_euclid(len as isize) as usize;

    let split = len - k;
    let (left, right) = s.split_at(split);

    format!("{}{}", right, left)
}
 */