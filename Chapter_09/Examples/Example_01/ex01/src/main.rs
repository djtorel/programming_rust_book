fn main() {
    let mut foo = Queue::new();
    foo.push(123);
    foo.push(0);
    
    println!("{}", foo.pop().unwrap());
    
    foo.push(1);
    foo.push(3);
    
    println!("{}", foo.pop().unwrap());
    println!("{}", foo.pop().unwrap());
    println!("{}", foo.pop().unwrap());
    println!("Queue is empty: {:?}", foo.is_empty());
    
}

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.younger.push(item);
    }
    
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        
        self.older.pop()
    }
}