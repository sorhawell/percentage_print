use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    rprintln!("\n\n print pct-char ");
    //println!("\n123%321");
    rprintln!("123%321");
    rprintln!("print completed");

    rprintln!("\n\n print quoted pct-char ");
    //println!("\n123'%'321");
    rprintln!("123'%'321");
    rprintln!("print completed");

    rprintln!("\n\n print inserted pct-char : ");
    //println!("{}", "\n123%321");
    rprintln!("123{}321", "%");
    rprintln!("print completed");

    rprintln!("\n\n print quoted inserted pct-char : ");
    //println!("{}", "\n123'%'321");
    rprintln!("123{}321", "'%'");
    rprintln!("print completed");

    rprintln!("\n\n print dollarsign-char ");
    //println!("\n123$321");
    rprintln!("123$321");
    rprintln!("print completed");

    rprintln!("\n\n print quoted dollarsign-char ");
    //println!("\n123'$'321");
    rprintln!("123'$'321");
    rprintln!("print completed");

    rprintln!("\n\n print inserted dollarsign-char : ");
    //println!("\n123{}321", "$");
    rprintln!("123{}321", "$");
    rprintln!("print completed");

    rprintln!("\n\n print quoted inserted dollarsign-char : ");
    //println!("\n123{}321", "'$'");
    rprintln!("123{}321", "'$'");
    rprintln!("print completed");

    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
}
