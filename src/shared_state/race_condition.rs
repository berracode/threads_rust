use std::{sync::{Mutex, Arc}, thread, rc::Rc};


/// Mutex sirve para usar la exlusión mutua en Rust
/// Nos ayuda a prevenir la race condition o data race (condición de carrera)
/// Race condition: cuando distintos hilos tratan de usar un recurso al mismo tiempo y este recurso está en la sección critica
/// Normalmente estos recursos son recursos compartidos, como variables globales, ficheros, conexiones, entre otros. 
pub fn basic_mutex(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}


/// Está función da error, debido a qué no podemos mover a multiples subprocesos la propiedad de la variable counter. 
/// Recordemos que para tener multiple propiedad usamos Rc
/// Recuerde que en Rust la propiedad tiene reglas y una de ellas es, solo existe un dueño
/// esto puede ser alterado usando Rc
pub fn mutex_with_miltiple_threads(){
    let counter = Mutex::new(0);
    // let mut handles = vec![];

    /*for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }*/

    println!("Result: {}", *counter.lock().unwrap());
}


/// Ahora tenemos un nuevo problema, tenemos propiedad multiple pero al compilar nos indica que no seguro. 
/// Neceitamos algo similar a Rc pero seguro para hilos
pub fn mutex_with_miltiple_threads_using_rc(){
    let counter = Rc::new(Mutex::new(0));
   /* let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }*/

    println!("Result: {}", *counter.lock().unwrap());
}


/// Arc es como Rc, pero seguro para usar en situaciones concurrentes. La 'A' significa Atomic.
/// 
/// 
pub fn mutex_with_miltiple_threads_using_arc(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
