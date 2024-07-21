use std::{io::{BufRead, BufReader}, net::TcpListener};

use memento::{query::Query, server::MementoStash};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7366").unwrap();

    let db = MementoStash::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.expect("unable to get connection stream");
        let buf_reader = BufReader::new(&mut stream);

        let bits = buf_reader.lines().next().unwrap().unwrap();

        let query = Query::from_string(bits);


        match query {
            Query::Add { key, value } => {
                db.add(key, value, stream).unwrap();
            }
            Query::Get { key } => {
                db.get(key, stream).unwrap();
            }
        }


    }
}
