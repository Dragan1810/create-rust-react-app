use std::env::{args, current_dir};
use std::path::Path;
use std::process::Command;
use std::str;

use async_std::fs;
//use async_std::io;
use async_std::prelude::*;
use async_std::task;

fn read_args() -> String {
    // 1. check for node.js
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "node --version"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("node --version")
            .output()
            .expect("failed to execute process")
    };
    let node_version = str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .split('.')
        .take(1)
        .fold("", |_acc, x| x)
        .chars()
        .fold("".to_owned(), |mut acc, x| match x.is_digit(10) {
            true => {
                acc.push_str(&x.to_string());
                acc
            }
            false => acc,
        })
        .parse::<i32>()
        .unwrap();

    if node_version < 12 {
        println!(
            "Unsupported Node.js version update to v12 or higher {:?}",
            node_version
        );
    }

    let project_name: String = args().skip(1).take(1).collect();

    project_name
}

async fn init_package_json(project_name: &str) {
    let curr_path = current_dir().unwrap();
    let dir_path = curr_path.join(project_name);
    dbg!(&dir_path);
    fs::create_dir(Path::new(&dir_path))
        .await
        .expect("Can't create Folder");

    let mut package_json = fs::File::create(Path::new(&dir_path.join("package.json")))
        .await
        .expect("Can't create File");

    package_json
        .write_all(
            &format!(
                r#"{{
            "name": "{project_name}",
            "version": "0.1.0",
            "keywords": [
              "react", "rust"
            ],
            "description": "Create React apps with no build configuration.",
            "dependencies": {{
              "react": "16.12.0"
              "react-dom": 16.12.0",
              "@dragan1810/react-scripts" : "0.1.0"
            }}
          }}"#,
                project_name = project_name
            )
            .into_bytes(),
        )
        .await
        .expect("Can't write to pkg.json");
}

fn main() {
    let project_name = read_args();
    task::block_on(init_package_json(&project_name));
}
