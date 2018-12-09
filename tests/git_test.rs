extern crate pug;

use std::path::PathBuf;

#[test]
fn it_upgrade() {
    pug::sys::git::upgrade(
        "https://github.com/saturn-xiv/pug.git",
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tmp")
            .join("pug"),
    )
    .unwrap();
}
