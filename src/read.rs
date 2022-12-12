use ::std::fmt::Debug;
use std::{fs, path::PathBuf, str::FromStr};


pub fn read_packets_from_file<T: FromStr>(file: impl Into<PathBuf>) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    let content = fs::read_to_string(file.into()).expect("file not found");

    content
        .split("\n\n")
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<T>().unwrap())
                .collect()
        })
        .collect()
}
