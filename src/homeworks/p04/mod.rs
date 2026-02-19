const WIDTH: usize = 11;
const HEIGHT: usize = 11;


pub fn draw_array_rhombus() {
    let arr = array_rhombus();     //отрим масив

    //---------------------------------------{
    let mut str: String = "\n".to_string();
    for y in 0.. arr.len() {
        for x in 0.. arr[0].len() {            //перевести в строку додаючи \n в спочатку строки та в кінці кожного рядка
            str.push(arr[y][x])
        }
        str.push('\n');
    }
    //----------------------------------------}

    print!("{}", str)   //виклик print! один раз
}

fn array_rhombus() -> [[char; WIDTH]; HEIGHT] {

    let cx = (WIDTH as isize - 1) / 2;      //центр
    let cy = (HEIGHT as isize - 1) / 2;

    let mut rhombus_arr: [[char; WIDTH]; HEIGHT] = [[' '; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = (x as isize - cx).abs();     //відстані від центра
            let dy = (y as isize - cy).abs();

            if (dx * cy + dy * cx <= cx * cy) {
                rhombus_arr[y][x] = '*'
            }
        }
    }
    return rhombus_arr;
}
