use std::fs::create_dir_all;
use std::path::Path;

const COL_DIR: &str = ".col";
const ROW_DIR: &str = ".row";

// this function will initialize or use the current
// directory for the database if it is already initialized
pub fn new_store(_location: &str) -> ReMem {
    // create root dir if needed
    create_dir_if_not_exists(_location);
    return ReMem { _root_location: String::from(_location) };
}


// this function returns a collection that can be
// used for reading, writing, and querying objects of
// any types
// if the collection directory does not exist it will
// this function also initializes the .col and .row directories
fn new_collection(_location: &str, _name: &str) -> Col {
    create_dir_if_not_exists(_location);
    // create col dir if not exists
    create_dir_if_not_exists(&join_paths(&[_location, COL_DIR]));
    // create row dir if not exists
    create_dir_if_not_exists(&join_paths(&[_location, ROW_DIR]));
    return Col {
        _root_location: String::from(_location),
        _collect_name: String::from(_name),
    };
}


// ReMem root struct
pub struct ReMem {
    _root_location: String,
}

pub struct Col {
    _root_location: String,
    _collect_name: String,
}

impl ReMem {
    pub fn print_location(&self) {
        println!("{}", self._root_location)
    }

    pub fn get_collection(&self, col_name: &str) -> Col {
        // creating a directory per collection
        // /root/location + "/" + col_name
        let new_col_path = join_paths(&[&self._root_location, col_name]);
        return new_collection(&new_col_path, col_name);
    }
}

fn file_exists(_location: &str) -> bool {
    Path::new(_location).exists()
}

fn create_dir_if_not_exists(_location: &str) {
    if !file_exists(_location) {
        let res = create_dir_all(_location);
        if res.is_err() {
            panic!("unable to create directory {:?}", res.err())
        }
    }
}


pub fn join_paths(args: &[&str]) -> String {
    let args_length = args.len();
    if args_length == 0 {
        return String::from("");
    } else if args_length == 1 {
        return String::from(args[0]);
    }


    let mut url = args[0].to_owned();
    let sep = "/";
    for idx in 1..args_length {
        url.push_str(sep);
        url.push_str(args[idx]);
    }
    return String::from(url);
}