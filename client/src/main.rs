extern crate ws;
extern crate time;

use ws::{connect, CloseCode, Message};
use time::get_time;


fn main() {
    println!("Started");
    connect("ws://127.0.0.1:2013", |out| {
        out.send("Hello WebSocket").unwrap();

        move |msg: Message| {
            let end = get_time().nsec;
            println!("- {} at {}", msg, end);
            let first = msg.as_text().unwrap().split_whitespace().next().unwrap();
            let start = first.parse::<i32>().unwrap();
            let diff = end - start;
            println!("Diff = {} nanoseconds", diff);
            out.close(CloseCode::Normal)
        }

    }).unwrap()
}
