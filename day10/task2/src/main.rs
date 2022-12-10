mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut register: i32 = 1;
    let mut screen = vec![vec![""; 40]; 6];
    let mut i = 0; let mut j = 0;
    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(" ").collect(); 
        match split[0] {
            "noop" => { 
                if j >= register-1 && j <= register+1 { screen[i as usize][j as usize] = "#" } 
                else { screen[i as usize][j as usize] = "." }
                if j + 1 == 40 { i += 1; j = 0}
                else { j += 1; }
            }
            "addx" => {
                for _i in 0..2 {
                    if j >= register-1 && j <= register+1 { screen[i as usize][j as usize] = "#" } 
                    else { screen[i as usize][j as usize] = "." }
                    if j + 1 == 40 { i += 1; j = 0}
                    else { j += 1; }
                }
                register += split[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    for elem in screen {
        println!("{:?}", elem);
    }
}
