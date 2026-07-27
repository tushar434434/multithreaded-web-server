//Building multithreaded web server ============


//starting with the single threaded web server


//the two protocols
//http and tcp ,both are request-response protocols 
//TCP is the lower-level protocol that describes the details of how information gets from one server to another but doesn’t specify what that information is. HTTP builds on top of TCP by defining the contents of the requests and responses. It’s technically possible to use HTTP with other protocols, but in the vast majority of cases, HTTP sends its data over TCP. We’ll work with the raw bytes of TCP and HTTP requests and responses.

//listening to the tcp connection=>The standard library offers a std::net module that lets us do this

use std::net::TcpListener;

fn main(){
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listner.incoming(){
        let stream = stream.unwrap();
        println!("connection established");
    }
}
//The bind function in this scenario works like the new function in that it will return a new TcpListener instance. The function is called bind because, in networking, connecting to a port to listen to is known as “binding to a port.”
//The bind function returns a Result<T, E>, which indicates that it’s possible for binding to fail
