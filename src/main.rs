mod bot;

fn main() {
    println!("{:?}", bot::get_version());
    bot::start_bot();
}
