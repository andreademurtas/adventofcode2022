mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut total = 0;
    while let Some(line) = lines.next() {
        let pair_raw = line.split(",")
            .collect::<Vec<&str>>();
        let pair = vec![vec![pair_raw[0].split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                            pair_raw[0].split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()],
                            vec![pair_raw[1].split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                            pair_raw[1].split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()]];
        if pair[0][0] <= pair[1][1] && pair[0][1] >= pair[1][0] {
            total += 1;
        }
    }
    println!("The total is: {}", total);
}
