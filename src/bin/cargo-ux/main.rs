mod argsparser;

fn main() {
    let args = std::env::args().filter(|arg| arg != "ux");
    let app = argsparser::create();
    app.get_matches_from(args);
}
