module main
import os


fn main() {
	if os.args.len < 2 {
		println("giter: commmand-line helper for git - version 0.1.0-dev")
		println("Usage: giter [command] --<args>")
	}
	else {
		match(os.args[1]) {
			
		}
	}
}

