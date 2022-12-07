#[derive(Clone, Debug, Default)]
struct DirectoryFolder<'a> {
    name: &'a str,
    directories: Vec<DirectoryFolder<'a>>,
    files: Vec<usize>, // do we need names?
}

impl<'a> DirectoryFolder<'a> {
    fn directory_size(&self) -> usize {
        self.files.iter().sum::<usize>()
            + self
                .directories
                .iter()
                .map(|d| d.directory_size())
                .sum::<usize>()
    }

    fn add_dir(&mut self, dir: DirectoryFolder<'a>) {
        self.directories.push(dir)
    }

    fn add_file(&mut self, file: usize) {
        self.files.push(file);
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
    #[test]
    fn it_sums_directories() {
        let mut root = DirectoryFolder {
            name: "Root",
            ..Default::default()
        };
        let mut inner = DirectoryFolder {
            name: "Inner",
            ..Default::default()
        };
        root.add_file(100);
        root.add_file(200);
        inner.add_file(600);
        root.add_dir(inner);

        assert_eq!(900, root.directory_size());
    }
}
