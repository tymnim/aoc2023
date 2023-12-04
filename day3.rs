use std::fs;

fn main() {
  let file_path = "input/day3.in";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
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
      if (prevc[index - 1] != '.' || nextc[index -1] != '.' || (linec[index - 1] != '.' && !linec[index - 1].is_digit(10)))
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

fn find_numbers_p2(tup: (String, String, String)) -> i64 {
  let prevc: Vec<char> = tup.0.chars().collect();
  let linec: Vec<char> = tup.1.chars().collect();
  let nextc: Vec<char> = tup.2.chars().collect();

  let mut numbers: Vec<i64> = vec![];
  let limit = linec.len();
  let directions: Vec<Vec<(&Vec<char>, i32, i32)>> = vec![
    vec![(&prevc, -1, -1), (&prevc, 0, 0), (&prevc, 1, 1)],
    vec![(&linec, -1, -1), (&linec, 0, 0), (&linec, 1, 1)],
    vec![(&nextc, -1, -1), (&nextc, 0, 0), (&nextc, 1, 1)]
  ];
  for index in 1..limit {
    if linec[index] == '*' {
      let mut number_pair: Vec<String> = vec![];
      for direction in directions.clone() {
        let mut number = String::from("");
        for (line, mut position, inc) in direction {
          if position < limit as i32 {
            let mut num = String::from("");
            let mut current = index as i32 + position;
            if !current.is_negative() {
              while !current.is_negative()
                && current < limit as i32
                && line[current as usize].is_digit(10)
              {
                num.push(line[current as usize]);
                position += inc;
                current = index as i32 + position;
                if inc == 0 {
                  break;
                }
              }
              if inc == 0 && !line[current as usize].is_digit(10) && number.len() > 0 {
                number_pair.push(number);
                number = String::from("");
              }
              if inc < 0 {
                number.push_str(&num.chars().rev().collect::<String>());
              }
              else {
                number.push_str(&num);
              }
            }
          }
        }
        if number.len() > 0 {
          number_pair.push(number);
        }
      }

      if number_pair.len() == 2 {
        let mut prod: i64 = 1;
        // print!("[{}, {}] ", number_pair[0], number_pair[1]);
        for num in number_pair {
          prod *= num.parse::<i64>().unwrap_or(1);

        }
        numbers.push(prod);
      }
    }
  }
  if numbers.len() > 0 {
    let mut sum: i64 = 0;
    for number in numbers {
      sum += number;
    }
    return sum;
  }
  return 0;
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
  // NOTE: 1 star
  // let mut sum = 0;
  // for tup in lines {
  //   for num in find_numbers(tup) {
  //     sum += num.parse::<i32>().unwrap_or(0);
  //   }
  // }

  let mut sum = 0;
  for tup in lines {
    let num = find_numbers_p2(tup);
    sum += num;
  }
  println!("SUM: {sum}");
}

