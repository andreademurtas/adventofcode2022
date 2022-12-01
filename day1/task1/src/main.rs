mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut elf: u32 = 0;
    let mut max: u32 = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            if elf > max {max=elf}
            elf = 0;
        }
        else {
            elf += line.parse::<u32>().unwrap();
        }
    }
    println!("The max is: {}", max);
}
