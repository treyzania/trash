use std::io;
use std::io::*;
use std::env;
use std::string::String;
use std::process::*;

fn process_command(stdin: &mut Stdin) {

	let mut buf = String::new();
	stdin.read_line(&mut buf).expect("error reading args");
	buf.pop();

	let words: Vec<&str> = buf.as_str().split(' ').collect();
	let mut cmd = Command::new(words[0]);
	for i in 1..words.len() {
		cmd.arg(words[i]);
	}

	cmd.stdin(Stdio::inherit());
	cmd.stdout(Stdio::inherit());

	match cmd.spawn() {
		Ok(mut c) => println!("exited: {}", c.wait().is_ok()),
		Err(m) => println!("trash: {}", m)
	}

}

fn main() {

	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	loop {

		let prompt =
			match env::var_os("PS1") {
				Some(v) => v.into_string().unwrap(),
				None => String::from("$ "),
			};

		// Write the promt and then process the command.
		match stdout.write(&String::from(prompt).as_bytes()) {
			Ok(_) => {},
			Err(_) => {}
		};

		stdout.flush().expect("couldn't flush stdout");

		process_command(&mut stdin);

	}

}
