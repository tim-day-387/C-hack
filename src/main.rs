use std::process::Command;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    let mut failed = false;

    match create_config_directory(args.verbose) {
	Ok(_) => if args.verbose {
	    println!("Found or created configuration directory.");
	},
	Err(e) => {
	    println!("{e}");
	    failed = true;
	},
    };

    if failed {
	return Err(());
    }

    let mut echo_hello = Command::new("echo");

    echo_hello.arg("-n");
    echo_hello.arg("Hello");
    echo_hello.arg(format!("{}!", args.name));

    let hello_1 = echo_hello.output().expect("Error!");
    let output = match std::str::from_utf8(&hello_1.stdout) {
	Ok(i) => i,
	Err(_e) => "Error!"
    };

    for _ in 0..args.count {
	if args.verbose {
	    println!("{output}");
	}
    }

    Ok(())
}

fn create_config_directory(verbose:bool) -> Result<(), String> {
    match home::home_dir() {
	Some(mut path) => {
	    path.push(".chack");

	    if verbose {
		println!("{}", path.display());
	    }

	    match fs::create_dir_all(path) {
		Ok(_) =>  Ok(()),
		Err(_) => Err("Could not create config directory!".to_string())
	    }
	},
	None => Err("Impossible to get your home dir!".to_string()),
    }
}
