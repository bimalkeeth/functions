
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize =20;

pub fn concurrency_one() {
    let mut threads =vec![];
    let (tx,rx)=mpsc::channel();

    for i in 0..NUM_THREADS {
        let sender1 = tx.clone();
       let th= thread::spawn(move|| {
            println!("new thread {}",i);

           sender1.send(i).unwrap();
        });

        threads.push(th);
    }


    for v in threads{
        v.join().unwrap();
    }

    for j in rx.iter().take(NUM_THREADS){
        println!("{}",j)
    }

    println!("main thread")
}