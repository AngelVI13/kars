use std::fmt;


#[derive(Debug)]
struct Node<'a> {
	name: &'a str,
}


impl<'a> fmt::Display for Node<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "N[{}]", self.name);
	}
}

fn main() {
    println!("Hello, world!");
    let node = Node{ name: "Angel"};
    println!("{:#?}", node);
    println!("{}", node);
}
