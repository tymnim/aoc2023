use std::fs;

fn main() {
  let file_path = "input/day4.in";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let lines = contents.split("\n");
  let sum: u32 = lines.into_iter()
  .map(parse_part_1)
  .sum();

  println!("SUM: {}", sum);
}

fn parse_part_1(line: &str) -> u32 {
  if let Some((_, numbers)) = line.split_once(": ") {
    if let Some((winning, mine)) = numbers.split_once(" | ") {
      let winning_numbers: Vec<&str> = winning.split(" ").filter(| number | number.len() > 0).collect();
      let my_numbers = mine.split(" ").filter(| number | number.len() > 0).into_iter();

      let my_winning: Vec<i32> = my_numbers
        .filter(|number| winning_numbers.contains(number))
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
      let count = my_winning.len();
      if count > 0 {
        let pp = u32::pow(2, (count - 1) as u32);
        return pp;
      }
      return 0;
    }
  }
  return 0;
}


