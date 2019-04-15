pub mod re_mem;

fn main() {
    let _store = re_mem::new_store("/Users/samorozco/rust_db");
    let _user_collection = _store.get_collection("users");
    _store.print_location()
}

#[cfg(test)]
mod tests {
    #[test]
    fn join_paths() {
        use super::*;
        let path = re_mem::join_paths(&["test", "test", "test"]);
        assert_eq!(path, "test/test/test");

        let path = re_mem::join_paths(&["test"]);
        assert_eq!(path, "test");

        let path = re_mem::join_paths(&[]);
        assert_eq!(path, "");
    }
}