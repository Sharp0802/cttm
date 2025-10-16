use std::process::exit;

fn main() {
    if let Err(e) = cttm::compile_all("tpl/**/*.ct") {
        eprintln!("{}", e);
        exit(1);
    }
}
