use std::io;
use std::path::Path;
use std::process::{Command, Output};

pub fn mk_deps(src: &Path) -> io::Result<Output> {
    Command::new("sudo")
        .arg("mk-build-deps")
        .arg("-i")
        .current_dir(src)
        .output()
}

pub fn debuild(src: &Path) -> io::Result<Output> {
    Command::new("debuild")
        .arg("-us")
        .arg("-uc")
        .current_dir(src)
        .output()
}

pub fn clean_deps(pkg: &str) -> io::Result<Output> {
    Command::new("sudo")
        .arg("apt")
        .arg("autoremove")
        .arg(format!("{}-build-deps", pkg).as_str())
        .output()
}

pub fn get_source<P: AsRef<Path>>(pkg: &str, dest: P) -> io::Result<Output> {
    Command::new("apt")
        .arg("source")
        .arg(pkg)
        .current_dir(dest)
        .output()
}

pub fn apt_install(pkg: &str) -> io::Result<Output> {
    Command::new("sudo")
        .arg("apt-get")
        .arg("-y")
        .arg("install")
        .arg(pkg)
        .output()
}
