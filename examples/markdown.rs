//! ```bash
//! cargo run --example markdown test.org
//! ```

use orgize::{export::MarkdownExport, Org};
use std::{env::args, fs};

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        panic!("Usage: {} <org-mode-file>", args[0]);
    }

    let content = fs::read_to_string(&args[1]).unwrap();

    let mut export = MarkdownExport::default();
    Org::parse(content).traverse(&mut export);

    fs::write(format!("{}.md", &args[1]), export.finish()).unwrap();

    println!("Wrote to {}.md", &args[1]);
}
