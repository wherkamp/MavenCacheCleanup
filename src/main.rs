use clap::{App, Arg};
use std::path::{PathBuf};
use std::fs::remove_dir_all;

fn main() {
    let matches = App::new("Maven Cache Cleanup").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Removes a depend from the Gradle or Maven Cache").
        arg(Arg::with_name("group").short("g").long("group").value_name("groupID").help("Group ID for the Depend").takes_value(true).required(true)).
        arg(Arg::with_name("artifact").short("a").long("artifact").value_name("artifactID").help("Artifact ID if you want to be more specific").takes_value(true).required(false)).
        get_matches();
    delete_gradle_cache(String::from(matches.value_of("group").unwrap()), String::from(matches.value_of("artifact").or(Option::from("")).unwrap()));
    delete_maven_cache(String::from(matches.value_of("group").unwrap()),String::from( matches.value_of("artifact").or(Option::from("")).unwrap()));
}

fn delete_gradle_cache(group_id: String, artifact_id: String) {
    let mut buf = get_gradle_folder().join(&group_id);
    if !artifact_id.is_empty() {
        buf = buf.join(&artifact_id);
    }
    remove_dir_all(&buf).unwrap();
    println!("Deleted {}", buf.to_str().unwrap());
}

fn delete_maven_cache(group_id: String, artifact_id: String) {
    let mut buf = get_maven_folder();
    for x in group_id.split(".") {
        buf = buf.join(x);
    }
    if !artifact_id.is_empty() {
        buf = buf.join(&artifact_id);
    }
    remove_dir_all(&buf).unwrap();
    println!("Deleted {}", buf.to_str().unwrap());
}

pub fn get_maven_folder() -> PathBuf {
    if cfg!(windows) {
        // Use %appdata% if in windows.
        return dirs::home_dir().unwrap().join(".m2").join("repository");
    } else {
        // Else attempt to find the home environment variable.
        return dirs::home_dir().unwrap().join(".m2").join("repository");
    }
}

pub fn get_gradle_folder() -> PathBuf {
    if cfg!(windows) {
        // Use %appdata% if in windows.
        return dirs::home_dir().unwrap().join(".gradle").join("caches").join("modules-2").join("files-2.1");
    } else {
        return dirs::home_dir().unwrap().join(".gradle").join("caches").join("modules-2").join("files-2.1");
    }
}
