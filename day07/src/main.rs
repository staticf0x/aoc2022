use std::collections::HashMap;

fn path_to_str(path: &Vec<String>) -> String {
    if path.len() == 1 {
        return String::from("/");
    }

    let path_str = path.join("/");
    let path_str_clean = path_str.strip_prefix("/").unwrap();

    path_str_clean.to_string()
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut path: Vec<String> = vec![];
    let mut ls_size: usize = 0;
    let mut sizes: HashMap<String, usize> = HashMap::new();

    for line in file.lines() {
        println!("Line: {line}");

        if line.starts_with("$") {
            if ls_size > 0 {
                let mut parts: Vec<String> = vec![];

                for part in &path {
                    parts.push(part.to_string());
                    let key = path_to_str(&parts);

                    println!("Adding {ls_size} bytes to {key}");
                    sizes
                        .entry(key)
                        .and_modify(|e| *e += ls_size)
                        .or_insert(ls_size);
                }
                ls_size = 0;
            }

            // Command
            if line == "$ ls" {
                ls_size = 0;
            } else if line == "$ cd .." {
                // Pop path
                path.pop();

                // Display path
                let path_str = path_to_str(&path);
                println!("{path_str}");
            } else {
                // cd into directory
                path.push(line.chars().skip(5).collect());

                // Display path
                let path_str = path_to_str(&path);
                println!("{path_str}");
            }
        } else {
            // ls output
            let (size_str, _) = line.split_once(" ").unwrap();

            if size_str != "dir" {
                let size_bytes: usize = size_str.parse().unwrap();
                ls_size += size_bytes;
            }
        }
    }

    if ls_size > 0 {
        let mut parts: Vec<String> = vec![];

        for part in &path {
            parts.push(part.to_string());
            let key = path_to_str(&parts);

            println!("Adding {ls_size} bytes to {key}");

            sizes
                .entry(key)
                .and_modify(|e| *e += ls_size)
                .or_insert(ls_size);
        }
    }

    let mut total: usize = 0;

    for (_, size) in &sizes {
        if *size < 100000 {
            total += *size;
        }
    }

    dbg!("{:?}", &sizes);
    println!("Total: {total}");
}
