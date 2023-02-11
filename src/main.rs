use std::thread;
use std::time::{Duration};
use chrono::{ Utc};
use threads_rust::basic_threads::single_thread::{one_new_thread, waiting_for_all_threads_to_finish};
use threads_rust::shared_state::race_condition::{basic_mutex, mutex_with_miltiple_threads_using_arc};

fn main() {

    //waiting_for_all_threads_to_finish();

    mutex_with_miltiple_threads_using_arc();
    
}
