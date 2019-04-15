use std::collections::hash_map::DefaultHasher;
use std::fs::create_dir_all;
use std::hash::{Hash, Hasher};
use std::path::Path;

const COL_DIR: &str = ".col";
const ROW_DIR: &str = ".row";

// this function will initialize or use the current
// directory for the database if it is already initialized
pub fn new_store(_location: &str) -> ReMem {
    // create root dir if needed
    create_dir_if_not_exists(_location);
    return ReMem {
        _root_location: String::from(_location),
    };
}


// ReMem root struct
pub struct ReMem {
    _root_location: String,
}

pub struct Col {
    _collection_root_path: String,
    _collect_name: String,
}

impl ReMem {
    pub fn print_location(&self) {
        println!("{}", self._root_location)
    }

    pub fn get_collection(&self, col_name: &str) -> Col {
        return self.new_collection(&self._root_location, col_name);
    }

    // this function returns a collection that can be
    // used for reading, writing, and querying objects of
    // any types
    // if the collection directory does not exist it will
    // this function also initializes the .col and .row directories
    fn new_collection(&self, _root_path: &str, _name: &str) -> Col {
        let _hash_name = hash_string(_name);
        let _col_path = join_paths(&[_root_path, &_hash_name]);
        create_dir_if_not_exists(&_col_path);
        // create col dir if not exists
        create_dir_if_not_exists(&join_paths(&[&_col_path, COL_DIR]));
        // create row dir if not exists
        create_dir_if_not_exists(&join_paths(&[&_col_path, ROW_DIR]));
        return Col {
            _collection_root_path: _col_path,
            _collect_name: String::from(_name),
        };
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


// universal hash string function
fn hash_string(value: &str) -> String {
    let hash_int = hash_it(value);
    format!("{}", hash_int)
}

fn hash_it<T: Hash>(t: T) -> u64 {
    let mut _hasher = DefaultHasher::new();
    t.hash(&mut _hasher);
    _hasher.finish()
}