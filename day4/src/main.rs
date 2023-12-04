use std::{fs, collections::HashMap};

fn main() {
  let test1 = fs::read_to_string("inputs/test1.txt").expect("ioerror");
  let input = fs::read_to_string("inputs/input.txt").expect("ioerror");

  println!("Test 1: {}", part1(&test1));
  println!("Part 1: {}", part1(&input));
  println!("Test 2: {}", part2(&test1));
  println!("Part 2: {}", part2(&input));
}

fn to_i32_list(line: &str) -> Vec<u32> {
    return line.trim().split(" ").filter_map(|x| -> Option<u32> {
        let y = x.trim();
        if y.len() > 0 {
            let z = y.parse();
            if z.is_ok() {
                return Some(z.unwrap());
            }
        }
        return None;
    }).collect();
}

fn parse(input: &str) -> Vec<u32> {
    let lines = input.trim().split("\n");
    let mut matches = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(&[':','|']).collect();
        let my_tims = to_i32_list(parts.get(1).unwrap());
        let their_nums = to_i32_list(parts.get(2).unwrap());

        let mut match_count = 0;
        for num in my_tims {
            if their_nums.contains(&num) {
                match_count += 1;
            }
        }

        matches.push(match_count);
    }
    return matches;
}

fn part1(input: &str) -> i32{
    let matches = parse(input);

    let mut sum = 0;
    for m in matches {
        if m > 0 {
            sum += i32::pow(2, m - 1);
        }
    }

    return sum;
}

fn part2(input: &str) -> i32{
    let matches = parse(input);
    let mut extra_copies = HashMap::new();
    let mut sum = 0;
    let mut i = 0;
    while i < matches.len() {
        let my_count = 1 + extra_copies.get(&i).unwrap_or(&0);
        
        let m = matches.get(i).unwrap();
        if m > &0 {
            let mut j = 0;
            let mut k = i + 1;
            while &j < m {
                let new_copies = my_count + extra_copies.get(&k).unwrap_or(&0);
                extra_copies.insert(k, new_copies);
                j += 1;
                k += 1;
            }
        }
        sum += my_count;
        i += 1;
    }

    return sum;
}