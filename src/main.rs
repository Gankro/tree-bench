extern crate collections;
extern crate test;

use std::collections::{RingBuf, BTreeSet, TreeSet};
use std::io;
use std::os;
use std::rc::Rc;

type Set = TreeSet<Rc<String>>;

struct History {
    q: RingBuf<Rc<String>>,
    max_size: uint,
}

impl History {
    fn new(max_size: uint) -> History {
        History {
            q: RingBuf::with_capacity(max_size),
            max_size: max_size,
        }
    }

    fn push(&mut self, s: Rc<String>, map: &mut Set) {
        self.q.push_back(s);
        if self.q.len() > self.max_size {
            match self.q.pop_front() {
                None => {},
                Some(e) => { remove(map, &e); },
            }
        }
    }
}

#[inline(never)]
fn insert(map: &mut Set, e: Rc<String>) {
    map.insert(e);
}

#[inline(never)]
fn remove(map: &mut Set, e: &Rc<String>) {
    map.remove(e);
}

#[inline(never)]
fn contains(map: &Set, e: &Rc<String>) -> bool {
    map.contains(e)
}

const HISTORY_SIZE: uint = 409600;
const CONTAINS_LOOPS: uint = 2;

fn main() {
    let mut f = io::BufferedReader::new(io::File::open(&Path::new(os::args()[1].clone())).unwrap());

    let mut map = TreeSet::new();
    //let mut map = BTreeSet::new();
    // TODO: all of these will be in cache.
    let mut hist = History::new(HISTORY_SIZE);

    let mut x = 0i;

    for line in f.lines() {
        for word in line.unwrap().words() {
            let w = Rc::new(word.to_string());
            insert(&mut map, w.clone());
            hist.push(w, &mut map);
            for s in hist.q.iter().take(CONTAINS_LOOPS) {
                if contains(&mut map, s) {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
        }
    }

    println!("x = {}", x);
}
