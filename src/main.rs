use std::{error::Error, path::{Path, PathBuf}, fs::File, io::ErrorKind, process::{Command, ExitStatus}, env};

use walkdir::{WalkDir, DirEntry};
use clap::Parser;

type PathCheckFunc = fn(&Path) -> bool;

type DynError = Box<dyn Error>;

#[derive(Parser,Default,Debug)]
struct Args {
    #[clap(short ,long)]
    java_path:String,
    #[clap(short ,long)]
    source_dir:String,
    #[clap(long)]
    args:Vec<String>,
}

fn main() -> Result<(), DynError> {
    let args = Args::parse();
    println!("javaPath:{}",args.java_path);
    println!("sourceDir:{}",args.source_dir);
    println!("args:{:?}",args.args);
    Ok(())
}

fn parse_env_args<'a,'b,'c>() -> (&'a str, &'b str,Vec<&'c str>) {

    ("","",vec![])
}

fn compile_java(java_path:&str, source_dir:&str, args: Vec<&str>) -> Result<ExitStatus, DynError> {
    let javac_path = java_path.to_owned() + "/bin/javac.exe";

    check_path(&javac_path, "javac not found", Path::is_file)?;

    let files = WalkDir::new(source_dir).into_iter()
            .map(entry_to_path)
            .filter(filter_java_files)
            .collect::<Result<Vec<String>,DynError>>()?;
    let exit_code = Command::new(javac_path)
            .args(files)
            .args(args)
            .status()?;
    Ok(exit_code)
}

fn entry_to_path(entry: Result<DirEntry, walkdir::Error>) -> Result<String, DynError> {
    match entry {
        Err(err) => Err(Box::new(err)),
        Ok(entry) => match entry.path().to_str() {
            Some(str) => Ok(str.to_owned()),
            None => Err(Box::new(std::io::Error::new(ErrorKind::NotFound,"Invalid path")))
        }
    }
}

fn filter_java_files(path: &Result<String, DynError>) -> bool {
    match path {
        Err(_) => true,
        Ok(path) => path.ends_with(".java")
    }
}



fn check_path(path:&str, not_found_msg:&str, func:PathCheckFunc) -> Result<(),DynError> {
    if !func(Path::new(path)) {
        return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, not_found_msg)));
    }
    Ok(())
}