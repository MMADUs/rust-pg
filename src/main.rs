use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

mod generic;

pub trait Person {
    fn say_identity(&self);
}

pub struct Individual {
    first_name: String,
    last_name: String,
}

impl Person for Individual {
    fn say_identity(&self) {
        println!("My name is {} {}", self.first_name, self.last_name);
    }
}

impl Individual {
    pub fn new(first_name: String, last_name: String) -> Self {
        Individual { first_name, last_name }
    }
}

fn main() {
    let person = Individual::new(String::from("Muhammad"), String::from("Nizwa"));
    person.say_identity();
    let daniel = Individual::new(String::from("daniel"), String::from("son son"));
    daniel.say_identity();

    let mut deque: VecDeque<String> = VecDeque::new();
    deque.push_front("danielson".to_string());
    deque.push_back("nizwa".to_string());
    deque.push_front("lintang".to_string());
    deque.pop_front();

    for i in deque {
        println!("nama: {}", i);
    }

    let mut linked: LinkedList<usize> = LinkedList::new();
    linked.push_front(10);
    linked.push_back(20);
    linked.push_front(30);

    for i in linked {
        println!("data: {}", i);
    }

    let mut pohon: BTreeMap<usize, String> = BTreeMap::new();
    pohon.insert(10, "nizwa".to_string());
    pohon.insert(8, "jeremey rusuh".to_string());
    pohon.insert(12, "danielson".to_string());

    for i in pohon {
        println!("btree key: {} value: {}", i.0, i.1);
    }
    
    let mut binary: BinaryHeap<String> = BinaryHeap::new();
    binary.push("tes".to_string());
    binary.push("abc".to_string());
    binary.push("xuas".to_string());
    binary.push("pwld".to_string());

    println!("binary: {:?}", binary);
    
    for i in binary {    
        println!("value: {}", i);
    }
}

