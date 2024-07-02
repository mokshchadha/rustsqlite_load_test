use rusqlite::{Connection, OpenFlags};
use tiny_http::{Server, Response};
use std::sync::Arc;
use crossbeam::thread;
use std::time::Instant;

pub fn run_server() {
    // Create an HTTP server that listens on port 8001
    let server = Arc::new(Server::http("0.0.0.0:8001").unwrap());

    println!("Listening on http://0.0.0.0:8001/");

    // Number of worker threads
    let num_workers = 4;

    // Open the SQLite database in multi-thread mode
    let db_flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_URI | OpenFlags::SQLITE_OPEN_NO_MUTEX;

    // Use crossbeam to create and manage the thread pool
    thread::scope(|scope| {
        for i in 0..num_workers {
            let server = Arc::clone(&server);
            let worker_name = format!("Worker-{}", i + 1);

            scope.spawn(move |_| {
                for request in server.incoming_requests() {
                    let start = Instant::now();
                    println!("Request handled by {}", worker_name);

                    // Open a connection to SQLite in multi-thread mode
                    match Connection::open_with_flags("my_db.sqlite", db_flags) {
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
