mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut check = false;
    let mut not_formed = true;
    while let Some(line) = lines.next() {
        if line == "" { check = true; continue; }
        if check == false {
            let mut i = 1;
            let mut j = 0;
            let chars = line.as_bytes();
            loop {
                if i < line.len() {
                    let c = chars[i] as char;
                    if c == '1' { break; }
                    if not_formed {
                        stacks.push(Vec::new());
                    }
                    if c != ' ' {
                        stacks[j].insert(0,c);
                    }
                    i += 4;
                    j += 1;
                }
                else {
                    break;
                }
            }
            not_formed = false;
        }
        else {
            let split: Vec<&str> = line.split(" ").collect();
            let quantity = split[1].parse::<usize>().unwrap();
            let from = split[3].parse::<usize>().unwrap();
            let to = split[5].parse::<usize>().unwrap();
            for i in (0..quantity).rev() {
                let to_be_moved= stacks[from-1][stacks[from-1].len() - 1 - i];
                stacks[to-1].push(to_be_moved);
            }
            for _i in 0..quantity {
                stacks[from-1].pop();
            }
        }
    }
    let mut message = String::new();
    for stack in stacks {
        message.push(stack[stack.len() - 1]);
    }
    println!("The message is: {}", message);
}
