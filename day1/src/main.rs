extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code Day 1")
                    .arg(Arg::with_name("sliding-window")
                        .help("Use sliding window as required by the second challenge.")
                        .long("sliding-window"))
                    .arg(Arg::with_name("INPUT")
                        .help("Sonar readings file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let depth_readings_contents = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the readings file");

    let readings_extracted = depth_readings_contents
        .split('\n')
        .filter(|reading| !reading.is_empty())
        .map(|reading| reading.parse::<i32>().unwrap());
    
    let sums: Vec<i32> = if matches.is_present("sliding-window") {
        calculate_triplet_sums(readings_extracted).collect()
    } else {
        readings_extracted.collect()
    };
    
    let depth_increases = calculate_depth_increases(sums);

    println!("Number of depth increases: {}", depth_increases);
}


fn calculate_depth_increases<'a, I>(values: I) -> usize
where
    I: IntoIterator<Item = i32> + Clone
{
    let previous = values.clone().into_iter();
    let current = values.into_iter().skip(1);

    let depth_increases = previous.zip(current).filter(|(prev, current)| prev < current).count();

    return depth_increases;
}

fn calculate_triplet_sums<'a, I>(values: I) -> impl Iterator<Item = i32>
where
    I: IntoIterator<Item = i32> + Clone 
{
    let first = values.clone().into_iter();
    let second = values.clone().into_iter().skip(1);
    let third = values.clone().into_iter().skip(2);

    let res = first.zip(second).zip(third).map(|((a, b), c)| {
        a + b + c
    });

    return res;
}

#[test]
fn test_simple() {
    let test_data = vec![0, 1, 0, 2];

    assert_eq!(calculate_depth_increases(test_data), 2);
}

#[test]
fn test_triplet_sums_simple() {
    let test_data = vec![199, 200, 208];

    let computed: Vec<i32> = calculate_triplet_sums(test_data).collect();

    assert_eq!(computed, vec![607]);
}

#[test]
fn test_triplet_sums_complex() {
    let test_data = vec![199, 200, 208, 210, 200];

    let computed: Vec<i32> = calculate_triplet_sums(test_data).collect();

    assert_eq!(computed, vec![607, 618, 618]);
}