use std::env::join_paths;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;

const COL_DIR: &str = ".col";
const ROW_DIR: &str = ".row";

// this function will initialize or use the current
// directory for the database if it is already initialized
pub fn new_store(_location: &str) -> ReMem {
    // validate given directory
    if !file_exists(_location) {
        // if the directory does not exists we are going to
        // attempt to initialize the directory

        // init root dir
        let init_res = create_dir_all(_location);
        if init_res.is_err() {
            panic!("There was an error initializing the directory {:?}", init_res.err())
        }

        // init col dir
        let init_res = create_dir_all(get_col_dir(_location));
    }
    return ReMem { _root_location: String::from(_location) };
}


pub struct ReMem {
    _root_location: String,
}

impl ReMem {
    pub fn print_location(&self) {
        println!("{}", self._root_location)
    }
}

fn file_exists(_location: &str) -> bool {
    Path::new(_location).exists()
}

// return col directory from root directory
// going to assume forward slash and path separator
// for now.
// TODO make path separator based on OS
fn get_col_dir(_location: &str) -> &str {
    _location + "/" + COL_DIR
}