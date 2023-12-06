
const COLS: usize = 10;

macro_rules! idx {
    ($i:expr, $j:expr) => { $i * COLS + $j };
}

fn next_idx(i:&mut usize, j:&mut usize) {
    if *j == 10{
        *j = 0;
        *i += 1;
    }
    else {
        *j += 1;
    }
}

fn parse_schematic(schematic: &str){
    let chars: Vec<char> = schematic.chars().collect();

    let mut i = 0;
    let mut j = 0;

    while idx!(i,j) < chars.len() {
        if chars.get(idx!(i, j)).unwrap().is_digit(10) {
            let mut len = 1;
            next_idx(&mut i,&mut j);
            while chars.get(idx!(i, j)).unwrap().is_digit(10) {
                len += 1;
                next_idx(&mut i,&mut j);
            }


        }
        else {
            next_idx(&mut i, &mut j)
        }
    }

}

pub fn day3(){
    let schematic = String::from("467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..");


    parse_schematic(schematic.as_str())
}