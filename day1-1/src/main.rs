use std::{env, iter};

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    let depth_readings_contents = std::fs::read_to_string(&cmd_args[1])
        .expect("Failed to open the readings file");

    let readings_extracted = depth_readings_contents
        .split('\n')
        .filter(|reading| !reading.is_empty())
        .map(|reading| reading.parse::<i32>().unwrap());
    
    let depth_increases = calculate_depth_increases(readings_extracted);

    println!("Number of depth increases: {}", depth_increases);
}


fn calculate_depth_increases<'a, I>(values: I) -> usize
where
    I: IntoIterator<Item = i32> + Clone
{
    let previous = values.clone().into_iter().chain(iter::repeat(0));
    let mut current = values.into_iter();
    current.next();

    let depth_increases = previous.zip(current).filter(|(prev, current)| prev < current).count();

    return depth_increases;
}

#[test]
fn test_simple() {
    let test_data = vec![0, 1, 0, 2];

    assert_eq!(calculate_depth_increases(test_data), 2);
}