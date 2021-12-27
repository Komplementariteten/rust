use crate::StoreableWithSchema;

pub struct Batch<T: StoreableWithSchema> {
    buff: Vec<Option<T>>,
    pub items: u32,
}

impl<T: StoreableWithSchema> Batch<T> {
    pub fn new() -> Batch<T> {
        Batch {
            items: 0,
            buff: Vec::new(),
        }
    }
    pub fn add(&mut self, item: T) -> usize {
        self.items += 1;
        self.buff.push(Some(item));
        self.buff.len() - 1
    }
    pub fn remove(&mut self, index: usize) {
        if let Some(item) = self.buff.get(index) {
            if item.is_some() {
                self.items -= 1;
                self.buff[index] = None
            }
        }
    }
    pub fn clear(&mut self) -> Vec<T> {
        let mut items = Vec::new();
        for item_opt in self.buff.drain(0..) {
            if let Some(item) = item_opt {
                items.push(item)
            }
        }
        self.items = 0;
        items
    }
}
