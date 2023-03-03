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
			"addl" {
				run("git add -A")
			}
			else {
				println("giter: commmand-line helper for git - version 0.1.0-dev")
				println("Usage: giter [command] [arguments]")
			}
		}
	}
}

