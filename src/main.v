module main
import os

fn run(command string) {
	mut code := os.execute(command)

	if code.exit_code == 0 {
		println("Done!")
	}
	else {
		println(code.output)
	}
}

fn main() {
	if os.args.len < 2 {
		println("giter: commmand-line helper for git - version 0.1.0-dev")
		println("Usage: giter [command] [arguments]")
		println("\nCommands:\nup - does `git commit -m \"updates\"`, or custom message if passed after up command\nsave - commit using \"save\" comment\naddl - stage all files to git\ndall - nikee's setup, stage files, commit, pull, push\npush - push\npull - pull")
	}
	else {
		match os.args[1] {
			"up" {
				if os.args.len > 2 {
					mut message := ""
					for i := 2; i < os.args.len; i++ {
						message += os.args[i] + " "
					}
					run("git commit -m '${message}'")
				}
				else {
					run("git commit -m 'updates'")
				}
			}
			"dall" {
				run("git add -A")
				run("git commit -m 'updates'")
				run("git pull")
				run("git push")
			}
			"save" {
				run("git commit -m 'save'")
			}
			"addl" {
				run("git add -A")
			}
			"push" {
				run("git push")
			}
			"fetch" {
				run("git pull")
			}
			else {
				println("giter: ${os.args[1]} not recognized as a command")
				println("Usage: giter [command] [arguments]")
			}
		}
	}
}

