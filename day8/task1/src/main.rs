mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut trees: Vec<Vec<u8>> = Vec::new();
    let mut visible = 0;
    while let Some(line) = lines.next() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push((c.to_digit(10).unwrap()) as u8);
        }
        trees.push(row);
    }
    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            if i == 0 || j == 0 || i == trees.len() - 1 || j == trees[0].len() - 1 {
                visible += 1;
            }
            else {
                let mut from_left = false;
                let mut from_right = false;
                let mut from_top = false;
                let mut from_bottom = false;
                for h in 0..i {
                    if trees[h][j] >= trees[i][j] {
                        from_top = false;
                        break;
                    }
                    else {
                        from_top = true;
                    }
                }
                for h in i+1..trees.len() {
                    if trees[h][j] >= trees[i][j] {
                        from_bottom = false;
                        break;
                    }
                    else {
                        from_bottom = true;
                    }
                }
                for k in 0..j {
                    if trees[i][k] >= trees[i][j] {
                        from_left = false;
                        break;
                    }
                    else {
                        from_left = true;
                    }
                }
                for k in j+1..trees[0].len() {
                    if trees[i][k] >= trees[i][j] {
                        from_right = false;
                        break;
                    }
                    else {
                        from_right = true;
                    }
                }
                if from_top || from_bottom || from_left || from_right {
                    visible += 1;
                }
            }
        }
    }
    println!("There are {} visible trees", visible);
}
