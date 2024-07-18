use std::{thread, time::Duration};

use memento::{query::{parse_add, parse_get, parse_key, parse_key_val, parse_query, Parser}, stash::Stash};

fn main() {
    let mut my_cache = Stash::new();

    my_cache.add("reno".to_string(), "iqbalsah".to_string());
    my_cache.add("renod".to_string(), "iqbalsah".to_string());
    my_cache.add("renow".to_string(), "iqbalsah".to_string());
    my_cache.add("renoq".to_string(), "iqbalsah".to_string());
    my_cache.add("renop".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxap".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpb".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpc".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpd".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpe".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpf".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpg".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxph".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpi".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpj".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpk".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpl".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpm".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpn".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpo".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpp".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpq".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpr".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxps".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpt".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpu".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpv".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpw".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpx".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpy".to_string(), "iqbalsah".to_string());
    my_cache.add("renoxpz".to_string(), "iqbalsah".to_string());
    

    thread::sleep(Duration::from_secs(3));

    println!("{:#?}", my_cache);
    println!("{:?}", parse_key().parse("(\"reno\")"));
    println!("{:?}", parse_key_val().parse("(\"reno\", \"iqbalsah\")"));
    println!("{:?}", parse_query().parse("<GET: (\"kolak\")>"));
    println!("{:?}", parse_query().parse("<ADD: (\"kolak\", \"pisang\")>"));
}
