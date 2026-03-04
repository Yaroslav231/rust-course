use rand::Rng;


#[test]
fn test() {
    print!("indexes:");

    for i in 0..20 {
        print!("{:>3}.", i);
    }

    let vec = gen_random_vector(20);

    println!();
    print!("data:  ");
    for i in 0.. 20 {
        print!("  {}", vec[i])
    }

    let min_sum_and_pair = min_adjacent_sum(&*vec);

    print!("\nmin adjacent sum={}+{}={}", min_sum_and_pair.0.0, min_sum_and_pair.0.1, min_sum_and_pair.1)
}

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);

    for i in 0..n {
        let value = rng.gen_range(10..99);
        vec.push(value);
    }
    return vec
}

fn min_adjacent_sum(data: &[i32]) -> ((i32, i32), i32) {
    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);

    for i in 1..(data.len() - 1) {
        let sum = data[i] + data[i + 1];

        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
        }
    }
    return (min_pair, min_sum)
}