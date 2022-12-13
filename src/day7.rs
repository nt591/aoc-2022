use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Clone, Debug, Default)]
struct DirectoryFolder {
    name: String,
    parent: Weak<DirectoryFolder>,
    directories: RefCell<HashMap<String, Rc<DirectoryFolder>>>,
    files: RefCell<HashMap<String, usize>>, // do we need names?
}

impl DirectoryFolder {
    fn directory_size(&self) -> usize {
        self.files.borrow().values().sum::<usize>()
            + self
                .directories
                .borrow()
                .values()
                .map(|d| d.directory_size())
                .sum::<usize>()
    }
}

fn recursive_size_sum(node: &Rc<DirectoryFolder>) -> usize {
    let mut curr = node.directory_size();
    if curr > 100000 {
        curr = 0;
    };

    let dirs = node.directories.borrow();
    curr + dirs
        .values()
        .map(|dir| recursive_size_sum(dir))
        .sum::<usize>()
}

fn part1(root: &Rc<DirectoryFolder>) {
    // sum of all subdirectory sizes <= 100000
    let sum = recursive_size_sum(root);
    println!("sum is {sum}");
}

fn recursive_size_map(node: &Rc<DirectoryFolder>) -> Vec<usize> {
    let mut sizes = vec![node.directory_size()];
    let dirs = node.directories.borrow();
    let mut child_sizes = dirs
        .values()
        .flat_map(|dir| recursive_size_map(dir))
        .collect::<Vec<usize>>();
    sizes.append(&mut child_sizes);
    sizes
}

fn part2(root: &Rc<DirectoryFolder>) {
    let max_size: usize = 70000000;
    let free_space = max_size - root.directory_size();
    let goal = 30000000;
    let space_to_free = goal - free_space;

    let sizes = recursive_size_map(&root);
    let s = sizes
        .into_iter()
        .sorted()
        .find(|s| s >= &space_to_free)
        .unwrap();
    println!("pt 2 {s}");
}

fn run_impl(input: &str) -> anyhow::Result<Rc<DirectoryFolder>> {
    let root = Rc::new(DirectoryFolder {
        name: "/".to_owned(),
        ..Default::default()
    });

    let mut curr = Rc::clone(&root);

    for (idx, line) in input.lines().enumerate() {
        let tokens = line.split(" ").collect_vec();

        match tokens[0] {
            "$" if tokens[1] == "ls" => continue,
            "$" if tokens[1] == "cd" => {
                // handle going into new dir
                let name = tokens[2];
                match name {
                    ".." => {
                        let parent = curr
                            .parent
                            .upgrade()
                            .expect("Cannot get parent of current dir");
                        curr = Rc::clone(&parent);
                    }
                    "/" => curr = Rc::clone(&root),
                    n => {
                        if curr.directories.borrow().contains_key(n) {
                            let dirs = curr.directories.borrow();
                            let c = Rc::clone(dirs.get(n).unwrap());
                            drop(dirs);
                            curr = c;
                        } else {
                            anyhow::bail!("Cannot cd into {} from {}", n, curr.name)
                        }
                    }
                }
            }
            "dir" => {
                // add new dir
                let name = tokens[1].to_owned();
                let mut dirs = curr.directories.borrow_mut();
                dirs.entry(name.clone()).or_insert(Rc::new(DirectoryFolder {
                    name,
                    parent: Rc::downgrade(&curr),
                    ..Default::default()
                }));
            }
            s if s.parse::<usize>().is_ok() => {
                // file
                let size: usize = s.parse().unwrap();
                let filename = tokens[1];
                curr.files.borrow_mut().insert(filename.to_owned(), size);
            }
            s => {
                println!("Unexpected token {} at position 0, aborting", s);
                anyhow::bail!("Failed to parse token {} at line num {}", line, idx)
            }
        }
    }

    Ok(root)
}

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../data/day7.txt");
    let output = run_impl(input)?;

    part1(&output);
    part2(&output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day7::DirectoryFolder;
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn it_sums_directories() {
        let root = Rc::new(DirectoryFolder {
            name: "Root".to_owned(),
            ..Default::default()
        });
        let inner = Rc::new(DirectoryFolder {
            name: "Inner".to_owned(),
            parent: Rc::downgrade(&root),
            ..Default::default()
        });
        root.files.borrow_mut().insert("a".to_owned(), 100);
        root.files.borrow_mut().insert("b".to_owned(), 200);
        inner.files.borrow_mut().insert("c".to_owned(), 600);
        root.directories
            .borrow_mut()
            .insert(inner.name.to_owned(), inner);

        assert_eq!(900, root.directory_size());
    }
}
