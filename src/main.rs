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

    // pub fn add_dog(&'a mut self, dog: Doggo) -> &'a mut Doggo {
    //     let allocdog = self.arena.alloc(dog);
    //     self.dogs.push(allocdog);
    //     allocdog
    // }
}

fn main() {
    g(f());
}

fn f<'a>() -> Vec<&'a mut Doggo> {
    // Create a new arena to bump allocate into.
    let bump = Bump::new();

    // Allocate values into the arena.
    let dogs = vec![
        bump.alloc(Doggo {
            cuteness: u64::max_value(),
            age: 8,
        }),
        bump.alloc(Doggo {
            cuteness: 9001,
            age: 2,
        }),
    ];

    dogs
}

fn g(dogs: Vec<&mut Doggo>) {
    for dog in dogs {
        println!(">> {:?}", dog);
    }
}
