use std::{io::{self, BufRead, BufReader, Write}, net::TcpStream, str::FromStr};

use crate::query::Query;

pub struct MementoPool<'a> {
    address: &'a str,
    port: usize,
}

impl<'a> MementoPool<'a> {
    pub fn connect(address: &'a str, port: usize) -> Self {
        MementoPool {address, port}
    }

    pub fn add<A, B>(&self, key: A, value: B) -> io::Result<()> 
    where
        A: ToString,
        B: ToString + FromStr,
    {
        let connection_string = format!("{}:{}", self.address, self.port);
        let mut stream = TcpStream::connect(&connection_string)?;

        let key = key.to_string();
        let value = value.to_string();

        let cmd = Query::Add { key, value };


        let command_stream = cmd.to_string();

        stream.write_all(command_stream.as_bytes())?;

        Ok(())
    }

    pub fn get<A, B>(&self, key: A) -> Option<B> 
    where
        A: ToString,
        B: ToString + FromStr,
    {
        let connection_string = format!("{}:{}", self.address, self.port);
        let mut connection = TcpStream::connect(&connection_string);

        match connection {
            Ok(ref mut stream) => {
                let key = key.to_string();
                let query = Query::Get { key };

                if let Err(_) = stream.write_all(query.to_string().as_bytes()) {
                    return None;
                }

                let buf_reader = BufReader::new(stream);

                let res = buf_reader.lines().next().unwrap().unwrap();

                if let Ok(obj) = B::from_str(&res) {
                    return Some(obj)
                }
                

            }
            Err(_) => return None,
        }

        None
    }
}