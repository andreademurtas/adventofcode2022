mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut knots: Vec<(i16,i16)> = Vec::new();
    for _i in 0..10 { knots.push((0,0)); }
    let mut visited: Vec<(i16,i16)> = Vec::new();
    while let Some(line) = lines.next() {
        let mv: Vec<&str> = line.split(" ").collect(); 
        let steps = mv[1].parse::<usize>().unwrap();
        for _step in 0..steps {
            match mv[0] {
                "U" => {
                    knots[9].0 += 1;
                    for ith in (0..9).rev() {
                        knots[ith] = calculate_motion(knots[ith+1].0, knots[ith+1].1, knots[ith].0, knots[ith].1);
                    } 
                    if !visited.contains(&(knots[0])) { visited.push(knots[0]); }
                }
                "D" => {
                    knots[9].0 -= 1;
                    for ith in (0..9).rev() {
                        knots[ith] = calculate_motion(knots[ith+1].0, knots[ith+1].1, knots[ith].0, knots[ith].1);
                    } 
                    if !visited.contains(&(knots[0])) { visited.push(knots[0]); }
                }
                "L" => {
                    knots[9].1 -= 1;
                    for ith in (0..9).rev() {
                        knots[ith] = calculate_motion(knots[ith+1].0, knots[ith+1].1, knots[ith].0, knots[ith].1);
                    } 
                    if !visited.contains(&(knots[0])) { visited.push(knots[0]); }
                }
                "R" => {
                    knots[9].1 += 1;
                    for ith in (0..9).rev() {
                        knots[ith] = calculate_motion(knots[ith+1].0, knots[ith+1].1, knots[ith].0, knots[ith].1);
                    } 
                    if !visited.contains(&(knots[0])) { visited.push(knots[0]); }
                }
                _ => {}
            }
        }
    }
    println!("The result is: {}", visited.len());
}

fn calculate_motion(i_1: i16, j_1: i16, i_2: i16, j_2: i16) -> (i16, i16) {
    let mut i_res = i_2;
    let mut j_res = j_2;
    if i_1 == i_2 {
        if j_1 > j_2 + 1 { j_res += 1; }
        else if j_1 < j_2 - 1 { j_res -= 1; }
    }
    else if j_1 == j_2 {
        if i_1 > i_2 + 1 { i_res += 1; }
        else if i_1 < i_2 - 1 { i_res -= 1; }
    }
    else if (i_1 > i_2 + 1 && j_1 < j_2) || (i_1 > i_2 && j_1 < j_2 - 1) {
        i_res += 1;
        j_res -= 1;
    }
    else if (i_1 > i_2 + 1 && j_1 > j_2) || (i_1 > i_2 && j_1 > j_2 + 1){
        i_res += 1;
        j_res += 1;
    }
    else if (i_1 < i_2 - 1 && j_1 < j_2) || (i_1 < i_2 && j_1 < j_2 - 1) {
        i_res -= 1;
        j_res -= 1;
    }
    else if (i_1 < i_2 - 1 && j_1 > j_2) || (i_1 < i_2 && j_1 > j_2 + 1){
        i_res -= 1;
        j_res += 1;
    }
    (i_res, j_res)
}

