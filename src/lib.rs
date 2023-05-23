use glob::glob;
use std::error::Error;

pub struct GlobObj {
    pub iter: glob::Paths,
}

impl Iterator for GlobObj {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(path)) => {
                Some(path.to_str().unwrap().to_owned())
            }
            _ => None,
        }
    }
}

pub fn globobj(pattern: &str) -> Result<GlobObj, Box<dyn Error>> {
    let iter = glob(pattern)?;
    Ok(GlobObj { iter })
}

#[test]
fn basic() {
    for target in globobj("./**/*.rs").unwrap() {
        println!("target: {}", target);
    }
}

