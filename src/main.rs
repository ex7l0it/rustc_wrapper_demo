use rustc_wrapper_demo::*;

fn main() {
    let running_program = std::env::current_exe().unwrap().to_string_lossy().into_owned();
    let first_param = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: RUSTC_WRAPPER={} cargo build", running_program);
        std::process::exit(1);
    });
    let first_param = first_param.split("/").last();

    match first_param {
        Some("rustc") => {
            let args: Vec<String> = std::env::args().skip(1).collect();
            run_compiler(args, &mut CompilerCalls::new());
        }
        _ => {
            eprintln!("Usage: RUSTC_WRAPPER={} cargo build", running_program);
            std::process::exit(1);
        }
    }
}
