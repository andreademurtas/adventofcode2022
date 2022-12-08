mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut trees: Vec<Vec<u8>> = Vec::new();
    let mut max_score = 0;
    while let Some(line) = lines.next() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push((c.to_digit(10).unwrap()) as u8);
        }
        trees.push(row);
    }
    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            let score;
            let mut score_from_left = 0;
            let mut score_from_right = 0;
            let mut score_from_top = 0;
            let mut score_from_bottom = 0;
            for h in (0..i).rev() {
                score_from_top += 1;
                if trees[h][j] >= trees[i][j] {
                    break;
                }
            }
            for h in i+1..trees.len() {
                score_from_bottom += 1;
                if trees[h][j] >= trees[i][j] {
                    break;
                }
            }
            for k in (0..j).rev() {
                score_from_left += 1;
                if trees[i][k] >= trees[i][j] {
                    break;
                }
            }
            for k in j+1..trees[0].len() {
                score_from_right += 1;
                if trees[i][k] >= trees[i][j] {
                    break;
                }
            }
            score = score_from_right * score_from_left * score_from_top * score_from_bottom;
            if score > max_score {
                max_score = score;
            }
        }
    }
    println!("The maximum score is: {}", max_score);
}
