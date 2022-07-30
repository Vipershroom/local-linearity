use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(value_parser)]
    num: Option<f32>,
}

fn local_linearity(x: f32) -> f32 {
    let a = x.floor();
    let f_of_a = f32::sqrt(a);
    let f_prime_of_a = 1.0 / (2.0 * f_of_a);
    let x_a = x - a;
    let product = f_of_a + (f_prime_of_a * x_a);
    product
}


fn main() {
    let args = Args::parse();
    let product = local_linearity(args.num.unwrap());
    println!("{:.2}", product)
}



