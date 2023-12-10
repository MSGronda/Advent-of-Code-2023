
// Distance = time_held * (total_time - time_held)
macro_rules! distance {
    ($n:expr, $t:expr) => {$n * ($t - $n)};
}

fn get_winning_times(total_time: u32, record_distance: u32) -> Vec<u32> {
    let mut resp = Vec::new();

    let f_total_time = total_time as f64;
    let f_record_distance = record_distance as f64;

    let inner = f_total_time * f_total_time - 4f64 * f_record_distance;

    let interval_start = ((f_total_time - inner.sqrt()) / 2f64).ceil() as u32;  // Awful notation
    let interval_end = ((f_total_time + inner.sqrt()) / 2f64).floor() as u32;

    let n_interval_start = if distance!(interval_start, total_time) <= record_distance {interval_start + 1} else {interval_start};
    let n_interval_end = if distance!(interval_end, total_time) <= record_distance {interval_end - 1} else {interval_end};

    for n in n_interval_start..(n_interval_end+1) {
        resp.push(n)
    }

    return resp;
}


pub fn day6(){
    let race_times: [u32; 3] = [7, 15, 30];
    let race_records: [u32; 3] = [9, 40, 200];

    let mut answer:u32 = 1;
    for i in 0..race_records.len() {
        let resp = get_winning_times(race_times[i], race_records[i]);
        answer *= resp.len() as u32;
    }
    println!("Answer: {}", answer)
}