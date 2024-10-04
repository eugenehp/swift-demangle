use std::env;
use swift_demangle::demangle;

fn main() {
    for (index,argument) in env::args().enumerate() {
        if index != 0  {
            let result = demangle(&argument);
            if let Ok(symbol) = result {
                println!("{symbol}");
            }else{
                println!("Error parsing mangled symbol {argument}");
                println!("{}", result.err().unwrap());
            }
        }
    }
}