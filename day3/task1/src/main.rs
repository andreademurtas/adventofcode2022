mod readfile;

static ALPH: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut total_priorities = 0;
    let mut check: bool;
    while let Some(line) = lines.next() {
        let compartments = vec![&line[..(line.len()/2)], &line[line.len()/2..]];
        check = false;
        for item1 in compartments[0].chars() {
            for item2 in compartments[1].chars() {
                if item1==item2 {
                    for item3 in ALPH.chars().enumerate() {
                        if item3.1 == item1 { 
                            total_priorities += item3.0 + 1; 
                            check = true;
                            break;
                        } 
                    }
                    break;
                }
            }
            if check == true {break;}
        }
    }
    println!("The total priority is: {}", total_priorities);
}
