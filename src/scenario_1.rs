use rusqlite::Connection;
use tiny_http::{Server, Response};
use std::sync::Arc;
use crossbeam::thread;
use std::time::Instant;

pub fn run_server() {
    // Create an HTTP server that listens on port 8000
    let server = Arc::new(Server::http("0.0.0.0:8000").unwrap());

    println!("Listening on http://0.0.0.0:8000/");

    // Number of worker threads
    let num_workers = 4;

    // Use crossbeam to create and manage the thread pool
    thread::scope(|scope| {
        for i in 0..num_workers {
            let server = Arc::clone(&server);
            let worker_name = format!("Worker-{}", i + 1);

            scope.spawn(move |_| {
                for request in server.incoming_requests() {
                    let start = Instant::now();
                    println!("Request handled by {}", worker_name);

                    // Open a connection to SQLite
                    match Connection::open("my_db.sqlite") {
                        Ok(conn) => {
                            // Close the connection by letting it go out of scope
                            drop(conn);
                        }
                        Err(e) => {
                            eprintln!("Failed to open SQLite connection: {}", e);
                        }
                    }

                    let duration = start.elapsed();
                    println!("Time taken to open and close SQLite connection: {:?}", duration);

                    // Respond to the request
                    let response = Response::from_string("SQLite connection open/close measured");
                    request.respond(response).unwrap();
                }
            });
        }
    }).unwrap();
}
