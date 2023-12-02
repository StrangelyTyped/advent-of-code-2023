use std::{fs, collections::HashMap};

fn main() {
  let test1 = fs::read_to_string("inputs/test1.txt").expect("ioerror");
  let test2 = fs::read_to_string("inputs/test2.txt").expect("ioerror");
  let input = fs::read_to_string("inputs/input.txt").expect("ioerror");

  let part1 = mapping_part1();
  let part2 = mapping_part2();

  compute(&test1, &part1);
  compute(&input, &part1);
  compute(&test2, &part2);
  compute(&input, &part2);
}

fn mapping_part1() -> HashMap<&'static str, i32>{
  let mut mapping: HashMap<&str, i32> = HashMap::new();
  mapping.insert("1", 1);
  mapping.insert("2", 2);
  mapping.insert("3", 3);
  mapping.insert("4", 4);
  mapping.insert("5", 5);
  mapping.insert("6", 6);
  mapping.insert("7", 7);
  mapping.insert("8", 8);
  mapping.insert("9", 9);
  mapping.insert("0", 0);
  return mapping;
}

fn mapping_part2() -> HashMap<&'static str, i32>{
  let mut mapping: HashMap<&str, i32> = HashMap::new();
  mapping.insert("1", 1);
  mapping.insert("2", 2);
  mapping.insert("3", 3);
  mapping.insert("4", 4);
  mapping.insert("5", 5);
  mapping.insert("6", 6);
  mapping.insert("7", 7);
  mapping.insert("8", 8);
  mapping.insert("9", 9);
  mapping.insert("0", 0);
  mapping.insert("one", 1);
  mapping.insert("two", 2);
  mapping.insert("three", 3);
  mapping.insert("four", 4);
  mapping.insert("five", 5);
  mapping.insert("six", 6);
  mapping.insert("seven", 7);
  mapping.insert("eight", 8);
  mapping.insert("nine", 9);
  return mapping;
}

fn compute(input :&String, map :&HashMap<&str, i32>){
  let data = input.trim();
  let mut sum = 0;
  for line in data.split("\n") {
    let mut vals: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < line.len() {
      for keyw in map.keys() {
        if line[i..].starts_with(keyw) {
          vals.push(*map.get(keyw).unwrap());
          break;
        }
      }
      i += 1;
    }
    if !vals.is_empty(){
      sum += (vals.first().unwrap() * 10) + vals.last().unwrap();
    }
  }
  println!("Result: {}", sum);
}
