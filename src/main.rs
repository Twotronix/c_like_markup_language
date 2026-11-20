mod markup;

fn main() {
    println!("{:?}", markup::parse("test.clml").expect("Could not open file"));
}
