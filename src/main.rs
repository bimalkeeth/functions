
fn main() {
    println!("Hello, world!");

    apply(
       |a:i32| {a*a},
       34
   );

   high_order()
}

fn apply(f:fn(i32)->i32,a:i32){
    println!("result {}",f(a))
}

fn high_order(){
    let limit =500;

    let sum2 =(0..).
        map(|x| x*x).take_while(|&x|x<=limit).
        filter(|x| is_even(*x)).fold(0,|sum,x|sum+x);

    println!("sum is {:?}",sum2)
}

fn is_even(p0: i32) -> bool {
    p0% 2 ==0
}