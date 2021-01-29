use std::marker::PhantomData;

pub struct Handle<T> {
    pub id: u64,
    marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}
