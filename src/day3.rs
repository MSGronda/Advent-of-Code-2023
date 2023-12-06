use std::ffi::c_uint;
use std::path::Component::ParentDir;

const COLS: usize = 10;
const NUMBER_BASE: u32 = 10;
const SYMBOL_ARRAY: [char;4] = ['$', '*', '#', '+'];

macro_rules! idx {
    ($i:expr, $j:expr) => { $i * COLS + $j };
}

fn next_idx_mut(i:&mut usize, j:&mut usize) {
    if *j == COLS - 1{
        *j = 0;
        *i += 1;
    }
    else {
        *j += 1;
    }
}

fn adv_idx_mut(i:&mut usize, j:&mut usize, step: usize) {
    let new_idx = idx!(*i,*j) + step;

    *j = new_idx % 10;
    *i = new_idx / 10;
}

fn up_idx(i:usize) -> Option<usize> {
    if i == 0 {
        return None;
    }
    return Some(i - 1)
}

fn down_idx(i:usize, max_height: usize) -> Option<usize> {
    if i == max_height - 1 {
        return None;
    }
    return Some(i + 1)
}

fn left_idx(j: usize) -> Option<usize> {
    return up_idx(j);
}

fn right_idx(j: usize) -> Option<usize> {
    return down_idx(j, COLS);
}

fn left_diag_up(i:usize, j:usize) -> Option<(usize, usize)> {
    if i == 0 || j == 0 {
        return None;
    }
    return Some((i-1, j-1))
}

fn right_diag_up(i:usize, j:usize, max_width: usize) -> Option<(usize, usize)> {
    if i == 0 || j == max_width {
        return None;
    }
    return Some((i-1, j+1))
}

fn left_diag_down(i:usize, j:usize, max_height: usize) -> Option<(usize, usize)> {
    if i >= max_height - 1 || j == 0 {
        return None;
    }
    return Some((i+1, j-1))
}

fn right_diag_down(i:usize, j:usize, max_height: usize, max_width: usize) -> Option<(usize, usize)> {
    if i >= max_height - 1 || j == max_width {
        return None;
    }
    return Some((i+1, j+1))
}


fn get_number(chars: &Vec<char>, i_start: usize, j_start:usize) -> (u32, usize){
    let mut len: usize = 0;
    let mut i = i_start;
    let mut j = j_start;
    let mut num: u32 = 0;

    loop {
        let maybe_num = chars.get(idx!(i, j)).unwrap();
        if ! maybe_num.is_digit(NUMBER_BASE) {
            break;
        }

        num *= NUMBER_BASE;
        num += maybe_num.to_digit(NUMBER_BASE).unwrap();

        len += 1;
        next_idx_mut(&mut i, &mut j);
    }
    return (num, len);
}
fn is_symbol(c: &char) -> bool{
    for symbol in SYMBOL_ARRAY {
        if symbol.eq(c) {
            return true;
        }
    }
    return false;
}
fn is_part_number(chars: &Vec<char>, i_start: usize, j_start: usize, len: usize) -> bool{
    let mut i = i_start;
    let mut j = j_start;
    let max_height = chars.len() / COLS;

    let left = left_idx(j);
    if left.is_some() && is_symbol(chars.get(idx!(i, left.unwrap())).unwrap()) {
        return true;
    }
    let ldu = left_diag_up(i,j);
    if ldu.is_some() && is_symbol(chars.get(idx!(ldu.unwrap().0, ldu.unwrap().1)).unwrap()) {
        return true;
    }
    let ldd = left_diag_down(i,j, max_height);
    if ldd.is_some() && is_symbol(chars.get(idx!(ldd.unwrap().0, ldd.unwrap().1)).unwrap()) {
        return true;
    }

    for l in 0..len {
        let up = up_idx(i);
        if up.is_some() && is_symbol(chars.get(idx!(up.unwrap(),  j)).unwrap()) {
            return true;
        }

        let down = down_idx(i, max_height);
        if down.is_some() && is_symbol(chars.get(idx!(down.unwrap(),  j)).unwrap()) {
            return true;
        }

        if l < len - 1 {
            next_idx_mut(&mut i, &mut j)
        }
    }

    let right = right_idx(j);
    if right.is_some() && is_symbol(chars.get(idx!(i, right.unwrap())).unwrap()) {
        return true;
    }

    let rdu = right_diag_up(i,j, COLS);
    if rdu.is_some() && is_symbol(chars.get(idx!(rdu.unwrap().0, rdu.unwrap().1)).unwrap()) {
        return true;
    }
    let rdd = right_diag_down(i,j, max_height, COLS);
    if rdd.is_some() && is_symbol(chars.get(idx!(rdd.unwrap().0, rdd.unwrap().1)).unwrap()) {
        return true;
    }

    return false;
}

fn parse_schematic(schematic: &str) -> u32{
    let chars: Vec<char> = schematic.chars().collect();

    let mut i = 0;
    let mut j = 0;

    let mut total_sum: u32 = 0;

    while idx!(i,j) < chars.len() {
        if chars.get(idx!(i, j)).unwrap().is_digit(NUMBER_BASE) {
            let (num, len) = get_number(&chars, i, j);

            if is_part_number(&chars, i, j, len) {
                total_sum += num;
            }

            adv_idx_mut(&mut i, &mut j, len);
        }
        else {
            next_idx_mut(&mut i, &mut j)
        }
    }

    return total_sum;
}

pub fn day3(){
    let schematic = String::from("467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..");

    let total_sum = parse_schematic(schematic.as_str());

    println!("Total sum: {}", total_sum)
}