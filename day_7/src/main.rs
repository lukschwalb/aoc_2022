use std::collections::HashMap;
use std::collections::HashSet;

struct Directory {
    name: String,
    files: HashMap<String, u32>,
    subdirs: HashSet<String>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name: name,
            files: HashMap::new(),
            subdirs: HashSet::new(),
        }
    }
}

fn get_size(directories: &HashMap<String, Directory>, check_dir: &String) -> u32 {
    let mut total_size: u32 = 0;

    if let Some(dir) = directories.get(check_dir) {
        total_size += dir.files.values().sum::<u32>();
        for subdir in dir.subdirs.iter() {
            let subdir_str: String = format!("{}/{}", check_dir.trim_end_matches('/'), subdir);
            total_size += get_size(directories, &subdir_str);
        }
    }

    return total_size;
}

fn main() {
    let input = include_str!("../aoc_input.txt");

    let mut current_dir = "/".to_string();
    let mut directories = HashMap::new();

    for line in input.lines() {
        let space_split: Vec<&str> = line.split(" ").collect();

        if line.starts_with("$ cd") {
            if space_split.len() != 3 { panic!("invalid cd command: {}", line) }

            if space_split[2] == ".." {
                current_dir = current_dir.trim_end_matches(|c| c != '/').trim_end_matches('/').to_string();
            } else if space_split[2].starts_with('/') {
                current_dir = space_split[2].to_string();
            } else {
                if !current_dir.ends_with('/') {
                    current_dir.push_str("/");
                }
                current_dir.push_str(space_split[2]);
            }
            if !directories.contains_key(&current_dir) {
                directories.insert(current_dir.clone(), Directory::new(current_dir.clone()));
            }
        } else if line.starts_with("$ ls") {
            // We dont have to do anything in this case
        } else {
            let dir = directories.get_mut(&current_dir).unwrap();

            if line.starts_with("dir") {
                dir.subdirs.insert(space_split[1].to_string());
            } else {
                let size: u32 = space_split[0].parse().unwrap();
                dir.files.insert(space_split[1].to_string(), size);
            }
        }
    }

    let size: u32 = directories.values()
        .map(|x| get_size(&directories, &x.name))
        .filter(|x| x <= &100000)
        .sum();
    println!("Sum of directories smaller than 10.000 is: {}", size);

    let max_space: u32 = 70000000;
    let used_space: u32 = get_size(&directories, &directories.get("/").unwrap().name);
    println!("Used space: {}", used_space);
    let free_space: u32 = max_space - used_space;

    let closest_size: u32 = directories.values()
        .map(|x| get_size(&directories, &x.name))
        .filter(|x| free_space + x > 30000000)
        .min()
        .unwrap();
    
    println!("Size of directory to delete is: {}", closest_size);

}
