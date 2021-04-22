use clap::Clap;
//use clap::{App, Arg};
// use std::env;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "nazeudon",
    about = "Super awsome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);

    // not using third party library.
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // using Builder pattern
    // let matches = App::new("My RPN program")
    //     .version("1.0.0")
    //     .author("nazeudon")
    //     .about("Super awsome sample RPN calculator")
    //     .arg(
    //         Arg::new("formula_file")
    //             .about("Formula written in RPN")
    //             .value_name("FILE")
    //             .index(1)
    //             .required(false),
    //     )
    //     .arg(
    //         Arg::new("verbose")
    //             .about("Sets the level of verbosity")
    //             .short('v')
    //             .long("verbose")
    //             .required(false),
    //     )
    //     .get_matches();

    // match matches.value_of("formula_file") {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // let verbose = matches.is_present("verbose");
    // println!("Is verbosity specified?: {}", verbose);
}
