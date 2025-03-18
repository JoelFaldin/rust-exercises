use trait_as_params::Summary;

fn main() {
    println!("Hello, world!");
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}