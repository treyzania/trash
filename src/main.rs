use std::io;
use std::io::*;
//use std::env;
use std::string::String;
use std::process::*;

fn process_command(stdin: &mut Stdin, cnt: u32) {

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

	println!("try: {}", cnt);

	match cmd.spawn() {
		Ok(_) => {},
		Err(m) => println!("trash: {}", m)
	}

	println!("exited: {}", cmd.status().is_ok());

}

fn main() {

	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	let mut iters = 0;

	loop {

		let prompt = "$ ";

		/*match env::var_os("PS1") {
			Some(v) => {
				prompt = v.to_str().unwrap();
			},
			None => {} // Already okay.
		};*/

		// Write the promt and then process the command.
		match stdout.write(&String::from(prompt).as_bytes()) {
			Ok(_) => {},
			Err(_) => {}
		};

		match stdout.flush() {
			Ok(_) => {}
			Err(_) => panic!("could not flush stdout for prompt!")
		};

		process_command(&mut stdin, iters);
		iters += 1;

	}

}
