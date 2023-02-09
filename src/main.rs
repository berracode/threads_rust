use std::thread;
use std::time::{Duration};
use chrono::{ Utc};

fn main() {

    let mut v = Vec::<std::thread::JoinHandle<()>>::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let ini = Utc::now();
            println!("{:?} Se ejecuta el hilo: {i}", ini);
            thread::sleep(Duration::from_secs(10));
            let fin = Utc::now();
            println!("{:?} finaliza el hilo {i}", fin);
        });
        v.push(handle);

    }

    for jh in v.into_iter() {
        jh.join().unwrap();
    }
    
}
