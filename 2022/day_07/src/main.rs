use std::fs;

#[derive(Debug)]
struct Tree {
    name: String,
    dirs: Vec<Tree>,
    files: Vec<File>,
}


impl Tree {

    fn add_files(&mut self, ls: &Vec<&str>) {
        for line in ls {
            if line.starts_with("dir ") {
                continue;
            }

            let parts: Vec<&str> = line.split(' ').collect();
            let file_name = parts[1].to_string();
            let file = self.get_file(&file_name);

            if file.is_none() {
                let file_size = parts[0].parse::<usize>().unwrap();

                let new_file = File {
                    name: file_name,
                    size: file_size,
                };

                self.files.push(new_file);
            }
        }
    }

    fn create_path(&mut self, path: &String) {
        if path.len() == 0 || path == "/" {
            return;
        }

        let mut parts: Vec<&str> = path.split('/').collect();

        if parts[0].len() == 0 {
            parts = parts[1..].to_vec();
        }

        let name = parts[0].to_string();
        let check_node = self.get_node(&name);

        if check_node.is_none() {
            let mut subdir = Tree {
                name: name,
                dirs: Vec::new(),
                files: Vec::new(),
            };

            subdir.create_path(&parts[1..].join("/"));
            self.dirs.push(subdir);
        }
        else {
            check_node.unwrap().create_path(&parts[1..].join("/"));
        }
    }

    fn get_file(&mut self, name: &String) -> Option<&mut File> {
        return self.files.iter_mut().find(|file| file.name == *name);
    }

    fn get_node(&mut self, path: &String) -> Option<&mut Self> {
        let parts: Vec<&str> = path.split('/').collect();
        let mut node = self;

        if parts.len() > 0 && parts[0] != "/" && parts[0].len() > 0 {
            let result = node.dirs.iter_mut().find(|dir| dir.name == parts[0].to_string());

            if result.is_none() {
                return result;
            }

            let rest_path = parts[1..].to_vec().join("/");
            node = result.unwrap().get_node(&rest_path).unwrap();
        }

        return Some(node);
    }

    fn get_dir_list_with_size_limit_max(&self, limit: usize) -> Vec<usize> {
        // Very inefficient solution. Every directory is
        // checked multiple times: To compute the size and
        // then to get its own directory list.
        let mut list: Vec<usize> = Vec::new();
        let size = self.get_size();

        if size <= limit {
            list.push(size);
        }

        for node in &self.dirs {
            let mut sub_list = node.get_dir_list_with_size_limit_max(limit);
            list.append(&mut sub_list);
        }

        list
    }

    fn get_dir_list_with_size_limit_min(&self, limit: usize) -> Vec<usize> {
        // Very inefficient solution. Every directory is
        // checked multiple times: To compute the size and
        // then to get its own directory list.
        let mut list: Vec<usize> = Vec::new();
        let size = self.get_size();

        if size >= limit {
            list.push(size);
        }

        for node in &self.dirs {
            let mut sub_list = node.get_dir_list_with_size_limit_min(limit);
            list.append(&mut sub_list);
        }

        list
    }

    fn get_size(&self) -> usize {
        let mut sum = 0;

        for file in &self.files {
            sum += file.size;
        }

        for node in &self.dirs {
            sum += node.get_size();
        }

        sum
    }

}


#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}


fn change_directory(current_path: &mut String, cmd_path: &str) {
    if cmd_path.starts_with("/") {
        *current_path = cmd_path[1..].to_string();
    }
    else if cmd_path == ".." {
        let start_index = match current_path.starts_with("/") {
            true => 1,
            false => 0,
        };

        let mut parts: Vec<&str> = current_path.split('/').collect();
        parts = parts[start_index..parts.len() - 1].to_vec();

        *current_path = parts.join("/");
    }
    else {
        if current_path.len() > 0 {
            *current_path += "/";
        }

        *current_path += cmd_path;
    }
}


fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt.");

    let mut current_path = String::new();
    let mut gather_lines = false;
    let mut ls: Vec<&str> = Vec::new();

    let mut root = Tree {
        name: String::from(""),
        dirs: Vec::new(),
        files: Vec::new(),
    };

    for line in content.lines() {
        if line.starts_with("$ ") {
            if gather_lines {
                let node = root.get_node(&current_path);

                if !node.is_none() {
                    node.unwrap().add_files(&ls);
                }
            }

            gather_lines = false;
            ls.clear();
        }

        if line.starts_with("$ cd") {
            let cmd_path = &line[5..];
            change_directory(&mut current_path, &cmd_path);
            root.create_path(&current_path);
        }
        else if line.starts_with("$ ls") {
            gather_lines = true;
        }
        else if gather_lines {
            ls.push(line);
        }
    }

    let list_min = root.get_dir_list_with_size_limit_max(100_000);
    let sum: usize = list_min.iter().sum();
    println!("The sum of all directories with the given limit is {}.", sum);

    let disk_space = 70_000_000;
    let required_free = 30_000_000;
    let used_size = root.get_size();
    let currently_free = disk_space - used_size;
    let space_needed = required_free - currently_free;

    let mut list_max = root.get_dir_list_with_size_limit_min(space_needed);
    list_max.sort();

    println!("{} is required to install the update.", space_needed);
    println!("The smallest directory above the limit has size {}.", list_max[0]);
}
