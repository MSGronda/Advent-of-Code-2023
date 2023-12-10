const START_IDX: usize = 0;
const END_IDX: usize = 1;
const DEST_START_IDX: usize = 0;
const SOURCE_START_IDX: usize = 1;
const LEN_IDX: usize = 2;


// Macros don't work :[
macro_rules! in_range {
    ($map:expr, $input:expr, $idx:expr) => { $input >= $map.$idx[START_IDX] && $input <= $map.$idx[END_IDX] };
}

macro_rules! offset {
    ($map:expr, $input:expr, $idx:expr) => { $input - $map.$idx[START_IDX] };
}

struct Mapper {
    map_range: Vec<([u32;2], [u32;2])>
}

impl Mapper {
    fn new(conversion_map: Vec<[u32;3]>) -> Mapper {
        let mut map_range: Vec<([u32;2], [u32;2])> = Vec::new();
        for map in conversion_map {
            let source = [map[SOURCE_START_IDX], map[SOURCE_START_IDX] + map[LEN_IDX] - 1];
            let dest = [map[DEST_START_IDX], map[DEST_START_IDX] + map[LEN_IDX] - 1];

            map_range.push((source, dest))
        }
        return Mapper {map_range}
    }
    fn map(&self, input: u32) -> u32 {
        for map in &self.map_range {
            if input >= map.0[START_IDX] && input <= map.0[END_IDX] {
                return map.1[START_IDX] + input - map.0[START_IDX]
            }
        }
        return input;
    }
}

pub fn day5(){
    let seeds = [79, 14, 55, 13];

    // Too lazy to parse strings again.
    let seed_soil:Vec<[u32;3]> = vec!([50, 98, 2],[52, 50, 48]);
    let soil_fertilizer:Vec<[u32;3]> = vec!([0, 15, 37],[37, 52, 2],[39, 0, 15]);
    let fertilizer_water:Vec<[u32;3]>= vec!([49, 53, 8],[0, 11, 42],[42, 0, 7],[57, 7, 4]);
    let water_light:Vec<[u32;3]> = vec!([88, 18, 7],[18, 25, 70]);
    let light_temperature:Vec<[u32;3]> = vec!([45, 77, 23],[81, 45, 19],[68, 64, 13]);
    let temperature_humidity:Vec<[u32;3]> = vec!([0, 69, 1],[1, 0, 69]);
    let humidity_location:Vec<[u32;3]> = vec!([60, 56, 37],[56, 93, 4]);

    let maps = vec!(seed_soil, soil_fertilizer, fertilizer_water, water_light, light_temperature, temperature_humidity, humidity_location);
    let mut mappers:Vec<Mapper> = Vec::new();

    for map in maps{
        mappers.push(Mapper::new(map))
    }

    let mut lowest_seed= 0;
    let mut seed_lowest_location = u32::MAX;
    for seed in seeds {
        let mut prev_output = seed;
        for mapper in &mappers {
            prev_output =  mapper.map(prev_output)
        }
        if prev_output < seed_lowest_location {
            seed_lowest_location = prev_output;
            lowest_seed = seed;
        }
    }
    println!("The seed with the lowest plating location is: {}, for the seed: {}", seed_lowest_location, lowest_seed);
}