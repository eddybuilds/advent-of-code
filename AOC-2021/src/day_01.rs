use crate::utils::lines_from_file;

pub fn part_one() -> i32 {
  let lines = lines_from_file("./src/data/day_01.data");

  let mut previous_sweep = 0;
  let mut total_count = -1;

  for line in lines {
    let depth_measurement = line.parse::<i32>().unwrap();

    println!("previous: {} | current: {}", previous_sweep, depth_measurement);

    if depth_measurement > previous_sweep {
      total_count += 1;
    }

    previous_sweep = depth_measurement;
  }

  total_count
}

// pub fn part_two() {}
