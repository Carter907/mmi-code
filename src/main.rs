use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short='l', long)]
    clion: bool,
    #[arg(short, long)]
    idea: bool,
    #[arg(short, long)]
    fleet: bool,
    #[arg(short='c', long)]
    idea_comm: bool,
    #[arg(short, long)]
    webstorm: bool,
    #[arg(short, long, default_value_t=(".".to_string()))]
    path: String

}

fn main() {

    include_bytes!("../scripts/clion.cmd");
    include_bytes!("../scripts/fleet.cmd");
    include_bytes!("../scripts/idea.cmd");
    include_bytes!("../scripts/idea_comm.cmd");
    include_bytes!("../scripts/webstorm.cmd");


    let args = Args::parse();
    if args.clion {

        println!("clion detected");
        open_ide("clion.cmd", &args.path);
    }
    if args.idea {
        println!("idea detected");
        open_ide("idea.cmd", &args.path);
    }
    if args.fleet {
        println!("fleet detected");
        open_ide("fleet.cmd", &args.path);
    }
    if args.idea_comm {
        println!("idea_comm detected");
        open_ide("idea_comm.cmd", &args.path);
    }
    if args.webstorm {
        println!("webstrm detected");
        open_ide("webstorm.cmd", &args.path);
    }

}

fn open_ide(program: &str, path: &String) {


    Command::new("cmd")
        .arg("/C")
        .arg(program)
        .arg(path)
        .output()
        .expect("failed to launch clion");
}
