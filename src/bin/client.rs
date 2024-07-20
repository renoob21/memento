use memento::connection::MementoPool;

fn main() {
    let conn = MementoPool::connect("127.0.0.1", 7366);

    conn.add("reno".to_string(), "iqbalsah".to_string()).unwrap();

    let res = conn.get("reno".to_string());

    println!("{:?}", res);
}