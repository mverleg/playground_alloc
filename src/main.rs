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
    pub fn new() -> Self {
        Park {
            arena: Bump::new(),
            dogs: vec![],
        }
    }

    pub fn add_dog(&'a mut self, dog: Doggo) -> &'a mut Doggo {
        let allocdog = self.arena.alloc(dog);
        self.dogs.push(allocdog);
        allocdog
    }
}

fn main() {
    g(f());
}

fn f<'a>() -> Park<'a> {
    let mut park = Park::new();

    park.add_dog(Doggo {
        cuteness: u64::max_value(),
        age: 8,
    });

    park
}

fn g(park: Park) {
    for dog in park.dogs {
        println!(">> {:?}", dog);
    }
}
