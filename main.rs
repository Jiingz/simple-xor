mod xor;
use std::env;
use std::fs;

fn main()
{
    let args: Vec<String> = env::args().collect(); // target, key
    let result = xor::encrypt(&args[1], &args[2]);

    fs::write("key.xor", result.clone());

    println!("{}", result);
}
