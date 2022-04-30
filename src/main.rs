use ::clap::Parser;
use ::std::process::Command;
use std::fs;
use::colored::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// name of the language to initialize
    /// rust, c++, python, javscript, c#
    #[clap(short, long)]
    lang: String,

    /// Name of project
    #[clap(short, long)]
    name: String,
}

pub fn init_rust(proj_name: String) {
    let mut rust_init = Command::new("cargo");
    rust_init.arg("new");
    rust_init.arg(proj_name);
    rust_init.status().expect("process failed to execute");
    //let rust_init_out = rust_init.output();
    //println!("To implement Rust {}", proj_name);
}
pub fn init_cplus(proj_name: String) -> std::io::Result<()> { 
    //TODO: maybe make the paths a vector and have it create by iteration?
    let src_path = format!("./{}/src/{}.cpp", proj_name, proj_name);
    let bin_path = format!("./{}/bin", proj_name);
    let include_path = format!("./{}/include/{}.h", proj_name, proj_name);
    let lib_path = format!("./{}/lib", proj_name);
    fs::create_dir_all(src_path).unwrap();
    fs::create_dir_all(bin_path).unwrap();
    fs::create_dir_all(include_path).unwrap();
    fs::create_dir_all(lib_path).unwrap();
    //println!("{}: {} was successfuly intialized", "Created".green().bold(), proj_name.bold());
    Ok(println!("{}: {} was successfuly intialized as C++", "Created".green().bold(), proj_name.bold()))
}
pub fn init_py(proj_name: String) -> std::io::Result<()> { 
    fs::create_dir_all(format!("./{}/{}.py", proj_name, proj_name)).unwrap();
    println!("{}: {} was successfuly intialized", "Created".green().bold(), proj_name.bold());
    Ok(())
}
pub fn init_js(proj_name: String) { 
    println!("To implement JavaScript {}", proj_name);
    //TODO: implement javascript initialization with node, react, typescript modifiers
}
pub fn init_csharp(proj_name: String) { //TODO: implement c# initialization
    println!("To implement C# {}", proj_name);
    //TODO: implement c# initialization
}
pub fn invalid_lang(proj_lang: &str, proj_name: String) { 
    println!(
        "{} is not yet implemented. Cannot initialize {}",
        proj_lang, proj_name
    );
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let lang_name: &str = &args.lang.to_lowercase();

    match lang_name {
        "rust" => init_rust(args.name),
        "c++" => init_cplus(args.name)?,
        "python" => init_py(args.name)?,
        "javascript" => init_js(args.name),
        "c#" => init_csharp(args.name),
        _ => invalid_lang(lang_name, args.name),
    }
    Ok(())
}
