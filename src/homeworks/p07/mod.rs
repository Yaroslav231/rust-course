#[test]
fn test() {
    let data =
        [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];


    data
        .iter()
        .for_each(|(a, b)| {
            assert_eq!(
                invert_the_case(a.to_string()),
                b.to_string()
            );
            assert_eq!(
                invert_the_case(b.to_string()),
                a.to_string()
            );
        });
}


fn invert_the_case(s: String) -> String {
    let mut res: String = "".to_string();
    for c in (&s).chars() {
        if c.is_uppercase() {
            res.push(c.to_lowercase().next().unwrap());
        } else if c.is_lowercase() {
            res.push(c.to_uppercase().next().unwrap());
        } else {
            res.push(c);
        }
    }
    return res;
}
