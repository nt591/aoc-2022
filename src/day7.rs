use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Clone, Debug, Default)]
struct DirectoryFolder<'a> {
    name: &'a str,
    parent: Option<Weak<DirectoryFolder<'a>>>,
    directories: RefCell<Vec<DirectoryFolder<'a>>>,
    files: RefCell<Vec<usize>>, // do we need names?
}

impl<'a> DirectoryFolder<'a> {
    fn directory_size(&self) -> usize {
        self.files.borrow().iter().sum::<usize>()
            + self
                .directories
                .borrow()
                .iter()
                .map(|d| d.directory_size())
                .sum::<usize>()
    }

    fn add_dir(&mut self, dir: DirectoryFolder<'a>) {
        self.directories.borrow_mut().push(dir)
    }

    fn add_file(&mut self, file: usize) {
        self.files.borrow_mut().push(file);
    }
}

fn run_impl(input: &str) -> anyhow::Result<()> {
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../data/day7.txt");
    let output = run_impl(input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day7::DirectoryFolder;
    use std::borrow::BorrowMut;
    use std::rc::Rc;
    #[test]
    fn it_sums_directories() {
        let root = Rc::new(DirectoryFolder {
            name: "Root",
            ..Default::default()
        });
        let inner = DirectoryFolder {
            name: "Inner",
            parent: Some(Rc::downgrade(&root)),
            ..Default::default()
        };
        root.files.borrow_mut().push(100);
        root.files.borrow_mut().push(200);
        inner.files.borrow_mut().push(600);
        root.directories.borrow_mut().push(inner);

        assert_eq!(900, root.directory_size());
    }
}
