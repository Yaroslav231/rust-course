const WIDTH: usize = 40;
const HEIGHT: usize = 20;


pub fn draw_array_envelope() {
    let arr = array_envelope();     //отрим масив
    
    //---------------------------------------{
    let mut str: String = "\n".to_string();
    for y in 0.. arr.len() {
        for x in 0.. arr[0].len() {            //перевести в строку додаючи \n в спочатку строки та в кінці кожного рядка
            str.push(arr[y][x])
        }
        str.push('\n');
    }
    //----------------------------------------}

    print!("{}", str)   //виклик println! один раз
}

fn array_envelope() -> [[char; WIDTH]; HEIGHT] {
    
    let width_minus_1 = WIDTH - 1;     
    let height_minus_1 = HEIGHT - 1;

    let center_x = width_minus_1 / 2;
    let center_y = height_minus_1 / 2;
    
    let mut envelope_arr: [[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];   //двовимірний масив для кодів ASII з відповідною кількістю елементів

    for y in 0..HEIGHT {    //цикл по висоті

        //------------------------полоси верх/низ{
        if (y == 0 || y == height_minus_1) {
            for x in 0.. WIDTH {
                envelope_arr[y][x] = '*';
            }
            continue;
        }
        //---------------------------------------}

        
        let dy = if y <= center_y { y } else { height_minus_1 - y }; //выдстань строки до до найближ горизонталі (верх/низ)
        
        let left_pos = (center_x * dy + (center_y / 2)) / center_y;
        let right_pos = width_minus_1.saturating_sub(left_pos);


        for x in 0..WIDTH {
            if (x == 0 || x == width_minus_1 || x == left_pos || x == right_pos) {
                envelope_arr[y][x] = '*';
            }
        }
    }
    return envelope_arr;
}
