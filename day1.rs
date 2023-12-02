use std::fs;

fn main() {
  let file_path = "input/day1.in";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let lines = contents.split("\n");
  let sum: i32 = lines.into_iter()
  .map(parse)
  .sum();

  println!("SUM: {}", sum);
}

fn parse(line: &str) -> i32 {
  let codes: Vec<(&str, &str)> = vec![
    ("one", "1"), ("two", "2"),
    ("three", "3"), ("four", "4"),
    ("five", "5"), ("six", "6"),
    ("seven", "7"), ("eight", "8"),
    ("nine", "9")
  ];

  let mut proper_line = String::from(line);
  for code in codes {
    while proper_line.contains(code.0) {
      // I am not very proud of it
      proper_line.insert_str(proper_line.find(code.0).unwrap() + 2, code.1);
    }
  }

  let mut numbers: Vec<u32> = Vec::new();
  for letter in proper_line.chars() {
    if letter.is_digit(10) {
      numbers.push(letter.to_digit(10).unwrap());
    }
  }
  // println!("{}{} <- {proper_line} <- {line}", numbers.first().unwrap_or(&0), numbers.last().unwrap_or(&0));
  return format!("{}{}", numbers.first().unwrap_or(&0), numbers.last().unwrap_or(&0)).parse::<i32>().unwrap();
}

