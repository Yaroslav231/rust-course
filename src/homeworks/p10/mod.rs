#[test]
fn test() {
    let data =
        [
            (123, false),
            (121, true),
            (1221, true),
        ];


    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}


fn is_palindrome(x: u32) -> bool {
    let cop = x.to_string();
    let t = (&cop).chars().rev();   //зворотня послідовність символів

    return (&cop).chars().eq(t)         //зрівняти послідовності
}