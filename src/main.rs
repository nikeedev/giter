use std::process::Command;
use std::{env, process::exit};

fn run(command: &str, args: Vec<&str>) {
    // println!("{} {:?}", command, args);
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("failed to execute process");
    
    // println!("{:#?}", Command::new(command).args(args.split("@1")));

    if output.status.code().unwrap() == 0 {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    } else {
        println!(
            "Error doing it (error code: {}), please run git commands on your own to debug the problem",
            output.status.code().unwrap()
        );
        println!("error: {}", String::from_utf8(output.stderr).unwrap().as_str());
        println!("info:  {}", String::from_utf8(output.stdout).unwrap().as_str());
        panic!();
    }
}

fn main() {
    let help_text = "\nCommands:
        up - does `git commit -m \"up\"`, or custom message if passed after the \"up\" command
        addl - stage all files to git
        dall - nikee's setup, stage files, commit, pull, push
        push - push
        pull - pull
        status - show git status
        ";

    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("giter: commmand-line helper for git - version 1.3.0 - by nikeedev");
        println!("Usage: giter [command] [arguments]");
        println!("{}", help_text);
        exit(0);
    }
    match args[1].as_str() {
        "up" => {
            println!("Commiting...");
            if args.len() > 2 {
                let mut message = "".to_string();
                
                for i in args.iter().skip(2) {
                    message.push_str(i.as_str());
                    message.push_str(" ");
                }

                // println!("git {}", format!("commit -m '{}'", message).as_str());
                if !(message.starts_with("\"") && message.ends_with("\"")) || (message.ends_with("'") && message.ends_with("'")) {
                    run("git", vec!["commit", "-m", message.as_str()]);
                } else {
                    run("git", vec!["commit", "-m", format!("\"{}\"", message).as_str()]);
                }
            } else {
                run("git", vec!["commit", "-m", "'up'"]);
            }
        }

        "dall" => { 
            println!("dall'ing...");

            run("git", vec!["add", "-A"]);
            run("git", vec!["commit", "-m", "'up'"]);
            run("git", vec!["pull"]);
            run("git", vec!["push"]);
            println!("Done!");
        }

        "addl" => {
            println!("Adding all files...");
            run("git", vec!["add", "-A"]);
            println!("Done!"); 
        }

        "push" => {
            println!("Pushing...");
            run("git", vec!["push"]);
            println!("Done!");
        }
        
        "pull" => {
            println!("Pulling...");
            run("git", vec!["pull"]);
            println!("Done!");
        }
        
        "status" => {
            run("git", vec!["status"]);
        }
        
        _ => {
            println!("giter: {} not recognized as a command", args[1]);
            println!("Usage: giter [command] [arguments]");
            println!("{}", help_text);
            exit(0);
        }
    }
}
