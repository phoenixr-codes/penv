use std::env;

use console::style;
use glob::Pattern;

fn main() {
    let mut args = env::args();
    let filter = args.nth(1);
    if args.count() > 0 {
        panic!("too many arguments");
    }

    for (key, value) in env::vars() {
        if let Some(ref f) = filter {
            if !Pattern::new(f).expect("invalid glob pattern").matches(&key) {
                continue;
            }
        }
        println!(
            "{}\n{}\n",
            style(key).yellow().bold(),
            if value.is_empty() {
                style("(none)").red().italic().to_string()
            } else {
                style(value).to_string()
            }
        );
    }
}
