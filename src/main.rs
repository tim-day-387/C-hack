use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use serde_yaml::{self};
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

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    add: Vec<String>,
    remove: Vec<String>
}

fn main() {
    match chack() {
	Ok(_) => (),
	Err(e) => println!("{e}")
    }
}

fn chack() -> Result<(), String> {
    let mut config_directory = PathBuf::new();
    let args = Args::parse();

    match check_config_directory(args.verbose) {
	Ok(i) => {
	    config_directory = i;
	    if args.verbose {
		println!("Found or created configuration directory.");
	    }
	},
	Err(e) => {
	    return Err(e);
	},
    };

    match check_global_config(args.verbose, config_directory) {
	Ok(_) => if args.verbose {
	    println!("Found or created default configuration.");
	},
	Err(e) => {
	    return Err(e);
	},
    };

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

fn check_config_directory(verbose:bool) -> Result<PathBuf, String> {
    match home::home_dir() {
	Some(mut path) => {
	    path.push(".chack");

	    if verbose {
		println!("{}", path.display());
	    }

	    match fs::create_dir_all(path.clone()) {
		Ok(_) =>  Ok(path),
		Err(_) => Err("Could not create config directory!".to_string())
	    }
	},
	None => Err("Impossible to get your home dir!".to_string()),
    }
}

fn check_global_config(verbose:bool, mut path:PathBuf) -> Result<(), String> {
    path.push("default.yaml");

    let f = std::fs::File::open(path).expect("Could not open file.");
    let scrape_config:Config = serde_yaml::from_reader(f).expect("Could not read values.");

    if verbose {
	println!("{scrape_config:?}");
    }

    Ok(())
}
