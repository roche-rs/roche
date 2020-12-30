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
fn generate_docker_file_default() {
    let path = root("generate_docker_file_default");
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
        .arg("gen")
        .current_dir(&path)
        .spawn()
        .unwrap();
    assert!(init_cmd.wait().is_ok());
    let df = match fs::read_to_string("Dockerfile") {
        Ok(s) => s,
        Err(e) => panic!("Read Dockerfile failed: {}", e),
    };
    assert!(df.contains("quay.io/roche/default:1.0.0"));
    assert!(df.contains("quay.io/roche/alpine:3.12"));

    remove_dir_all(path).unwrap();
}

#[test]
fn generate_docker_file_with_params() {
    let path = root("generate_docker_file_with_params");
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
        .arg("gen")
        .arg("-r")
        .arg("runtime/testcontainer")
        .arg("-b")
        .arg("build/testimage")
        .current_dir(&path)
        .spawn()
        .unwrap();
    assert!(init_cmd.wait().is_ok());
    let df = match fs::read_to_string("Dockerfile") {
        Ok(s) => s,
        Err(e) => panic!("Read Dockerfile failed: {}", e),
    };
    assert!(df.contains("uild/testimage"));
    assert!(df.contains("runtime/testcontainer"));

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
