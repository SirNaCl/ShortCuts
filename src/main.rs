extern crate savefile;
use savefile::prelude::*;
use std::collections::HashMap;
use std::env;

#[macro_use]
extern crate savefile_derive;
#[derive(Savefile)]
struct ShortCuts {
    shortcuts: HashMap<String, String>,
}

fn parse_args(a: &Vec<String>) -> Option<Vec<String>> {
    if a.is_empty() {
        return None;
    }
    let mut newvec = a.clone();
    newvec.remove(0);
    return Some(newvec);
}

fn save_shortcuts(sc: &ShortCuts, p: &str) {
    save_file(p, 0, sc).unwrap();
}

fn load_shortcuts(p: &str) -> ShortCuts {
    load_file(p, 0).unwrap_or(ShortCuts {
        shortcuts: HashMap::new(),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = parse_args(&args);
    println!("{:?}", n);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{load_shortcuts, parse_args, save_shortcuts, ShortCuts};

    #[test]
    fn test_parse_args() {
        let arg = vec!["prog".to_string(), "1".to_string(), "2".to_string()];
        let res = parse_args(&arg);

        match res {
            Some(a) => assert!(vec_is_eq(&a, &vec!["1".to_string(), "2".to_string()])),
            None => assert!(false),
        }
    }

    #[test]
    fn test_save_load() {
        let mut test_sc = ShortCuts {
            shortcuts: HashMap::new(),
        };
        test_sc
            .shortcuts
            .insert("home".to_string(), "~".to_string());
        save_shortcuts(&test_sc, "test_data/save.bin");
        let loaded = load_shortcuts("test_data/save.bin");

        // Test that we can save and load file
        assert!(hm_is_eq(&test_sc.shortcuts, &loaded.shortcuts));
    }

    #[test]
    fn test_load_nonexistant() {
        // Test that we load a new Shortcuts instance if no file is found
        assert!(hm_is_eq(
            &load_shortcuts("thisisnotafile.bin").shortcuts,
            &HashMap::new()
        ))
    }

    fn vec_is_eq(v1: &Vec<String>, v2: &Vec<String>) -> bool {
        let matching = v1
            .iter()
            .zip(v2.iter())
            .filter(|&(v1, v2)| v1 == v2)
            .count();
        matching == v1.len() && matching == v2.len()
    }

    fn hm_is_eq(hm1: &HashMap<String, String>, hm2: &HashMap<String, String>) -> bool {
        let matching = hm1
            .iter()
            .zip(hm2.iter())
            .filter(|&(hm1, hm2)| hm1.0 == hm2.0 && hm1.1 == hm2.1)
            .count();
        matching == hm1.len() && matching == hm2.len()
    }
}
