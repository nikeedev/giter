use std::process::Command;
use std::{env, process::exit};

fn run(command: &str, args: &str) {
    let output = Command::new(command)
        .args(args.split(" "))
        .output()
        .expect("failed to execute process");

    if output.status.code().unwrap() == 0 {
        println!("Done!");
    } else {
        println!(
            "Error doing it (error code: {}), please run git commands on your own to debug the problem",
            output.status.code().unwrap()
        );
        println!("error: {}", String::from_utf8(output.stderr).unwrap().as_str());
        println!("info: {}", String::from_utf8(output.stdout).unwrap().as_str());
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("giter: commmand-line helper for git - version 1.2.0 - by nikeedev");
        println!("Usage: giter [command] [arguments]");
        println!(
            "\nCommands:\nup - does `git commit -m \"updates\"`, or custom message if passed after up command\nsave - commit using \"save\" comment\naddl - stage all files to git\ndall - nikee's setup, stage files, commit, pull, push\npush - push\npull - pull"
        );
        exit(0);
    }
    match args[1].as_str() {
        "up" => {
            if args.len() > 2 {
                let mut message = "".to_string();
                
                for i in args.iter().skip(2) {
                    message.push_str(i.as_str());
                }

                println!("git {:#?}", format!("commit -m \"{}\"", message).as_str());
                run("git", format!("commit -m {}", message).as_str());
            } else {
                run("git", "commit -m 'update'");
            }
        }

        "dall" => {
            run("git", "add -A");
            run("git", "commit -m 'update'");
            run("git", "pull");
            run("git", "push");
        }

        "addl" => {
            run("git", "add -A");
        }

        "push" => {
            run("git", "push");
        }
        
        "pull" => {
            run("git", "pull");
        }
        
        _ => {
            println!("giter: {} not recognized as a command", args[1]);
            println!("Usage: giter [command] [arguments]");
            println!(
                "\nCommands:\nup - does `git commit -m \"updates\"`, or custom message if passed after up command\naddl - stage all files to git\ndall - nikee's setup, stage files, commit, pull, push\npush - push\npull - pull"
            );
            exit(1);
        }
    }
}
