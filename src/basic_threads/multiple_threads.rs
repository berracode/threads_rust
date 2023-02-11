use std::{thread, time::Duration};

use chrono::Utc;



pub fn multiple_threads_with_join(){
    let mut v = Vec::<std::thread::JoinHandle<()>>::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            let ini = Utc::now();
            println!("{:?} Se ejecuta el hilo: {i}", ini);
            thread::sleep(Duration::from_secs(2));
            let fin = Utc::now();
            println!("{:?} finaliza el hilo {i}", fin);
        });
        v.push(handle);

    }

    for jh in v.into_iter() {
        jh.join().unwrap();
    }
}