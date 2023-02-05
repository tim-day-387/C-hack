use serde::{Deserialize, Serialize};
use std::process::Command;
use clap::{Args, Parser};
use std::path::PathBuf;
use serde_yaml::{self};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Arguments {
    #[command(subcommand)]
    commands: Option<Commands>,

    /// Output verbose logging
    #[clap(global = true)]
    #[arg(short, long)]
    verbose: bool,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Create new compiler profile
    Create(Create),

    /// List all compiler profiles
    List(List),

    /// Enable/disable a given compiler profile
    Toggle(Toggle),

    /// Show compiler profile
    Show(Show)
}

#[derive(Args)]
struct Create {
    /// Compiler profile name
    #[arg(short, long)]
    name: String
}

#[derive(Args)]
struct List {
}

#[derive(Args)]
struct Toggle {
    /// Compiler profile name
    #[arg(short, long)]
    name: String
}

#[derive(Args)]
struct Show {
    /// Compiler profile name
    #[arg(short, long)]
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: String,
    add: Vec<String>,
    remove: Vec<String>
}

fn main() {
    match chack() {
	Ok(_) => (),
	Err(e) => eprintln!("{e}")
    }
}

fn chack() -> Result<(), String> {
    let args = Arguments::parse();

    let config_directory = match check_config_directory(args.verbose) {
	Ok(i) => {
	    if args.verbose {
		println!("Found or created configuration directory.");
	    }
	    i
	},
	Err(e) => {
	    return Err(e);
	}
    };

    match &args.commands {
	Some(i) => match run_sub_command(i, config_directory, args.verbose) {
	    Ok(_) => return Ok(()),
	    Err(e) => return Err(e)
	},
	None => ()
    };

    let mut program;
    match read_profile(config_directory, "default".to_string(), args.verbose) {
	Ok(i) => {
            program = Command::new(i.path);
	    for arg in i.add {
		program.arg(arg);
	    }
	},
	Err(e) => {
            return Err(e)
	}
    };

    let results = match program.output() {
	Ok(i) => {
	    if args.verbose {
		println!("Program ran correctly");
	    }
	    i
	},
	Err(_) => {
            return Err("Program failed!".to_string());
	}
    };

    let output = match std::str::from_utf8(&results.stdout) {
	Ok(i) => i,
	Err(_) => {
	    return Err("Could not parse output!".to_string());
	}
    };

    if args.verbose {
	println!("{output}");
    }

    Ok(())
}

fn run_sub_command(commands:&Commands, config_directory:PathBuf, verbose:bool) -> Result<(), String> {
    match commands {
	Commands::Create(_) => {
	    Ok(())
	}
	Commands::List(_) => {
	    Ok(())
	}
	Commands::Toggle(_) => {
	    Ok(())
	}
	Commands::Show(i) => {
            match read_profile(config_directory, i.name.clone(), verbose) {
		Ok(i) => {
                    print!("{}", serde_yaml::to_string(&i).unwrap());
                    return Ok(());
		},
		Err(e) => {
                    return Err(e)
		},
            };
	}
    }
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

fn _list_profiles(_path:PathBuf, _verbose:bool) -> Result<(), String> {
    Ok(())
}

fn _create_profile(mut path:PathBuf, name:String, _verbose:bool) -> Result<(), String> {
    path.push(name);
    Ok(())
}

fn read_profile(mut path:PathBuf, name:String, verbose:bool) -> Result<Config, String> {
    path.push(name);

    let file = match std::fs::File::open(path) {
	Ok(i) => i,
	Err(_) => return Err("Could not open file!".to_string())
    };

    match serde_yaml::from_reader(file) {
	Ok(i) => {
	    if verbose {
		println!("{i:?}");
	    }
	    Ok(i)
	},
	Err(_) => Err("Could not read yaml!".to_string())
    }
}
