//Building multithreaded web server ============


//starting with the single threaded web server


//the two protocols
//http and tcp ,both are request-response protocols 
//TCP is the lower-level protocol that describes the details of how information gets from one server to another but doesn’t specify what that information is. HTTP builds on top of TCP by defining the contents of the requests and responses. It’s technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over TCP. We’ll work with the raw bytes of TCP and HTTP requests and responses.

//listening to the tcp connection=>The standard library offers a std::net module that lets us do this
/*
use std::net::TcpListener;

fn main(){
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listner.incoming(){
        let stream = stream.unwrap();
        println!("connection established");
    }
}*/
//The bind function in this scenario works like the new function in that it will return a new TcpListener instance. The function is called bind because, in networking, connecting to a port to listen to is known as “binding to a port.”
//The bind function returns a Result<T, E>, which indicates that it’s possible for binding to fail


// reading the request ===
use std::{
    fs,
    io::{BufReader,prelude::*},// to get access to traits and types that let us read from and write to the stream.
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
};
use multithreaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

/*
fn handle_connection(mut stream: TcpStream){
    let buf_reader =BufReader::new(&stream);//create a new BufReader instance that wraps a reference to the stream
    //The BufReader adds buffering by managing calls to the std::io::Read trait methods for us.
    let http_request: Vec<_> =buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())//BufReader implements the std::io::BufRead trait, which provides the lines method. The lines method returns an iterator of Result<String, std::io::Error> by splitting the stream of data whenever it sees a newline byte. 
        .collect();
     //   println!("Request: {http_request:#?}");
     // let response = "HTTP/1.1 200 OK\r\n\r\n";//succes message data  
     let status_line = "HTTP/1.1 200 OK";
      let contents = fs::read_to_string("hello.html").unwrap();
      let length = contents.len();
      let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

      stream.write_all(response.as_bytes()).unwrap();//convert the string data to bytes. The write_all method on stream takes a &[u8] and sends those bytes directly down the connection. the write_all could fail thats why we wrapped it
}*/
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    /*let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };*/
    let (status_line,filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK","hello.html")
        }
        _=> ("HTTP/1.1 404 NOT FOUND","404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
    /* request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
    }*/




//Looking More Closely at an HTTP Request====
/*HTTP is a text-based protocol, and a request takes this format:


Method Request-URI HTTP-Version CRLF//first line indicate the method get or post then /, which indicate the uniform resource identifier
headers CRLF
message-body*/
//CRLF stands for carriage return and line feed, which are terms from the typewriter days!) The CRLF sequence can also be written as \r\n, where \r is a carriage return and \n is a line feed. The CRLF sequence separates the request line from the rest of the request data.


//Writing a resposne====
/*response format =>
HTTP-Version Status-Code Reason-Phrase CRLF//The first line is a status line that contains the HTTP version used in the response, a numeric status code that summarizes the result of the request, and a reason phrase that provides a text description of the status code
headers CRLF
message-body*/

//Returning real html =====
//Validating the Request and Selectively Responding



//Refactoring=====


//From a Single-Threaded to a Multithreaded Server========

//simulating a slow request=======


//improving throughtput with a thread pool
//spawning a thread for each request     
