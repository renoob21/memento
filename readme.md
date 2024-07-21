# Memento

My own attempt to implement a memory-caching program using Rust. I am currently learning Rust and I'm trying to improve my Rust capabilities by building this program.

I was inspired to build this application when watching Professor David J. Malan's video on scalability and they talked about memory caching [here](https://www.youtube.com/watch?v=-W9F__D3oY4)

## Some Key Learning Point / Rust Features that I Used when Building This

* Generic Types
* Traits: trait bounds, trait implementation
* Managing errors using ```Result``` type
* Managing Null data using ```Option``` type
* Implementing parser
* Connecting and performing data exchanges through TCP Connection using ```std::net```
* Perform non-blocking operation with multi-threading using ```std::thread```
* Multi-threading with thread-safe types such as ```Arc``` and ```Mutex``` for database sharding

## Some Other Works that I Used for References

* [Parser Combinator by Bodil](https://bodil.lol/parser-combinators/)
* [Mini-Redis by Tokio Team](https://tokio.rs/tokio/tutorial/setup)

### Note

I tried my best not to use any external dependencies when building this, and the program is presented as is

## Some Key Features of Memento

* Can store data in the form of key-value pair
* The ```key``` could be any data types as long as it satisfies the ```ToString``` trait
* The ```value``` could be any data types as long as it satisfies the ```ToString``` and ```FromStr``` trait
* Assign "age" in each entries, updated whenever new data get added or when a data is accessed

Thank you everyone, enjoy 😊