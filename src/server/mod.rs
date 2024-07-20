use std::{io::{self, Write}, net::TcpStream, sync::{Arc, Mutex}, thread};

use crate::stash::Stash;

pub struct MementoStash {
    stashes: Arc<Vec<Mutex<Stash>>>,
}

impl MementoStash {
    pub fn new(capacity: usize) -> Self {
        let mut stashes = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            stashes.push(Mutex::new(Stash::new()))
        }

        MementoStash {
            stashes: Arc::new(stashes)
        }
    }

    pub fn add(&self, key: String, value: String, mut stream: TcpStream) -> io::Result<()> {
        let db = Arc::clone(&self.stashes);

        thread::spawn(
            move || -> io::Result<()> {
                db[hash(&key) % db.len()].lock().unwrap().add(key, value);
                stream.write_all("\0\n".as_bytes())?;
                Ok(())
            }
        );

        Ok(())
    }

    pub fn get(&self, key: String, mut stream: TcpStream) -> io::Result<()> {
        let db = Arc::clone(&self.stashes);

        thread::spawn(
            move || -> io::Result<()> {
                let data = db[hash(&key) % db.len()].lock().unwrap().get(key);
                match data {
                    Some(value) => {
                        println!("{}", value);
                        stream.write_all(format!("{}\n", value).as_bytes())?;
                        Ok(())
                    },
                    None => {
                        stream.write_all("\0\n".as_bytes())?;
                        Ok(())
                    }
                }
            }
        );

        Ok(())
    }
}

fn hash(key: &str) -> usize {
    let chars = key.as_bytes();

    let mut total = 0;

    for c in chars {
        total += *c as usize;
    }

    for i in 0..(chars.len() - 1) {
        let addition = chars[i] as i32 - chars[i+1] as i32;
        total += abs(addition);
    }
    total
}

fn abs(num: i32) -> usize {
    if num < 0 {
        (num * -1) as usize
    } else {
        num as usize
    }
} 