use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    let mut echo_hello = Command::new("echo");

    echo_hello.arg("-n");
    echo_hello.arg("Hello");
    echo_hello.arg(format!("{}!", args.name));

    let hello_1 = echo_hello.output().expect("failed to execute process");
    let output = match std::str::from_utf8(&hello_1.stdout) {
	Ok(i) => i,
	Err(_e) => "Error!"
    };

    for _ in 0..args.count {
	println!("{output}");
    }
}
