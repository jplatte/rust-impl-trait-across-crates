extern crate lib;
use lib::Thing;

fn main() {
    let thing = Thing::new();

    for x in thing.even_elems() {
        println!("{}", x)
    }
}
