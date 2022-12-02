mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut total_score = 0;
    while let Some(line) = lines.next() {
        let round: Vec<&str> = line.split(" ").collect(); 
        match round[0] {
           "A" => {
                match round[1] {
                    "X" => {
                        total_score += 3;
                    }
                    "Y" => {
                        total_score += 4;
                    }
                    "Z" => {
                        total_score += 8;
                    }
                    _ => {
                        println!("value not recognised");
                    }
                } 
            } 
           "B" => {
                match round[1] {
                    "X" => {
                        total_score += 1;
                    }
                    "Y" => {
                        total_score += 5;
                    }
                    "Z" => {
                        total_score += 9;
                    }
                    _ => {
                        println!("value not recognised");
                    }
                } 
            } 
           "C" => {
                match round[1] {
                    "X" => {
                        total_score += 2;
                    }
                    "Y" => {
                        total_score += 6;
                    }
                    "Z" => {
                        total_score += 7;
                    }
                    _ => {
                        println!("value not recognised");
                    }
                } 
            } 
            _ => {
                println!("value not recognised");
            }
        }
    }
    println!("The final score is: {}", total_score);
}
