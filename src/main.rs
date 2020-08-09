use ::bumpalo::Bump;
use ::std::u64;

#[derive(Debug)]
struct Doggo {
    cuteness: u64,
    age: u8,
}

struct Park<'a> {
    pub arena: Bump,
    pub dogs: Vec<&'a mut Doggo>
}

impl <'a> Park<'a> {
    pub fn new() -> Box<Self> {
        Box::new(Park {
            arena: Bump::new(),
            dogs: vec![],
        })
    }

    // pub fn add_dog(&mut self, dog: Doggo) -> &mut Doggo {
    //     let allocdog = self.arena.alloc(dog);
    //     self.dogs.push(allocdog);
    //     allocdog
    // }
}

fn main() {
    g(f());
}

fn f<'a>() -> Box<Park<'a>> {
    let mut park = Park::new();

    let allocdog = park.arena.alloc_with(|| Doggo {
        cuteness: u64::max_value(),
        age: 8,
    });
    park.dogs.push(allocdog);

    park
}

fn g(park: Box<Park>) {
    for dog in park.dogs {
        println!(">> {:?}", dog);
    }
}
