use memento::connection::MementoPool;

fn main() {
    let conn = MementoPool::connect("127.0.0.1", 7366);

    conn.add("reno", "iqbalsah".to_string()).unwrap();

    let res: Option<String> = conn.get("reno".to_string());

    println!("{:?}", res);
}