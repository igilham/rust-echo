use std::env;

#[cfg(windows)] pub const NL: &'static str = "\r\n";
#[cfg(not(windows))] pub const NL: &'static str = "\n";

fn main() {
    let mut print_new_line = true;

    // skip the first element as it is the program name
    for arg in env::args().skip(1) {
        if arg == "-n" {
            print_new_line = false;
        } else {
            print!("{} ", arg);
        }
    }
    
    if print_new_line {
        print!("{}", NL);
    }
}
