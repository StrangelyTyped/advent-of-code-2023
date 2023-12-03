use std::{fs, collections::HashMap, cmp};

fn main() {
  let test1 = fs::read_to_string("inputs/test1.txt").expect("ioerror");
  let input = fs::read_to_string("inputs/input.txt").expect("ioerror");

  println!("Test 1: {}", part1(&test1));
  println!("Part 1: {}", part1(&input));
  println!("Test 2: {}", part2(&test1));
  println!("Part 2: {}", part2(&input));
}

fn is_digit(chr: char) -> bool {
    return chr >= '0' && chr <= '9';
}
fn is_symbol(chr: char) -> bool {
    return chr != '.' && (chr < '0' || chr > '9');
}
fn is_gear(chr: char) -> bool {
    return chr == '*';
}

fn compute(input: &str, mut pred: impl FnMut(char, i32, (usize, usize)) -> bool) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut chrs: Vec<Vec<char>> = Vec::new();
    for line in lines {
        chrs.push(line.chars().collect());
    }

    let mut y = 0;
    while y < chrs.len() {
        let mut x = 0;
        'numLoop: while x < chrs[y].len() {
            if is_digit(chrs[y][x]) {
                let mut numStr = String::new();
                numStr.push(chrs[y][x]);
                let x_start = x;
                x += 1;
                while x < chrs[y].len() && is_digit(chrs[y][x]){
                    numStr.push(chrs[y][x]);
                    x += 1;
                }
                let num = numStr.parse().unwrap();
                
                let mut y2 = if y > 1 { y - 1 } else { 0 };
                while y2 < cmp::min(chrs.len() - 1, y + 2){
                    let mut x2 = if x_start > 1 { x_start - 1 } else { 0 };
                    while x2 < cmp::min(chrs[y2].len() - 1, x+1) {
                        if pred(chrs[y2][x2], num, (x2, y2)) {
                            x += 1;
                            continue 'numLoop;
                        }
                        x2 += 1;
                    }
                    y2 += 1;
                }
                 
            }
            x += 1;
        }
        y += 1;
    }
    
}

fn part1(input: &str) -> i32{
    let mut sum = 0;

    let pred = |chr: char, num: i32, coord: (usize, usize)| -> bool { 
        if is_symbol(chr) {
            sum += num;
            return true;
        }
        return false;
    };
    compute(input, pred);

    return sum;
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut gearMap: HashMap<(usize, usize), i32> = HashMap::new();

    let pred = |chr: char, num: i32, coord: (usize, usize)| -> bool { 
        if is_gear(chr) {
            if gearMap.contains_key(&coord) {
                sum += num * gearMap.get(&coord).unwrap();
            }else{
                gearMap.insert(coord, num);
            }
            return true;
        }
        return false;
    };
    compute(input, pred);

    return sum;
}