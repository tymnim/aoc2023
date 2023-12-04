use std::fs;

fn main() {
  let file_path = "input/day3.in";
  let contents = fs::read_to_string(file_path)
  .expect("Should have been able to read the file");
  println!("{}", contents);
  let lines = contents.split("\n").into_iter();

  process(lines);
}

fn find_numbers(tup: (String, String, String)) -> Vec<String>{
  let prevc: Vec<char> = tup.0.chars().collect();
  let linec: Vec<char> = tup.1.chars().collect();
  let nextc: Vec<char> = tup.2.chars().collect();

  let mut numbers: Vec<String> = vec![];
  let mut current = String::from("");
  let mut found_non_dot = false;
  for index in 1..linec.len() {
    if linec[index].is_digit(10) {
      current.push(linec[index]);
      if index - 1 >= 0
          && (prevc[index - 1] != '.' || nextc[index -1] != '.' || (linec[index - 1] != '.' && !linec[index - 1].is_digit(10)))
        || (prevc[index] != '.' || nextc[index] != '.')
      {
        found_non_dot = true;
      }
    }
    else if current.len() > 0 {
      if prevc[index] != '.' || nextc[index] != '.' || linec[index] != '.' || found_non_dot {
        numbers.push(current);
      }
      found_non_dot = false;
      current = String::from("");
    }
  }
  if current.len() > 0 && found_non_dot {
    numbers.push(current);
  }
  return numbers;
}

fn process<'a>(mut data: impl Iterator<Item = &'a str>) {
  let mut prev: String;
  let mut next: String = String::from(".") + data.next().unwrap();
  let mut line: String = ".".repeat(next.len());
  let mut lines: Vec<(String, String, String)> = vec![];
  loop {
    prev = line;
    line = next;
    if let Some(new_line) = data.next() {
      next = String::from(".") + new_line;
      if next.len() > 1 {
        lines.push((prev.clone(), line.clone(), next.clone()));
      }
      else {
        // NOTE: expected to have last character blank line. It has something to do with
        //       how files are read probably
        lines.push((prev.clone(), line.clone(), ".".repeat(prev.len())));
        // This is actually the last line. Can break here
        break;
      }
    }
    else {
      lines.push((prev.clone(), line.clone(), ".".repeat(prev.len())));
      break;
    }
  }
  let mut sum = 0;
  for tup in lines {
    for num in find_numbers(tup) {
      sum += num.parse::<i32>().unwrap_or(0);
    }
  }
  println!("SUM: {sum}");
}

