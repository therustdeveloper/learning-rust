//! src//home/william/workspace/rust/learning-rust/src/lib.rs
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod add_millimeters_to_meters;
pub mod advanced_match;
pub mod algorithms;
pub mod api_mutex;
pub mod arrays;
pub mod automatic_clean;
pub mod basics;
pub mod bit_patterns;
pub mod borrowing;
pub mod boxes;
pub mod channels;
pub mod closures;
pub mod collections;
pub mod compiler_directives;
pub mod concurrency;
pub mod cow_type;
pub mod cubesats;
pub mod custom_smart_pointer;
pub mod data_collections;
pub mod decisions;
pub mod deconstructing_float;
pub mod destructuring_structs;
pub mod doc;
pub mod draw;
pub mod effective_rust;
pub mod enums;
pub mod errors;
pub mod exercises;
pub mod expressions;
pub mod extern_functions;
pub mod external_crates;
pub mod files;
pub mod for_loops;
pub mod formatted_print;
pub mod functions;
pub mod futures;
pub mod generics;
pub mod get_type;
pub mod guessing_game;
pub mod hashmaps;
pub mod hello_world;
pub mod inspecting_endianness;
pub mod interior_mutability;
pub mod interpreting_float;
pub mod iterators;
pub mod joinhandle;
pub mod lifetimes;
pub mod loops;
pub mod machine_learning;
pub mod macros;
pub mod mandelbrot;
pub mod matches;
pub mod message_passing;
pub mod messenger;
pub mod methods;
pub mod modules;
pub mod move_vars;
pub mod mutable_reference;
pub mod network;
pub mod new_type;
pub mod numbers;
pub mod object_oriented;
pub mod open_file;
pub mod operator_overloading;
pub mod optionals;
pub mod ownership;
pub mod patterns;
pub mod post;
pub mod print_var_type;
pub mod random_numbers;
pub mod ranges;
pub mod ratatui_app;
pub mod rc_t;
pub mod references;
pub mod results;
pub mod rust_atomics_and_locks;
pub mod shadowing;
pub mod slices;
pub mod static_variable;
pub mod strings;
pub mod structs;
pub mod system_calls;
pub mod testing;
pub mod threads;
pub mod traits;
pub mod tree_data_structure;
pub mod tuples;
pub mod type_aliases;
pub mod typestate_pattern;
pub mod unsafe_code;
pub mod unsafe_trait;
pub mod user_input;
pub mod variables;
pub mod vectors;
pub mod webserver;
pub mod while_loop;
pub mod zero_cost_abstraction;

pub fn print_title(title: &str) {
    println!(" ");
    println!("### {title} ###");
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _id in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    //id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            //id,
            thread: Some(thread),
        }
    }
}
