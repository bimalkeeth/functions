struct Dog{}
struct Cat{}

pub trait Animal {
    fn make_noise(&self)->&'static str;
}

fn get_animal(rand_number:f64)->Box<dyn Animal>{
    if rand_number < 1.0{
        Box::new(Dog{})
    }else{
        Box::new(Cat{})
    }
}

impl Animal for Dog{
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat{
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}