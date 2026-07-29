# 🦀 Multithreaded Web Server in Rust

A production-inspired multithreaded HTTP server built in **Rust** by following the concepts from *The Rust Programming Language*. This project demonstrates low-level networking, concurrency, thread pool implementation, graceful shutdown, and HTTP request handling without relying on external web frameworks.

---

## 🚀 Features

* ✅ Custom Thread Pool implementation
* ✅ Multi-threaded request handling
* ✅ TCP server using `TcpListener`
* ✅ Basic HTTP request parsing
* ✅ Route handling

  * `/` → Returns `hello.html`
  * `/sleep` → Simulates a slow request (5 seconds)
  * Unknown routes → Returns `404.html`
* ✅ Graceful thread shutdown using the `Drop` trait
* ✅ Worker communication using Rust channels (`mpsc`)
* ✅ Shared receiver with `Arc<Mutex<T>>`
* ✅ Static HTML file serving
* ✅ Clean and modular Rust code

---

## 📂 Project Structure

```text
multithreaded-web-server/
│
├── src/
│   ├── main.rs          # HTTP server
│   └── lib.rs           # ThreadPool implementation
│
├── hello.html           # Home page
├── 404.html             # Not Found page
├── Cargo.toml
└── README.md
```

---

## 🏗️ Architecture

```text
                Browser
                   │
            HTTP Request
                   │
            TcpListener
                   │
          Incoming Connection
                   │
            ThreadPool::execute()
                   │
              mpsc Channel
                   │
        ┌──────────┴──────────┐
        │                     │
    Worker 1             Worker 2
        │                     │
      Handle               Handle
    Connection          Connection
        │                     │
     HTTP Response      HTTP Response
```

---

## ⚙️ How It Works

### 1. Server Initialization

The server binds to a local TCP socket.

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
```

---

### 2. Thread Pool Creation

Instead of creating a new thread for every request, a fixed number of worker threads are created.

```rust
let pool = ThreadPool::new(4);
```

Each worker waits for incoming jobs through an `mpsc` channel.

---

### 3. Accept Incoming Connections

```rust
for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
        handle_connection(stream);
    });
}
```

Each connection becomes a job that is executed by an available worker thread.

---

### 4. Request Handling

The server reads the first HTTP request line.

Example:

```http
GET / HTTP/1.1
```

Routes:

| Route         | Response                                 |
| ------------- | ---------------------------------------- |
| `/`           | hello.html                               |
| `/sleep`      | Waits 5 seconds, then returns hello.html |
| Anything else | 404.html                                 |

---

### 5. Sending HTTP Response

The response contains:

* Status Line
* Content Length
* HTML Body

Example:

```http
HTTP/1.1 200 OK
Content-Length: 125

<html>...</html>
```

---

## 🧵 Thread Pool Design

### Components

### ThreadPool

Stores:

* Worker threads
* Job sender

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
```

---

### Worker

Each worker owns:

* Worker ID
* Thread

```rust
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}
```

---

### Job

Every incoming request is converted into a closure.

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

The closure is sent through the channel and executed by a worker.

---

## 🔄 Concurrency Model

```text
Request
   │
   ▼
ThreadPool::execute()
   │
   ▼
mpsc Sender
   │
   ▼
Shared Receiver (Arc<Mutex<_>>)
   │
   ▼
Available Worker
   │
   ▼
Execute Job
```

---

## 🛑 Graceful Shutdown

The server implements the `Drop` trait.

When the thread pool is dropped:

1. The sender is closed.
2. Workers detect channel disconnection.
3. Each worker exits its loop.
4. Main thread joins every worker.

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap();
        }
    }
}
```

This ensures no threads are left running.

---

## 📚 Rust Concepts Demonstrated

* Ownership
* Borrowing
* Lifetimes
* Traits
* Closures
* Dynamic Dispatch
* Smart Pointers
* `Box`
* `Arc`
* `Mutex`
* Channels (`mpsc`)
* Threads
* Thread Synchronization
* Error Handling
* Pattern Matching
* TCP Networking
* File I/O
* HTTP Protocol Basics
* Resource Cleanup with `Drop`

---

## ▶️ Running the Project

Clone the repository:

```bash
git clone https://github.com/tushar434434/multithreaded-web-server.git
```

Move into the project:

```bash
cd multithreaded-web-server
```

Run the server:

```bash
cargo run
```

Open your browser:

```
http://127.0.0.1:7878/
```

Test the slow route:

```
http://127.0.0.1:7878/sleep
```

Test an invalid route:

```
http://127.0.0.1:7878/anything
```

---

## 📸 Example Output

```text
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 disconnected; shutting down.
Shutting down worker 0
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

---

## 🎯 Learning Outcomes

Through this project, I gained practical experience with:

* Designing a custom thread pool from scratch
* Building concurrent applications in Rust
* Synchronizing shared state using `Arc<Mutex<T>>`
* Inter-thread communication with channels
* Parsing HTTP requests manually
* Serving static web content
* Managing worker lifecycles and graceful shutdown
* Understanding how web servers process multiple client requests efficiently

---

## 🔮 Future Improvements

* Support persistent HTTP connections
* HTTP/1.1 keep-alive
* Serve static assets (CSS, JavaScript, images)
* MIME type detection
* Logging middleware
* Configurable thread pool size
* HTTP POST request handling
* Routing system
* Async version using Tokio
* Benchmarking and performance metrics

---

## 📖 References

* *The Rust Programming Language* (The Book)
* Rust Standard Library (`std::net`, `std::thread`, `std::sync`)
* HTTP/1.1 Specification (RFC 9112)

---

## 👨‍💻 Author

**Tushar Kumar**

* GitHub: **https://github.com/tushar434434**

If you found this project useful, consider giving it a ⭐ on GitHub!
