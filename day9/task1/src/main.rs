mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut i_h: i16 = 0;
    let mut j_h: i16 = 0;
    let mut i_t: i16 = 0;
    let mut j_t: i16 = 0;
    let mut visited: Vec<(i16,i16)> = Vec::new();
    visited.push((0,0));
    while let Some(line) = lines.next() {
        let mv: Vec<&str> = line.split(" ").collect(); 
        let steps = mv[1].parse::<usize>().unwrap();
        for _step in 0..steps {
            match mv[0] {
                "U" => {
                    if i_t != i_h && i_t != i_h+1{
                        i_t += 1;
                        j_t = j_h;
                    }
                    i_h += 1;
                    if !visited.contains(&(i_t, j_t)) { visited.push((i_t, j_t)); }
                }
                "D" => {
                    if i_t != i_h && i_t != i_h-1 {
                        i_t -= 1;
                        j_t = j_h;
                    }
                    i_h -= 1;
                    if !visited.contains(&(i_t, j_t)) { visited.push((i_t, j_t)); }
                }
                "L" => {
                    if j_t != j_h && j_t != j_h-1 {
                        j_t = j_h;
                        i_t = i_h;
                    }
                    j_h -= 1;
                    if !visited.contains(&(i_t, j_t)) { visited.push((i_t, j_t)); }
                }
                "R" => {
                    if j_t != j_h && j_t != j_h+1 {
                        j_t = j_h;
                        i_t = i_h;
                    }
                    j_h += 1;
                    if !visited.contains(&(i_t, j_t)) { visited.push((i_t, j_t)); }
                }
                _ => {}
            }
        }
    }
    println!("The result is: {}", visited.len());
}
