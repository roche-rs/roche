use remove_dir_all::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::*;

#[cfg(test)]

static CNT: AtomicUsize = AtomicUsize::new(0);
thread_local!(static IDX: usize = CNT.fetch_add(1, Ordering::SeqCst));

#[test]
fn generate_release_function_file_only() {
    let path = root("generate_release_function_file_only");
    if Path::new(&path).exists() {
        remove_dir_all(&path).unwrap();
    }
    fs::create_dir_all(&path).expect(&format!("couldn't create {:?} directory", path.display()));
    assert!(env::set_current_dir(&path).is_ok());

    let mut me = env::current_exe().expect("couldn't find current exe");
    me.pop(); // chop off exe name
    me.pop(); // chop off `deps`
    me.push("roche");

    let mut init_cmd = Command::new(&me)
        .arg("init")
        .current_dir(&path)
        .spawn()
        .unwrap();
    assert!(init_cmd.wait().is_ok());

    let function_file = format!("{}/{}", path.display(), "functions.rs");
    println!("{}", function_file);
    assert!(Path::new(&function_file).exists());
    let mut build_cmd = Command::new(&me)
        .arg("release")
        .arg("-t")
        .arg("generates_release_function_file_only")
        .current_dir(&path)
        .spawn()
        .unwrap();

    assert!(build_cmd.wait().is_ok());

    remove_dir_all(path).unwrap();
}

#[test]
fn generate_release_project() {
    let path = root("generate_release_project");
    if Path::new(&path).exists() {
        remove_dir_all(&path).unwrap();
    }
    fs::create_dir_all(&path).expect(&format!("couldn't create {:?} directory", path.display()));
    assert!(env::set_current_dir(&path).is_ok());

    let mut me = env::current_exe().expect("couldn't find current exe");
    me.pop();
    me.pop();
    me.push("roche");

    let mut init_cmd = Command::new(&me)
        .arg("init")
        .arg("default")
        .arg("-n")
        .arg("generate_release_project")
        .current_dir(&path)
        .spawn()
        .unwrap();
    assert!(init_cmd.wait().is_ok());

    let function_folder = format!("{}/{}", path.display(), "generate-release-project");
    println!("{}", function_folder);
    assert!(env::set_current_dir(&function_folder).is_ok());

    //assert!(Path::new(&function_file).exists());

    let mut build_cmd = Command::new(&me)
        .arg("release")
        .arg("-t")
        .arg("generate_release_project")
        .current_dir(&function_folder)
        .spawn()
        .unwrap();

    assert!(build_cmd.wait().is_ok());

    remove_dir_all(path).unwrap();
}

#[test]
fn generate_release_project_no_tag() {
    let path = root("generate_release_project_no_tag");
    if Path::new(&path).exists() {
        remove_dir_all(&path).unwrap();
    }
    fs::create_dir_all(&path).expect(&format!("couldn't create {:?} directory", path.display()));
    assert!(env::set_current_dir(&path).is_ok());

    let mut me = env::current_exe().expect("couldn't find current exe");
    me.pop();
    me.pop();
    me.push("roche");

    let mut init_cmd = Command::new(&me)
        .arg("init")
        .arg("default")
        .arg("-n")
        .arg("generate_release_project_no_tag")
        .current_dir(&path)
        .spawn()
        .unwrap();
    assert!(init_cmd.wait().is_ok());

    let function_folder = format!("{}/{}", path.display(), "generate-release-project-no-tag");
    println!("{}", function_folder);
    assert!(env::set_current_dir(&function_folder).is_ok());

    //assert!(Path::new(&function_file).exists());

    let mut build_cmd = Command::new(&me)
        .arg("release")
        .current_dir(&function_folder)
        .spawn()
        .unwrap();

    assert!(build_cmd.wait().is_ok());

    remove_dir_all(path).unwrap();
}

fn root(name: &str) -> PathBuf {
    let idx = IDX.with(|x| *x);

    let mut me = env::current_exe().expect("couldn't find current exe");
    me.pop(); // chop off exe name
    me.pop(); // chop off `deps`
    me.pop(); // chop off `debug` / `release`
    me.push("generated-tests");
    me.push(&format!("test-{}-{}", idx, name));
    return me;
}
