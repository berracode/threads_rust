use std::{thread, time::Duration};


/// Función que usa spawn para crear un nuevo hilo de ejecución
/// No hay garantía que el bucle se ejecute completamente, es decir el for
/// quizá nunca llegue a decir: "hi number 9 form the spawned thread"
/// esto ocurré debido a que no hay garantía en el orden y debido a que por 1 millisegundo el hilo spawned duerme.
pub fn one_new_thread(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// Está función guarda el retorno de spawn en una variable y luego con el metodo join
/// garantizamos que el hilo finalicé antes de que acabe toda la función. 
/// Los hilos se siguen alternando, es decir: imprimira entre hi number {} from the spawned thread y 
/// "hi number {} from the main thread!"
/// Pero el hilo principal de la función espera debido a handle.join
pub fn waiting_for_all_threads_to_finish(){
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}