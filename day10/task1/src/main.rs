mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut register: i32 = 1;
    let mut cycle: i32 = 0;
    let mut tot_pow = 0;
    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(" ").collect(); 
        match split[0] {
            "noop" => { cycle += 1; if (cycle - 20) % 40 == 0 { tot_pow += cycle * register }}
            "addx" => {
                for _i in 0..2 {
                    cycle += 1; if (cycle - 20) % 40 == 0 { tot_pow += cycle * register }
                }
                register += split[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }
    println!("The total power is: {}", tot_pow);
}
