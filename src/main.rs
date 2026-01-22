mod client;
use client::send_chat;

fn main() {
    let sent = send_chat("Hello, world!");
    println!("{}", sent.chat.message)
}
