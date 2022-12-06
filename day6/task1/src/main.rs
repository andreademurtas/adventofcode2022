mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let input_as_b = input.as_bytes();
    let mut result = 0;
    let mut four = String::with_capacity(4);
    let len = input_as_b.len();
    for i in 0..len-3 {
        for j in i..i+4 {
            if !four.contains(input_as_b[j] as char) {
                four.push(input_as_b[j] as char);
            }
        }
        if four.len() == 4 {
            result = i + 4; 
            break;
        }
        four.clear();
    }
    println!("The result is: {}", result);
}
