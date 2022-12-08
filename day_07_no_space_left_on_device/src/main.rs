use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    pub children: HashMap<String, Member>,
}

#[derive(Debug)]
struct File {
    pub size: u64,
}

#[derive(Debug)]
enum Member {
    Directory(Directory),
    File(File),
}

type Filesystem = Directory;

fn construct_filesystem(input: &str) -> Filesystem {
    let mut fs = Filesystem {
        children: HashMap::default(),
    };

    let mut current_dir = "/".to_owned();
    input.split("$ ").filter(|x| !x.is_empty()).for_each(|cmd| {
        let (cmd, response) = cmd.split_once(['\n']).unwrap_or((cmd, ""));
        let (cmd, args) = cmd.split_once(' ').unwrap_or((cmd, ""));
        match cmd.trim() {
            "cd" => match args {
                ".." => {
                    current_dir = current_dir.rsplit_once('/').unwrap().0.to_owned();
                }
                "/" => current_dir = "/".to_owned(),
                path => {
                    current_dir += "/";
                    current_dir += path;
                }
            },
            "ls" => {
                response.lines().for_each(|line| {
                    let (meta, name) = line.split_once(' ').unwrap();
                    let dir = current_dir
                        .split('/')
                        .fold(&mut fs, |acc: &mut Directory, cur| {
                            if let Member::Directory(dir) =
                                acc.children.entry(cur.to_owned()).or_insert_with(|| {
                                    Member::Directory(Directory {
                                        // parent: Some(acc),
                                        children: HashMap::default(),
                                    })
                                })
                            {
                                dir
                            } else {
                                panic!("cannot cd into file")
                            }
                        });

                    dir.children.insert(
                        name.to_owned(),
                        if meta == "dir" {
                            Member::Directory(Directory {
                                // parent: Some(dir),
                                children: HashMap::default(),
                            })
                        } else {
                            Member::File(File {
                                size: meta.parse::<u64>().unwrap(),
                            })
                        },
                    );
                });
            }
            _ => panic!("invalid input, unknown command {cmd}"),
        }
    });

    fs
}

fn find_sizes(dir: &Directory) -> (u64, Vec<u64>) {
    let mut sizes = vec![];
    let mut sum = 0;
    dir.children.iter().for_each(|child| {
        sum += match child.1 {
            Member::Directory(ndir) => {
                let (size, mut dsizes) = find_sizes(ndir);
                sizes.append(&mut dsizes);
                size
            }
            Member::File(file) => file.size,
        };
    });
    dbg!(sum);
    sizes.push(sum);
    (sum, sizes)
}

fn count_total_directory_size(input: &str) -> u64 {
    let fs = construct_filesystem(input);
    find_sizes(&fs)
        .1
        .into_iter()
        .filter(|size| *size <= 100000)
        .sum()
}

fn main() {
    println!(
        "total directory size of at most 100000: {}",
        count_total_directory_size(include_str!("input.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_total_directory_size(include_str!("test.txt")), 95437);
    }
}
