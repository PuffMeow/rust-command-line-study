fn main() {
    // get_args 返回 Ok 的话，返回的结果会通过 and_then 被传递到 catr::run 里面
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
