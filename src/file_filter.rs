use std::path::Path;

use globset::{Glob, GlobSet};

type Result<T> = std::result::Result<T, globset::Error>;

#[derive(Debug)]
pub struct FileFilter {
    pub include: GlobSet,
    pub ignore: GlobSet,
}

static INCLUDE_SYMBOL: char = '!';

impl FileFilter {
    pub fn new(pattern: Vec<String>) -> Result<FileFilter> {
        let include = pattern.iter()
        .filter(|pattern| pattern.starts_with(INCLUDE_SYMBOL))
        .map(|pattern| pattern.trim_start_matches(INCLUDE_SYMBOL).to_string())
        .collect::<Vec<String>>();

        // remove ! from the include patterns
        let ignore = pattern.iter()
        .filter(|pattern| !pattern.starts_with(INCLUDE_SYMBOL))
        .map(|pattern| pattern.to_string())
        .collect::<Vec<String>>();


        let mut include_builder = GlobSet::builder();


        for pattern in include {
            let pattern = Glob::new(&pattern)?;
            include_builder.add(pattern);
        }

        let include = include_builder.build()?;

        for pattern in ignore {
            let pattern = Glob::new(&pattern)?;
            include_builder.add(pattern);
        }

        let ignore = include_builder.build()?;
    
        // glob pattern
        Ok(FileFilter {
            include,
            ignore,
        })
    }

    pub fn allow_file(&self, file: &Path, root: &Path) -> bool {
        let path = file.strip_prefix(root).unwrap();
        !self.ignore.is_match(path) || self.include.is_match(path) 
    }
}