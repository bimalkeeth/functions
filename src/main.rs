
fn main() {
    println!("Hello, world!");

    apply(
       |a:i32| {a*a},
       34
   );
}

fn apply(f:fn(i32)->i32,a:i32){
    println!("result {}",f(a))
}