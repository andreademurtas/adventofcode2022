mod readfile;
use std::collections::HashMap; 

fn main() {
    let input = readfile::readfile("input");
    let mut lines = input.lines();
    let mut curr_path = Vec::new();
    let mut paths: HashMap<String, i32> = HashMap::new();
    while let Some(line) = lines.next() {
        let line_sp: Vec<&str> = line.split(" ").collect();
        if line_sp[0] == "$" {
            match line_sp[1] {
                "cd" => {
                    if line_sp[2] != ".." {
                        if line_sp[2] == "/" {
                            curr_path.clear();
                            curr_path.push("/");
                        }
                        else {
                            curr_path.push(line_sp[2]);
                        }
                        paths.entry(curr_path.join("/").to_string()).or_insert(0);
                    }
                    else {
                        curr_path.pop();
                    }
                }
                _ => {}
            } 
        } 
    }
    lines = input.lines();
    curr_path.clear();
    while let Some(line) = lines.next() {
        let line_sp: Vec<&str> = line.split(" ").collect();
        if line_sp[0] == "$" {
            match line_sp[1] {
                "cd" => {
                    if line_sp[2] != ".." {
                        if line_sp[2] == "/" {
                            curr_path.clear();
                            curr_path.push("/");
                        }
                        else {
                            curr_path.push(line_sp[2]);
                        }
                    }
                    else {
                        curr_path.pop();
                    }
                }
                _ => {}
            } 
        } 
        else {
            match line_sp[0] {
                "dir" => {
                }
                _ => {
                    for i in 0..curr_path.len() {
                        paths.entry(curr_path[..curr_path.len()-i].join("/").to_string()).and_modify(|x| *x += line_sp[0].parse::<i32>().unwrap());
                    }
                }
            }
        }
    }
    let mut result = 0;
    for (_key, value) in paths.clone().into_iter() {
        if value <= 100000 {
            result += value;
        }
    } 
    //println!("What hell looks like: {:?}", paths);
    println!("The result is: {}", result);
}
