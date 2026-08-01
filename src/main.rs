use nonebot_rs;
use mybot::*;
use builtin_plugins::matcher::prelude::*;
fn main() {
    let mut nb = nonebot_rs::Nonebot::new();
    let mut matchers = Matchers::new_empty();
    matchers.add_message_matchers(vec![
        meme_parser::meme_parser(),
        helper::helper(),
        hzys::hzys(),
        otto_quotation::otto_quotation(),
        ]);
    nb.add_plugin(matchers);
    nb.add_plugin(builtin_plugins::logger::Logger);
    nb.run()
}
