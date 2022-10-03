

pub trait Voice {
    fn new(name: &'static str) -> Self;
    fn voice(&self)-> String;
}

struct Dog{
    species:&'static str
}

struct Cat{
    color:&'static str
}

impl Voice for Dog{
    fn new(name: &'static str) -> Self {
        Dog{
            species:name
        }
    }

    fn voice(&self) -> String {
        let string = format!("{:?} is barking", self.species);
        string
    }
}

impl Voice for Cat{
    fn new(name: &'static str) -> Self {
        Cat{
            color:name
        }
    }

    fn voice(&self) -> String {
        let string = format!("{:?} is barking", self.species);
        string
    }
}

pub fn make_it_sound<T:Voice>(b:T){
    println!("{:?}",b.voice())
}