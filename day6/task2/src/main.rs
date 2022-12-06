mod readfile;

fn main() {
    let input = readfile::readfile("input");
    let input_as_b = input.as_bytes();
    let mut packet_marker_pos = 0;
    let mut four = String::with_capacity(4);
    let len = input_as_b.len();
    for i in 0..len-3 {
        for j in i..i+4 {
            if !four.contains(input_as_b[j] as char) {
                four.push(input_as_b[j] as char);
            }
        }
        if four.len() == 4 {
            packet_marker_pos = i + 4; 
            break;
        }
        four.clear();
    }
    let mut result = 0;
    let mut fourteen = String::with_capacity(14);
    for i in packet_marker_pos-3..len-13 {
        for j in i..i+14 {
            if !fourteen.contains(input_as_b[j] as char) {
                fourteen.push(input_as_b[j] as char);
            }
        }
        if fourteen.len() == 14 {
            result = i + 14;
            break
        }
        fourteen.clear();
    }
    println!("The result is: {}", result);
}
