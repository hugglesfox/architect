use crate::utils;
use apt_cache::Package;
use glob::glob;
use serde::Deserialize;
use std::fs::create_dir;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub build_dir: String,
    #[serde(rename = "package")]
    packages: Vec<Package>,
}

impl Config {
    pub fn get_package(&self, name: &str) -> Option<&Package> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn build(&self, pkg: &Package) {
        let dest = Path::new(self.build_dir.as_str());
        if !dest.exists() {
            create_dir(dest).expect("Unable to create build directory");
        }
        pkg.get_source(dest).expect("Unable to get sources");

        let src = glob(format!("{}/{}*/", dest.to_str().unwrap(), pkg.name).as_str())
            .unwrap()
            .filter_map(Result::ok)
            .next()
            .expect("Unable to find sources directory");
        utils::mk_deps(src.as_path()).expect("Unable to make build deps");
        utils::debuild(src.as_path()).expect("Unable to build");
        utils::clean_deps(pkg.name.as_str()).expect("Unable to clean build deps");
    }

    pub fn build_all(&self) {
        for package in &self.packages {
            self.build(&package);
        }
    }
}
