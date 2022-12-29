use md5::Md5;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::{collections::HashMap, fs, io};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
struct Version {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "packageUrl")]
    package_url: String,
    #[serde(rename = "remoteManifestUrl")]
    remote_manifest_url: String,
    #[serde(rename = "remoteVersionUrl")]
    remote_version_url: String,
}

#[derive(Serialize, Deserialize)]
struct Project {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "packageUrl")]
    package_url: String,
    #[serde(rename = "remoteManifestUrl")]
    remote_manifest_url: String,
    #[serde(rename = "remoteVersionUrl")]
    remote_version_url: String,
    #[serde(rename = "assets")]
    assets: HashMap<String, Assets>,
    #[serde(rename = "searchPaths")]
    search_paths: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Project256 {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "packageUrl")]
    package_url: String,
    #[serde(rename = "remoteManifestUrl")]
    remote_manifest_url: String,
    #[serde(rename = "remoteVersionUrl")]
    remote_version_url: String,
    #[serde(rename = "assets")]
    assets: HashMap<String, Assets256>,
    #[serde(rename = "searchPaths")]
    search_paths: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Assets {
    // name: String,
    md5: String,
    size: u64,
}
#[derive(Serialize, Deserialize)]
struct Assets256 {
    // name: String,
    md5: String,
    size: u64,
    sha256: String,
}
const PROJECT_PATH: &str = "/Users/wuhao/Projects/Cocos/ludo-baloot-club/build/jsb-link/";
//assets
fn main() -> std::io::Result<()> {
    let path = format!("{}assets", PROJECT_PATH);
    let json_path = format!("{}json.json", PROJECT_PATH);
    println!("Hello, world!:{path}");
    let assets: HashMap<String, Assets256> = HashMap::new();
    let mut project: Project256 = Project256 {
        version: "".to_string(),
        package_url: "".to_string(),
        remote_manifest_url: "".to_string(),
        remote_version_url: "".to_string(),
        assets: assets,
        search_paths: vec![],
    };
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() && !file.path().ends_with(".DS_Store") {
            let mut ffile = fs::File::open(file.path())?;
            let mut hasher = Md5::new();
            let mut sha_hasher = Sha256::new();
            let n = io::copy(&mut ffile, &mut hasher)?;
            io::copy(&mut ffile, &mut sha_hasher)?;
            let hash = hasher.finalize();
            let sha_hash = sha_hasher.finalize();
            project.assets.insert(
                file.path().display().to_string().replace(PROJECT_PATH, ""),
                Assets256 {
                    md5: format!("{:x}", hash),
                    size: n,
                    sha256: format!("{:x}", sha_hash),
                },
            );
        }
    }
    let json = serde_json::to_string_pretty(&project).unwrap();
    println!("{}", project.assets.capacity());
    // save as json file

    let mut file = fs::File::create(json_path)?;
    io::Write::write_all(&mut file, json.as_bytes())?;
    Ok(())
}

#[warn(dead_code)]
fn fun_name(json_path: String) -> Result<Project, io::Error> {
    let mut file = File::open(json_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let project: Project = serde_json::from_str(&contents).unwrap();
    println!("{}", project.assets.capacity());
    Ok(project)
}
#[test]
fn test_json() -> std::io::Result<()> {
    let json_path = format!("{}json.json", PROJECT_PATH);
    let json_project = fun_name(json_path)?;
    let new_json_path=format!("/Users/wuhao/Projects/Cocos/ludo-baloot-club/packages-hot-update/manifest/project.manifest");
    let new_json_project = fun_name(new_json_path)?;
    for (key, value) in new_json_project.assets.iter() {
        if json_project.assets.contains_key(key) {
            if json_project.assets.get(key).unwrap().md5 != value.md5 {
                println!("{}:{}", key, value.md5);
            }
        } else {
            println!("{}:{}", key, value.md5);
        }
    }
    Ok(())
}
