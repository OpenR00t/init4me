use::clap::Parser;
use::std::process::Command;

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

pub fn initRust(proj_name:String) {
    let mut rust_init = Command::new("cargo");
    rust_init.arg("new");
    rust_init.arg(proj_name);
    rust_init.status().expect("process failed to execute");
    //let rust_init_out = rust_init.output();
    //println!("To implement Rust {}", proj_name);
}
pub fn initCplus(proj_name:String) {
    //initializes new directory from name arg
    let mut init_dir = Command::new("mkdir");
    init_dir.arg(&proj_name);
    init_dir.status().expect("process failed to execute");
    //makes initialization directory
    let mut new_subdir = Command::new("mkdir");
    new_subdir.arg("/{}/bin", proj_name);
    new_subdir.arg("src");
    new_subdir.arg("lib");
    new_subdir.arg("include");
    new_subdir.status().expect("process failed to execute");
    //change to src directory
    let mut chng_src = Command::new("cd");
    chng_src.arg("src");
    chng_src.status().expect("process failed to execute");

    
}
pub fn initPython(proj_name:String) {
    println!("To implement Python {}", proj_name);
}
pub fn initJavaScript(proj_name:String) {
    println!("To implement JavaScript {}", proj_name);
}
pub fn initCsharp(proj_name:String) {
    println!("To implement C# {}", proj_name);
}
pub fn invalidLang(proj_lang:String ,proj_name:String) {
    println!("{} is not yet implementd. Cannot initialize {}", proj_lang, proj_name);
}

fn main() {
    let args = Args::parse();
    
    //How the hell do I do this another way?
    let rust_lang = String::from("rust");
    let cplus_lang = String::from("c++");
    let python_lang = String::from("python");
    let javascript_lang = String::from("javascript");
    let csharp_lang = String::from("c#");

    match args.lang {
        rust_lang => initRust(args.name.to_lowercase()),
        cplus_lang => initCplus(args.name.to_lowercase()),
        python_lang => initPython(args.name.to_lowercase()),
        javascript_lang => initJavaScript(args.name.to_lowercase()),
        csharp_lang => initCsharp(args.name.to_lowercase()),
        _ => invalidLang(args.lang, args.name.to_lowercase()),
    }
}
