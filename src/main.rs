mod arrays;
mod cli;
mod conditionals;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;

fn message(module: &str) {
    let hdr = "====================";
    println!(
        "\n{hdr}\n  RUNNING: {module}\n{ftr}",
        hdr = hdr,
        module = module,
        ftr = hdr
    )
}

fn main() {
    message("print");
    print::run();

    message("vars");
    vars::run();

    message("types");
    types::run();

    message("strings");
    strings::run();

    message("tuples");
    tuples::run();

    message("arrays");
    arrays::run();

    message("vectors");
    vectors::run();

    message("conditionals");
    conditionals::run();

    message("loops");
    loops::run();

    message("functions");
    functions::run();

    message("pointer_ref");
    pointer_ref::run();

    message("structs");
    structs::run();

    message("enums");
    enums::run();

    message("cli");
    cli::run();
}
