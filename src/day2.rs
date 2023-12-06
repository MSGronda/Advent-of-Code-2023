// enum ColorCube {
//     Red,
//     Green,
//     Blue
// }
// impl ColorCube {
//     fn name(&self) -> &str {
//         match *self {
//             ColorCube::Red => "red",
//             ColorCube::Green => "green",
//             ColorCube::Blue => "blue"
//         }
//     }
//     fn parse(name: &str) -> Option<ColorCube> {
//         if name.eq("red") {
//             return Option::Some(ColorCube::Red)
//         }
//         if name.eq("green") {
//             return Option::Some(ColorCube::Green)
//         }
//         if name.eq("blue") {
//             return Option::Some(ColorCube::Blue)
//         }
//         return Option::None
//     }
//     fn parse_idx(name: &str) -> Option<usize>{
//         if name.eq("red") {
//             return Option::Some(0)
//         }
//         if name.eq("green") {
//             return Option::Some(1)
//         }
//         if name.eq("blue") {
//             return Option::Some(2)
//         }
//         return Option::None
//     }
// }
//
// fn get_num(game_text_array : &Vec<char>, start_idx: usize) -> (u32, usize) {
//     let mut j: usize = start_idx;          // Game *
//     let mut game_num: u32 = 0;
//
//     loop {
//         let c = game_text_array.get(j).unwrap();
//         if !c.is_digit(10) {   // We avoid calling all these fucking functions all the time.
//             break;
//         }
//         game_num *= 10;
//         game_num += c.to_digit(10).unwrap();
//         j += 1;
//     }
//     return (game_num, j - start_idx);
// }
//
// fn check_game_possible(config: &[u32; 3], game_text : &str) -> (u32, bool) {
//     let game_text_array: Vec<char>  = game_text.chars().collect();
//
//     let (game_num, len) = get_num(&game_text_array, 5);
//
//     let mut i: usize = 5 + len + 2;
//     let mut combination: [u32;3] = [0, 0, 0];
//     while i < game_text_array.len(){
//
//         let (val, len) = get_num(&game_text_array, i);
//         i += len + 1;
//
//         match game_text_array.get(i).unwrap() {
//             'r' => {
//                 combination[0] = val;
//                 i += "red".len()
//             }
//             'g' => {
//                 combination[1] = val;
//                 i += "green".len()
//             }
//             'b' => {
//                 combination[2] = val;
//                 i += "blue".len()
//             }
//             ';' | '\n' => {
//                 if ! valid_game(config, &combination) {
//                     return (game_num, false);
//                 }
//                 combination = [0, 0, 0]
//             }
//             _ => {}
//         }
//     }
//     return (game_num, true);
// }

const RED_IDX: usize = 0;
const GREEN_IDX: usize = 1;
const BLUE_IDX: usize = 2;

fn valid_game(config: &[u32; 3], combination: &[u32; 3]) -> bool{
    let mut i = 0;
    while i < config.len() {
        if combination.get(i).unwrap() > config.get(i).unwrap() {
            return false;
        }
        i += 1;
    }
    return true;
}




// Config = [Rmax, Gmax, Bmax]
fn parse_game(config: &[u32; 3], game_text : &str) -> (u32, bool){
    let mut tokens: Vec<&str> = game_text.split(' ').collect();
    let game_num = tokens.get(1).unwrap().replace(":","").parse::<u32>().unwrap();
    let mut current_token = 2;

    let mut combination = [0, 0, 0];
    while current_token + 1 < tokens.len() {
        let val = tokens.get(current_token).unwrap().parse::<u32>().unwrap();
        let name = tokens.get(current_token + 1).unwrap();

        if name.starts_with("red") {
            combination[RED_IDX] += val;
        }
        else if name.starts_with("green") {
            combination[GREEN_IDX] += val;
        }
        else if name.starts_with("blue") {
            combination[BLUE_IDX] += val;
        }

        if name.ends_with(";") || name.ends_with("\n"){
            if ! valid_game(config, &combination) {
                return (game_num, false)
            }
            combination = [0, 0, 0]
        }

        current_token += 2;
    }
    return (game_num, true)
}



pub fn day2(){
    let games = [
        String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n"),
        String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n"),
        String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n"),
        String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n"),
        String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n")
    ];
    let config: [u32; 3] = [12, 13, 14];
    let mut total_sum: u32 = 0;

    for game in games {
        let (game_num, valid) = parse_game(&config, game.as_str());

        if valid {
            total_sum += game_num
        }
    }

    println!("Total sum: {}", total_sum)
}