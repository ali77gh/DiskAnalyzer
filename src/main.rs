mod args;
mod crawler;
mod util;

use args as arg;
use util::*;
fn main() {
    let arguments = arg::get_args();
    let tree = crawler::get_tree(&arguments);
    let s = tree.get_size();
    println!("{}", thousends_seperator(s));
    dbg!(tree);
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
}

mod tests {
    use super::*;
    #[test]
    fn custom_folder_test() {}
}
