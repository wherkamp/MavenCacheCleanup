use clap::{App, Arg};
use std::path::{PathBuf};
use std::fs::remove_dir_all;

fn main() {
    let mut app = App::new("Maven Cache Cleanup").
        version("1.0-SNAPSHOT").author("Wyatt Jacob Herkamp <wherkamp@kingtux.me>").about("Removes a depend from the Gradle or Maven Cache").
        arg(Arg::with_name("all").long("all").help("Delete all").takes_value(false)).
        arg(Arg::with_name("group").short("g").long("group").value_name("groupID").help("Group ID for the Depend").takes_value(true).required(false)).
        arg(Arg::with_name("artifact").short("a").long("artifact").value_name("artifactID").help("Artifact ID if you want to be more specific").takes_value(true).required(false));
    let matches = app.clone().get_matches();
    if matches.is_present("group") {
        delete_gradle_cache(String::from(matches.value_of("group").unwrap()), String::from(matches.value_of("artifact").or(Option::from("")).unwrap()));
        delete_maven_cache(String::from(matches.value_of("group").unwrap()), String::from(matches.value_of("artifact").or(Option::from("")).unwrap()));
    } else if matches.is_present("all") {
        let gradle = get_gradle_folder();
        if gradle.exists() {
            let result = remove_dir_all(&gradle);
            if result.is_ok() {
                println!("Deleted Gradle {}", gradle.to_str().unwrap());
            } else {
                println!("Failed to Delete Gradle {}", gradle.to_str().unwrap());
            }
        }
        let maven = get_maven_folder();
        if maven.exists() {
            let x = remove_dir_all(&maven);
            if x.is_ok() {
                println!("Deleted Maven {}", maven.to_str().unwrap());
            } else {
                println!("Failed to Delete Maven {}", maven.to_str().unwrap());
            }
        }
    } else {
        app.print_long_help().unwrap();
    }
}

fn delete_gradle_cache(group_id: String, artifact_id: String) {
    let mut buf = get_gradle_folder().join(&group_id);
    if !artifact_id.is_empty() {
        buf = buf.join(&artifact_id);
    }
    if !buf.exists() {
        return;
    }
    let result = remove_dir_all(&buf);
    if result.is_ok() {
        println!("Deleted Gradle {}", buf.to_str().unwrap());
    } else {
        println!("Failed to Delete Gradle {}", buf.to_str().unwrap());
    }
}

fn delete_maven_cache(group_id: String, artifact_id: String) {
    let mut buf = get_maven_folder();
    for x in group_id.split(".") {
        buf = buf.join(x);
    }
    if !artifact_id.is_empty() {
        buf = buf.join(&artifact_id);
    }
    if !buf.exists() {
        return;
    }
    let result = remove_dir_all(&buf);
    if result.is_ok() {
        println!("Deleted Maven {}", buf.to_str().unwrap());
    } else {
        println!("Failed to Delete Maven {}", buf.to_str().unwrap());
    }
}

pub fn get_maven_folder() -> PathBuf {
    return dirs::home_dir().unwrap().join(".m2").join("repository");
}

pub fn get_gradle_folder() -> PathBuf {
    return dirs::home_dir().unwrap().join(".gradle").join("caches").join("modules-2").join("files-2.1");
}
