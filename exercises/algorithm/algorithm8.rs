/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
{
	queues: [Queue<T>; 2],
    active_idx: usize,
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            queues: [Queue::new(), Queue::new()],
            active_idx: 0,
        }
    }
    pub fn push(&mut self, elem: T) {
        self.active_queue().enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        while let Ok(elem) = self.active_queue().dequeue() {
            if self.active_queue().is_empty() {
                self.active_idx = 1 - self.active_idx;
                return Ok(elem);
            } 
            self.inactive_queue().enqueue(elem);
        }
        Err("Stack is empty")
    }
    pub fn is_empty(&self) -> bool {
		self.queues[self.active_idx].is_empty()
    }
    fn active_queue(&mut self) -> &mut Queue<T> {
        &mut self.queues[self.active_idx]
    }
    fn inactive_queue(&mut self) -> &mut Queue<T> {
        &mut self.queues[1 - self.active_idx]
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}