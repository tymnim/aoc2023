use std::fs;

fn main() {
  let file_path = "input/day4.in";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let lines = contents.split("\n");
  let games = lines.into_iter()
  .map(parse_ticket);
  // NOTE: part1
  // .map(|count| {
  //   if count > 0 {
  //     let pp = u32::pow(2, (count - 1) as u32);
  //     return pp;
  //   }
  //   return 0;
  // })
  // .sum();

  let mut game_scores: Vec<(u32, u32)> = games.map(|score| (score, 1)).collect();
  game_scores = game_scores[0..game_scores.len() - 1].to_vec();
  let limit = game_scores.len();
  for i in 0..limit {
    let (score, tickets) = game_scores[i];
    for j in 1..(score + 1) as usize {
      let game = i + j;
      if game < limit {
        game_scores[game].1 += tickets
      }
    }
  }
  let sum: u32 = game_scores.iter().map(|(_, tickets)| tickets).sum();
  println!("SUM: {sum}");

}

fn parse_ticket(line: &str) -> u32 {
  if let Some((_, numbers)) = line.split_once(": ") {
    if let Some((winning, mine)) = numbers.split_once(" | ") {
      let winning_numbers: Vec<&str> = winning.split(" ").filter(| number | number.len() > 0).collect();
      let my_numbers = mine.split(" ").filter(| number | number.len() > 0).into_iter();

      let my_winning: Vec<i32> = my_numbers
        .filter(|number| winning_numbers.contains(number))
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
      return my_winning.len() as u32;
    }
  }
  return 0;
}


