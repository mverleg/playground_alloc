use ::bumpalo::Bump;
use ::smallvec::SmallVec;
use ::ustr::Ustr;

#[derive(Debug)]
pub struct Element {
    pub name: Ustr,
}

#[derive(Debug)]
pub struct Node<'a> {
    pub elem: Element,
    pub children: SmallVec<[&'a mut Node<'a>; 2]>
}

#[derive(Debug)]
pub struct File<'a> {
    pub arena: Bump,
    pub data: Ustr,
    pub elements: Vec<&'a mut Element>,
    pub root: Option<Node<'a>>,
}

fn main() {
    let mut f1 = File {
        arena: Bump::new(),
        data: Ustr::from("hello world, this is some random data, don't clone it"),
        elements: vec![],
        root: None,
    };
    let mut elem: &mut Element;
    elem = f1.arena.alloc(Element { name: Ustr::from("hello"), });
    f1.elements.push(elem);
    println!("{:?}", f1);
}
