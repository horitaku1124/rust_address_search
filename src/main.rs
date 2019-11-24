#[derive(Debug)]
struct LinkedList {
    name: String,
    next: Option<Box<LinkedList>>,
}


fn main() {
    println!("Hello, Address Search!");

    let mut list = LinkedList{
        name: String::from("abc"),
        next: None
    };
    println!("{:?}", list);
    println!("{}", list.name);

    match list.next {
        Some(n) => {
            println!("next exists");
        },
        None    => {
            println!("next doesn't exist");
        },
    }
}

