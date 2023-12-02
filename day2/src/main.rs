use std::{fs, collections::HashMap};

fn main() {
  let test1 = fs::read_to_string("inputs/test1.txt").expect("ioerror");
  let input = fs::read_to_string("inputs/input.txt").expect("ioerror");

  println!("Test 1: {}", part1(&test1));
  println!("Part 1: {}", part1(&input));
  println!("Test 2: {}", part2(&test1));
  println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut constraint: HashMap<&str, i32> = HashMap::new();
    constraint.insert("red", 12);
    constraint.insert("green", 13);
    constraint.insert("blue", 14);

    let games = parse(input);

    let mut sum = 0;
    'gameLoop: for game in games {
        for segment in game.1 {
            if constraint.get(segment.1).unwrap() < &segment.0 {
                continue 'gameLoop;
            }
        }
        sum += game.0;
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    let games = parse(input);

    let mut sum = 0;
    for game in games {
        let mut mins: HashMap<&str, i32> = HashMap::new();
        for segment in game.1 {
            let elem = mins.get(segment.1);
            if elem.is_none() || elem.unwrap() < &segment.0 {
                mins.insert(segment.1, segment.0);
            }
        }
        let mut power = 1;
        for res in mins {
            power *= res.1;
        }
        sum += power;
    }

    return sum;
}

fn parse(input: &str) -> HashMap<i32, Vec<(i32, &str)>> {
    let mut map = HashMap::new();
    let lines = input.trim().split("\n");

    for line in lines {
        let mut p1 = line.split(": ");
        let gameno :i32 = p1.nth(0).unwrap().split(" ").nth(1).unwrap().parse().unwrap();

        let mut game = Vec::new();

        let segments = p1.nth(0).unwrap().split(&[',', ';'][..]);
        for segment in segments {
            let mut p2 = segment.trim().split(" ");
            let qty: i32 = p2.nth(0).unwrap().trim().parse().unwrap();
            let typ = p2.nth(0).unwrap().trim();
            game.push((qty, typ));
        }

        map.insert(gameno, game);
    }

    return map;
}
