use std::fs;

fn main() {
  let file_path = "input/day2.in";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let lines = contents.split("\n");
  let sum: i32 = lines.into_iter()
  .map(parse_line_part2)
  .sum();

  println!("SUM: {}", sum);
}
fn parse_line_part2(line: &str) -> i32 {
  if let Some((_game, content)) = line.split_once(": ") {
    let mut reds: Vec<i32> = Vec::new();
    let mut greens: Vec<i32> = Vec::new();
    let mut blues: Vec<i32> = Vec::new();
    let draws = content.split("; ");
    for draw in draws {
      let balls = draw.split(", ");
      for ball in balls {
        if let Some((number, color)) = ball.split_once(" ") {
          match color {
            "red" => reds.push(number.parse::<i32>().unwrap()),
            "green" => greens.push(number.parse::<i32>().unwrap()),
            "blue" => blues.push(number.parse::<i32>().unwrap()),
            _ => {}
          }
        }
        else {
          // error
        }
      }
    }

    return reds.iter().max().unwrap_or(&1)
      * greens.iter().max().unwrap_or(&1)
      * blues.iter().max().unwrap_or(&1);
  }
  else {
    // do nothing;
    println!("Failed Initial Parse");
  }
  return 0;
}

fn parse_line_part1(line: &str) -> i32 {
  let mut possible = true;
  if let Some((game, content)) = line.split_once(": ") {

    let draws = content.split("; ");
    for draw in draws {
      let balls = draw.split(", ");
      for ball in balls {
        if let Some((number, color)) = ball.split_once(" ") {
          match color {
            "red" => if number.parse::<i32>().unwrap() > 12 { possible = false; }
            "green" => if number.parse::<i32>().unwrap() > 13 { possible = false; }
            "blue" => if number.parse::<i32>().unwrap() > 14 { possible = false; }
            _ => {}
          }
        }
        else {
          // error
        }
      }
    }

    if possible {
      if let Some((_g, number)) = game.split_once(" ") {
        return number.parse::<i32>().unwrap();
      }
      else {
        // error
      }
    }
  }
  else {
    // do nothing;
    println!("Failed Initial Parse");
  }
  return 0;
}



