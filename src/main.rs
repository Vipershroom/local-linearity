use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(value_parser)]
    num: Option<f32>,
}

fn local_linearity() {
    
}


fn main() {
    let args = Args::parse();
    println!("{:?}", args.num.unwrap())
}



