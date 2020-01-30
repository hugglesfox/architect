use crate::utils::*;
use git2::{ErrorCode, Repository};
use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize)]
pub struct Repo {
    name: String,
    url: String,
    tag: String,
}

impl Repo {
    /// Clones the repo to the path `$repo_dir/$name`. If a git repository already exists at that
    /// path, that is used.
    pub fn repo_clone(&self, dest: &str) -> PathBuf {
        // Clone/initalise
        let path = Path::new(dest).join(self.name.as_str());
        let _repo = match Repository::clone(self.url.as_str(), &path) {
            Ok(repo) => repo,
            Err(e) => match e.code() {
                ErrorCode::Exists => Repository::open(&path).unwrap(),
                _ => panic!("{}", e),
            },
        };

        // HACK
        Command::new("git")
            .arg("checkout")
            .arg(&self.tag)
            .current_dir(&path)
            .output()
            .expect("git debuild returned non 0 exit code");

        path

        // FIXME: If anyone can get the tag checkout code working, please do!

        // // Fetch
        // repo.find_remote("origin")
        //     .expect("Unable to find the remote origin")
        //     .fetch(&["refs/tags/*:refs/tags/*"], None, None)
        //     .unwrap();
        //
        // // Checkout tag
        // for reference in repo.references().unwrap() {
        //     let reference = reference.unwrap();
        //
        //     if reference.is_tag() {
        //         let tag_obj = reference.peel(ObjectType::Tag).unwrap();
        //         if tag_obj.as_tag().unwrap().name().unwrap() == self.tag.as_str() {
        //             repo.checkout_tree(&tag_obj, None)
        //                 .expect("Unable to checkout tag");
        //         }
        //     }
        //}
    }
}

#[derive(Deserialize)]
pub struct Config {
    repo_dir: String,
    #[serde(rename = "repo")]
    repos: Vec<Repo>,
}

impl Config {
    pub fn get_repo(&self, name: &str) -> Result<&Repo, &'static str> {
        for repo in self.repos.iter() {
            if repo.name == name {
                return Ok(repo);
            }
        }

        Err("Unable to find repo")
    }

    pub fn build(&self, repo: &Repo) {
        let dest = repo.repo_clone(self.repo_dir.as_str());

        mk_deps(dest.as_path()).expect("mk-build-deps returned non 0 exit code");
        debuild(dest.as_path()).expect("debuild returned non 0 exit code");
        clean_deps(repo.name.as_str()).expect("apt returned non 0 exit code");
    }

    pub fn build_all(&self) {
        for repo in self.repos.iter() {
            self.build(repo);
        }
    }
}
