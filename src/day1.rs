
fn decode_line(line: &str) -> u32{
    let mut i = 0;
    let mut j = line.len() - 1;
    let mut left_processed = false;
    let mut right_processed = false;

    let mut left_value = 0;
    let mut right_value= 0;

    let chars: Vec<char> = line.chars().collect();


    while i <= line.len() - 1 && (!left_processed || !right_processed) {
        let l = chars.get(i).unwrap();
        if !left_processed && l.is_numeric() {
            left_value = l.to_digit(10).unwrap();
            left_processed = true;
        }
        let r = chars.get(j).unwrap();
        if !right_processed && r.is_numeric() {
            right_value = r.to_digit(10).unwrap();
            right_processed = true;
        }
        i+=1;
        j-=1;
    }

    return left_value * 10 + right_value;
}

fn decode_line2(line: &str) -> u32{
    let mut left_processed = false;
    let mut right_processed = false;
    let mut left_value = 0;
    let mut right_value= 0;

    let mut forward =  line.chars();
    let mut backward =  line.chars().rev();

    while !left_processed || !right_processed{
        let l = forward.next();

        // Reached end of string
        if ! l.is_some(){
            break;
        }

        if !left_processed && l.unwrap().is_numeric() {
            left_value = l.unwrap().to_digit(10).unwrap();
            left_processed = true;
        }
        let r = backward.next().unwrap();
        if !right_processed && r.is_numeric() {
            right_value = r.to_digit(10).unwrap();
            right_processed = true;
        }
    }

    return left_value * 10 + right_value;
}

pub fn day1(){
    let lines = [
        String::from("1abc2"),
        String::from("pqr3stu8vwx"),
        String::from("a1b2c3d4e5f"),
        String::from("treb7uchet"),
    ];

    let mut sum:u32 = 0;
    for line in lines {
        sum += decode_line2(line.as_str())
    }

    println!("Total sum: {}", sum)
}