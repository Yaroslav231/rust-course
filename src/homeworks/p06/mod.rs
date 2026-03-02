#[test]
fn test() {
paint_tree(5)
}


fn paint_tree(number_of_triangles: usize) {

    fn paint_triangle(size: usize, shift: usize) {
        let size = if size % 2 == 0 { size - 1 } else { size };

        let height = (size + 1) / 2;

        for row in 0..height {
            let stars = 2 * row + 1;
            let left_padd = shift + (size - stars) / 2;

            for _ in 0..left_padd {
                print!(" ");
            }

            for _ in 0..stars {
                print!("*");
            }

            println!();
        }
    }
    
    let mut size: usize = 3;
    let mut sizes: Vec<usize> = Vec::new();

    for i in 0..number_of_triangles {
        sizes.push(size);

        size = (size as f32 * 1.5).round() as usize;
        
        if size % 2 == 0 { //для центровки
            size += 1;
        }
    }

    let max_size = *sizes.last().unwrap();
    
    for s in sizes {
        let shift: usize = (max_size - s) / 2 ;
        paint_triangle(s, shift)
    }
}

