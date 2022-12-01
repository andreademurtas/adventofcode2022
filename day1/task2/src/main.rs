mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut elf: u32 = 0;
    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            if elf > max1 {max3=max2;max2=max1;max1=elf}
            else if elf > max2 {max3=max2;max2=elf}
            else if elf > max3 {max3=elf}
            elf = 0;
        }
        else {
            elf += line.parse::<u32>().unwrap();
        }
    }
    println!("The max is: {}", max1+max2+max3);
}
