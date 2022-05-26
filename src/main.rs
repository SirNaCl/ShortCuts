extern crate savefile;
use savefile::prelude::*;
use std::collections::HashMap;
use std::env;
use std::process::exit;

// MACROS:
#[macro_use]
extern crate savefile_derive;
// STRUCTS:
#[derive(Savefile)]
struct ShortCuts {
    shortcuts: HashMap<String, String>,
}

// TYPES:
type Operation = fn(&mut ShortCuts, Vec<String>);

// FUNCTIONS:
fn parse_args(a: &Vec<String>) -> Vec<String> {
    if a.len() < 2 {
        eprintln!("Not enough arguments!");
        exit(1);
    }
    let mut newvec = a.clone();
    newvec.remove(0);
    return newvec;
}

fn save_shortcuts(sc: &ShortCuts, p: &str) {
    // TODO: Save and load from executable dir and not from running dir
    save_file(p, 0, sc).unwrap();
}

fn load_shortcuts(p: &str) -> ShortCuts {
    load_file(p, 0).unwrap_or(ShortCuts {
        shortcuts: HashMap::new(),
    })
}

fn operation_err(sc: &mut ShortCuts, args: Vec<String>) {
    println!("Invalid operation!");
    exit(1);
}

fn list_shortcuts(sc: &mut ShortCuts, _args: Vec<String>) {
    if sc.shortcuts.len() == 0 {
        println!("No shortcuts!");
        return;
    }

    println!("Your shortcuts:");

    for (k, v) in &sc.shortcuts {
        println!("{} : {}", k, v);
    }
}

fn run_shortcut(sc: &mut ShortCuts, args: Vec<String>) {
    todo!()
}

fn open_shortcut(sc: &mut ShortCuts, args: Vec<String>) {
    todo!()
}

fn get_shortcut(sc: &mut ShortCuts, args: Vec<String>) {
    todo!() // Print out the path, should work with pipe
}

fn del_shortcut(sc: &mut ShortCuts, args: Vec<String>) {
    if args.len() < 1 {
        println!("The del operator needs at least 1 shortcut as argument...");
        exit(1);
    }
    for a in args {
        match sc.shortcuts.remove(&a) {
            Some(_v) => println!("Removed shortcut \"{}\"", a),
            None => println!("Couldn't find shortcut \"{}\"", a),
        }
    }
}

fn add_shortcut(sc: &mut ShortCuts, args: Vec<String>) {
    //TODO: Check if the given path points to dir or file, first relative, then absolute - add full absolute path to shortcut
    if args.len() != 2 {
        println!(
            "Wrong number of args for add, 2 required but {} given",
            args.len()
        );
        exit(1);
    }
    sc.shortcuts.insert(args[0].clone(), args[1].clone());
}

fn get_operation(o_name: &str) -> Option<Operation> {
    match o_name {
        "add" => Some(add_shortcut),
        "del" => Some(del_shortcut),
        "list" => Some(list_shortcuts),
        "run" => Some(run_shortcut),
        "open" => Some(open_shortcut),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed = parse_args(&args);
    let op = parsed.remove(0);
    let mut sc = load_shortcuts("save.bin");
    let f: Operation;

    match get_operation(&op) {
        Some(o) => f = o,
        None => f = operation_err,
    }

    f(&mut sc, parsed);
    save_shortcuts(&sc, "save.bin");
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::PathBuf, process::exit};

    use crate::{
        add_shortcut, del_shortcut, get_operation, list_shortcuts, load_shortcuts, open_shortcut,
        parse_args, run_shortcut, save_shortcuts, ShortCuts,
    };

    #[test]
    fn test_parse_args() {
        let arg = vec!["prog".to_string(), "1".to_string(), "2".to_string()];
        let res = parse_args(&arg);
        assert!(vec_is_eq(&res, &vec!["1".to_string(), "2".to_string()]))
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

    #[test]
    fn test_get_operations() {
        assert_eq!(
            get_operation("add").unwrap() as usize,
            add_shortcut as usize
        );
        assert_eq!(
            get_operation("del").unwrap() as usize,
            del_shortcut as usize
        );
        assert_eq!(
            get_operation("list").unwrap() as usize,
            list_shortcuts as usize
        );
        assert_eq!(
            get_operation("open").unwrap() as usize,
            open_shortcut as usize
        );
        assert_eq!(
            get_operation("run").unwrap() as usize,
            run_shortcut as usize
        );
        assert!(get_operation("nonsenseOP").is_none());
    }
    #[test]
    fn test_open_shortcut() {
        let mut sc = ShortCuts {
            shortcuts: HashMap::new(),
        };
        let s_path = match std::env::current_dir() {
            Ok(p) => p,
            Err(e) => exit(1),
        };
        sc.shortcuts.insert("root".to_string(), "/".to_string());
        open_shortcut(&mut sc, vec!["root".to_string()]);
        let r_path = match std::env::current_dir() {
            Ok(p) => p,
            Err(e) => exit(1),
        };

        assert_ne!(r_path, s_path);
        let p = PathBuf::from(r"/");

        assert_eq!(r_path, p);
    }

    #[test]
    fn test_del_shortcut() {
        let mut sc = ShortCuts {
            shortcuts: HashMap::new(),
        };
        let hm = HashMap::new();
        sc.shortcuts.insert("home".to_string(), "~".to_string());
        del_shortcut(&mut sc, vec!["home".to_string()]);

        assert!(hm_is_eq(&hm, &sc.shortcuts));
    }

    #[test]
    fn test_add_shortcut() {
        let mut sc = ShortCuts {
            shortcuts: HashMap::new(),
        };
        let mut hm = HashMap::new();
        add_shortcut(&mut sc, vec!["home".to_string(), "~".to_string()]);
        hm.insert("home".to_string(), "~".to_string());

        assert!(hm_is_eq(&hm, &sc.shortcuts));
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
