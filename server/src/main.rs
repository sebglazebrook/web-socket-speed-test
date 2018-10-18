extern crate ws;
extern crate time;

use ws::listen;
use time::get_time;

fn main() {
    listen("127.0.0.1:2013", |out| {
        move |msg| {
            let new_message = format!("{} {}", get_time().nsec, msg);
            out.send(new_message)
        }
    });
}
