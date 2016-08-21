#![feature(conservative_impl_trait)]

pub struct Thing {
    data: Vec<u32>
}

impl Thing {
    pub fn new() -> Thing {
        Thing {
            data: vec![1, 2, 3, 4]
        }
    }

    pub fn even_elems<'a>(&'a self) -> impl Iterator<Item=&'a u32> {
        self.data.iter().filter(|x| **x % 2 == 0)
    }
}
