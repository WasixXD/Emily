use clap::Parser;
use std::fs;
use std::path::PathBuf;
use nu_ansi_term::Color::{Green, Blue, Red};
use rand::Rng;
use serde::{Deserialize, Serialize};

use reqwest;
use tokio;
use clap;

const FILES_TO_IGNORE: [&str;10] = [".lock", ".json", ".config", ".jpg", ".png", ".jpeg", ".mov", ".mp3", ".gif", ".pdf"];
const DIRECTORY_TO_IGNORE: [&str;3] = ["node_modules", "build", "target"];
const EMOJIS: [&str;13] = ["🎉", "🎨", "⚡️", "🔥", "🐛", "🚑️", "✨", "🚀", "🔨", "👽️", "🚩", "🏗️", "💥"];


fn scan_dir(dir_name: &str) -> Vec<PathBuf> {
    let dir_list = fs::read_dir(dir_name).unwrap();
    let mut files_path = Vec::new();
   
    let dir_len = dir_name.len() + 1;


    for paths in dir_list {
        let path = paths.unwrap().path();
        let path_display = path.display().to_string();
        
        //Dont check .files (.gitignore)
        if &path_display[dir_len..dir_len + 1] != "." {
            let current_directory: Vec<_> = path_display.split("/").collect();
            
            //recursive way to get file for each subdirectory and checking if is not a prohibited directory
            if !path_display.contains(".") && !DIRECTORY_TO_IGNORE.contains(&current_directory[current_directory.len() - 1])  {
                
                for i in scan_dir(&path_display) {
                    files_path.push(i);
                }
                
            } else if path_display.contains(".") {
                
                let file_type = &path_display[path_display.find(".").unwrap()..];
                // Ignore some files
                if !FILES_TO_IGNORE.contains(&file_type) {
                    files_path.push(path);
                }
                
            }
                
            
        } 
    
    }

    return files_path
    
}


fn remove_non_alphanumeric(string: &str) -> String {
    let mut new_str = String::from("");
    for i in string.chars() {
        if i.is_alphabetic() || i == ' ' {
            
            new_str.push(i);
            
        }
    }
   
    return new_str.to_string()
} 
//#TODO Emily todo test 

fn get_todos(files: &Vec<PathBuf>) -> Vec<String> {
    let mut todos = Vec::new();
    for file in files {
        let data = fs::read_to_string(file).expect("Couldn't get to file");
        
       
        if data.contains("#TODO") {
            //MATCH all #todos in file
            let index_of_todos: Vec<_> = data.match_indices("#TODO").collect();
            //get todos
            for i in index_of_todos {
                let line_end = i.0 + &data[i.0..].find('\n').unwrap();
                let todo = &data[i.0..line_end];
               

                todos.push(format!("TODO {} . {}", file.display().to_string(), remove_non_alphanumeric(todo)));
            }
            
        }
    }
    return todos;
}


#[derive(Debug, Serialize, Deserialize)]
struct Issue {
    title: String,
    body: String,
    state: String,
    html_url: String
}


#[derive(Parser, Debug)]

struct Command {
    #[arg(short, long)]
    dir: String,

    #[arg(short, long, default_value_t = String::new())]
    
    name: String,

    #[arg(short, long, default_value_t = String::new())]
    repo: String,

    #[arg(short, long, default_value_t = String::new())]
    key: String,

    #[arg(short, long)]
    push: bool
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Command::parse();

    

    if args.dir != ""  {
        let mut rng = rand::thread_rng();
        let all_files = scan_dir(&args.dir);
        let mut todos = get_todos(&all_files);
        
        
        
        
        if args.push && args.name != "" && args.repo != "" && args.key != "" {
            let client = reqwest::Client::new();

            let repo_issues: Vec<Issue> = client.get(format!("https://api.github.com/repos/{}/{}/issues", args.name, args.repo))
                .header("User-Agent", format!("{}", args.name))
                .header("Authorization", format!("Bearer {}", args.key))
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send().await?.json().await?;
               
           
            //remove duplicated todos
            for i in &repo_issues {
                todos.retain(|x| !x.contains(&i.title));
            }
            
                
         
            if todos.len() > 0 {
                
                for i in todos {
                    let title_body: Vec<_> = i.split(" . ").collect();
                    
        
                    let _res = client.post(format!("https://api.github.com/repos/{}/{}/issues", args.name, args.repo))
                        .header("User-Agent", format!("{}", args.name))
                        .header("Accept", "application/vnd.github+json")
                        .header("Authorization", format!("Bearer {}", args.key))
                        .header("X-GitHub-Api-Version", "2022-11-28")
                        .body(format!("{{ \"title\" : \"{}\", \"body\" : \"{}\"}}", title_body[0], title_body[1]))
                        .send().await?.text().await?;
                        println!("{} -  is up to git {}", Green.paint(i), EMOJIS[rng.gen_range(0..EMOJIS.len())]);
                
                }
            } else {
                for i in &repo_issues {
                    println!("{} / {} is already on git - {} as {} state\n", Green.paint(&i.title),Blue.paint(i.body.trim_end()), i.html_url, Red.paint(&i.state));
                }
            }

       } else if !args.push {
            for (i, item) in todos.iter().enumerate() {
                let title_body: Vec<_> = item.split(" . ").collect();

                println!("{}. {} - {} {}", i + 1, Green.paint(title_body[0]), Blue.paint(title_body[1].trim_end()), EMOJIS[rng.gen_range(0..EMOJIS.len())]);
            }
       } else {
            println!("{}", Red.paint("Please provide your NAME, the REPOSITORY and your acess KEY from github"))
       }
        
    }

   
    Ok(())

}


