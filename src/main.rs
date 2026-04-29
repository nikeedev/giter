use std::process::Command;
use std::{env, process::exit};

fn run(command: &str, args: &str) {
    // println!("{} {}", command, args);
    let output = Command::new(command)
        .args(args.split("@1"))
        .output()
        .expect("failed to execute process");

    // println!("{:#?}", Command::new(command).args(args.split("@1")));

    if output.status.code().unwrap() == 0 {
        println!("Done!");
    } else {
        println!(
            "Error doing it (error code: {}), please run git commands on your own to debug the problem",
            output.status.code().unwrap()
        );
        println!("error: {}", String::from_utf8(output.stderr).unwrap().as_str());
        println!("info:  {}", String::from_utf8(output.stdout).unwrap().as_str());
    }
}

fn main() {
    let help_text = "\nCommands:\nup - does `git commit -m \"updates\"`, or custom message if passed after the \"up\" command\naddl - stage all files to git\ndall - nikee's setup, stage files, commit, pull, push\npush - push\npull - pull";

    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("giter: commmand-line helper for git - version 1.2.1 - by nikeedev");
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
                }

                // println!("git {}", format!("commit -m '{}'", message).as_str());
                run("git", format!("commit@1-m@1'{}'", message).as_str());
            } else {
                run("git", "commit -m 'update'");
            }
        }

        "dall" => { 
            println!("\"dall'ing\"...");

            run("git", "add@1-A");
            run("git", "commit@1-m@1'update'");
            run("git", "pull");
            run("git", "push");
        }

        "addl" => {
            println!("Adding all files...");
            run("git", "add@1-A");
        }

        "push" => {
            println!("Pushing...");
            run("git", "push");
        }
        
        "pull" => {
            println!("Pulling...");
            run("git", "pull");
        }
        
        _ => {
            println!("giter: {} not recognized as a command", args[1]);
            println!("Usage: giter [command] [arguments]");
            println!("{}", help_text);
            exit(1);
        }
    }
}
