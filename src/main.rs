pub mod re_mem;

fn main() {
    let _store = re_mem::ReMem::new_store("/Users/samorozco/rust_db");
    _store.print_location()
}