use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

#[derive(Clone, Debug)]
struct ItemShard {
    data: String,
    age: usize,
}

#[derive(Debug)]
pub struct Stash {
    stash: Arc<Mutex<HashMap<String, ItemShard>>>,
    iter: usize,
}

impl Stash {
    pub fn new() -> Self {
        let stash = Arc::new(Mutex::new(HashMap::new()));
        Stash { stash, iter: 0 }
    }

    pub fn add(&mut self, key: String, value: String) {
        let new_item = ItemShard {data: value, age: 0};
        if let Ok(ref mut stash) = self.stash.lock() {
            stash.insert(key, new_item);
        }
        self.iter += 1;

        println!("{:?}", self);

        if self.iter >= 5 {
            self.add_age();
            self.iter = 0;
        }
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        if let Ok(ref mut stash) = self.stash.lock() {
            let entry = stash.get_mut(&key);
            match entry {
                Some(item) => {
                    item.age = 0;
                    return Some(item.data.clone());
                }
                None => return None,
            }
        } else {
            return None;
        };
    }

    fn add_age(&mut self) {
        let hash_map = Arc::clone(&self.stash);

        thread::spawn(move || {
            if let Ok(ref mut stash) = hash_map.lock() {
                for (key, _) in stash.clone().into_iter() {
                    let entry = stash.get_mut(&key).unwrap();

                    entry.age += 1;

                    if entry.age >= 5 {
                        stash.remove(&key);
                    }
                }
            }
        });
    }
}