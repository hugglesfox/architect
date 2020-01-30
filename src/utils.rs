use std::io;
use std::path::Path;
use std::process::{Command, Output};

pub fn mk_deps(src: &Path) -> io::Result<Output> {
    Command::new("mk-build-deps").arg("-i").current_dir(src).output()
}

pub fn debuild(src: &Path) -> io::Result<Output> {
    Command::new("debuild").arg("-us").arg("-uc").current_dir(src).output()
}

pub fn clean_deps(pkg: &str) -> io::Result<Output> {
    Command::new("apt")
        .arg("autoremove")
        .arg(format!("{}-build-deps", pkg).as_str())
        .output()
}
