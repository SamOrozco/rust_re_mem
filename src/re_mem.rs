use std::fs::create_dir_all;
use std::io;
use std::path::Path;

const COL_DIR: &str = ".col";
const ROW_DIR: &str = ".row";

// this function will initialize or use the current
// directory for the database if it is already initialized
pub fn new_store(location: &str) -> ReMem {
    // validate given directory
    if !file_exists(location) {
        // if the directory does not exists we are going to
        // attempt to initialize the directory

        let init_res = create_all_directory(location);
        if init_res.is_err() {
            panic!("There was an error initializing the directory {:?}", init_res.err())
        }
    }
    return ReMem { _root_location: String::from(location) };
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

fn create_all_directory(_location: &str) -> io::Result<()> {
    let _is_created = create_dir_all(_location);
    _is_created
}

fn get_col_dir(_location: &str) -> &str {}