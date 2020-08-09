use ::bumpalo::Bump;
use ::smallvec::SmallVec;
use ::ustr::Ustr;

#[derive(Debug)]
pub struct Element {
    pub data: u32,
    pub name: Ustr,
}

#[derive(Debug)]
pub struct Node<'a> {
    pub elem: &'a mut Element,
    pub children: SmallVec<[&'a mut Node<'a>; 2]>
}

#[derive(Debug)]
pub struct File<'a> {
    pub data: Ustr,
    pub elements: Vec<&'a mut Element>,
    pub root: Option<Node<'a>>,
}

fn main() {
    let arena = Bump::new();
    let mut f1 = File {
        data: Ustr::from("hello world, this is some random data, don't clone it"),
        elements: vec![],
        root: None,
    };
    f1.elements.push(arena.alloc(Element { data: 1, name: Ustr::from("hello"), }));
    f1.elements.push(arena.alloc(Element { data: 2, name: Ustr::from("hello"), }));
    f1.elements.push(arena.alloc(Element { data: 4, name: Ustr::from("world"), }));
    let root = Node {
        elem: f1.elements.get_mut(0).unwrap(),
        children: SmallVec::new(),
    };
    f1.root = Some(root);
    println!("{:?}", f1);
}
