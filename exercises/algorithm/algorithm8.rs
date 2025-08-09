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

    // 入队
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }
    // 出队
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }
    // 队首元素
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

pub struct myStack<T>{
	//TODO
    /*
    入栈时：先把新元素加入辅助队列，再把主队列的所有元素依次移到辅助队列，最后交换两个队列。
    出栈时：直接从主队列出队即可（因为最后入栈的元素在队首）。
    */
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // q1为辅，q2位主
        if self.q1.size() == 0{
            self.q1.enqueue(elem);
            while !self.q2.is_empty() {
                self.q1.enqueue( self.q2.dequeue().unwrap());
            }
        }else{
            // q2为辅，q1位主
            self.q2.enqueue(elem);
            while !self.q1.is_empty() {
                self.q2.enqueue( self.q1.dequeue().unwrap());
            }
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty(){
            Err("Stack is empty")
        }else{
            if !self.q1.is_empty(){
                Ok(self.q1.dequeue().unwrap())
            }else {
                Ok(self.q2.dequeue().unwrap())
            }
        }
    }


    pub fn is_empty(&self) -> bool {
		if self.q1.is_empty() && self.q2.is_empty(){
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
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