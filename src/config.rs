use crate::utils;
use apt_cache::apt::AptError;
use apt_cache::Package;
use glob::glob;
use serde::Deserialize;
use std::fs::create_dir;
use std::path::Path;

#[derive(Deserialize)]
pub struct PackageConfig {
    pub name: String,
    install: Option<bool>,
    build_depends: Option<bool>,
    build_recommends: Option<bool>,
}

impl PackageConfig {
    pub fn new(name: &str, install: bool, build_depends: bool, build_recommends: bool) -> Self {
        PackageConfig {
            name: name.to_string(),
            install: Some(install),
            build_recommends: Some(build_depends),
            build_depends: Some(build_recommends),
        }
    }
    pub fn as_package(&self) -> Result<Package, AptError> {
        Package::new(&self.name)
    }

    pub fn install(&self) -> bool {
        self.install.unwrap_or(false)
    }

    pub fn build_depends(&self) -> bool {
        self.build_depends.unwrap_or(false)
    }

    pub fn build_recommends(&self) -> bool {
        self.build_recommends.unwrap_or(false)
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub build_dir: String,
    #[serde(rename = "package")]
    packages: Vec<PackageConfig>,
}

impl Config {
    pub fn get_package(&self, name: &str) -> Option<&PackageConfig> {
        self.packages.iter().find(|p| p.name == name)
    }

    pub fn build(&self, pkg: &PackageConfig) {
        let dest = Path::new(self.build_dir.as_str());
        if !dest.exists() {
            create_dir(dest).expect("Unable to create build directory");
        }

        utils::get_source(pkg.name.as_str(), dest).expect("Unable to get sources");

        let src = glob(format!("{}/{}*/", dest.to_str().unwrap(), pkg.name).as_str())
            .unwrap()
            .filter_map(Result::ok)
            .next()
            .expect("Unable to find sources directory");
        utils::mk_deps(src.as_path()).expect("Unable to make build deps");
        utils::debuild(src.as_path()).expect("Unable to build");
        utils::clean_deps(pkg.name.as_str()).expect("Unable to clean build deps");

        if pkg.install() {
            utils::apt_install(pkg.name.as_str()).expect("Failed to install");

            if pkg.build_depends() {
                self.build_package_depends(pkg);
            }

            if pkg.build_recommends() {
                self.build_package_recommends(pkg);
            }
        }

        if !pkg.install() && (pkg.build_depends() || pkg.build_recommends()) {
            panic!(
                "The \"build_depends\" and \"build_recommends\" options require the \"install\" option to be set"
            )
        }
    }

    pub fn build_package_depends(&self, pkg: &PackageConfig) {
        if let Some(depends) = pkg.as_package().expect("Package not in repos").depends() {
            for package in depends {
                if Package::new(&package.name).is_err() {
                    self.build(&PackageConfig::new(
                        package.name.as_str(),
                        pkg.install(),
                        pkg.build_depends(),
                        pkg.build_recommends(),
                    ));
                }
            }
        }
    }

    pub fn build_package_recommends(&self, pkg: &PackageConfig) {
        if let Some(recommends) = pkg.as_package().expect("Package not in repos").recommends() {
            for package in recommends {
                if Package::new(&package.name).is_err() {
                    self.build(&PackageConfig::new(
                        package.name.as_str(),
                        pkg.install(),
                        pkg.build_depends(),
                        pkg.build_recommends(),
                    ));
                }
            }
        }
    }

    pub fn build_all(&self) {
        for package in &self.packages {
            self.build(&package);
        }
    }
}
