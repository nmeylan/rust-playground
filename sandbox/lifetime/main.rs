
#[derive(Debug)]
struct A {

}
#[derive(Debug)]
struct Node<'a> {
    x: u16,
    y: u16,
    a: &'a A,
    parent: Option<&'a Node<'a>>
}

fn main() {
    let a = A{};
    let mut parent = Node {
        x: 0,
        y: 0,
        a: &a,
        parent: None
    };
    let mut node = Node {
        x: 1,
        y: 1,
        a: &a,
        parent: Some(&parent)
    };

    println!("{:?}", node);
}