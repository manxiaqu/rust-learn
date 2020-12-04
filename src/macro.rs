// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

// check hex with expect len
macro_rules! create_validator {
    ($fn_name:ident, $len: expr, $msg: literal) => {
        fn $fn_name(val: &str) {
            if val.len() != $len {
                println!("{}: expect {}, actual {}", $msg, $len, val.len());
            }
        }
    };
}

macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_validator!(check_addr, 20, "invalid address");

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!();

    check_addr("32");
}
