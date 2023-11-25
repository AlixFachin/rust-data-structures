pub struct LinkedList<T: Clone> {
    next: Option<Box<LinkedList<T>>>,
    val: Option<T>,
}

impl<T: Clone> LinkedList<T> {

    pub fn new(val: Option<T>) -> Self {
        return Self {
            val: val,
            next: None,
        }
    }

    pub fn len(&self) -> i32 {
        if let None = self.val {
            return 0;
        }

        let mut result = 1;
        let mut cur_pointer = self;

        while let Some(ref next) = cur_pointer.next {
            result = result + 1;
            cur_pointer = next;
        }

        return result;

    }

    pub fn append(&mut self, new_val: T) {        
        
        match &self.val {
            None => {
                self.val = Some(new_val);
                return;},
            Some(_val) => {
                // Let's find the last one
                if let Some(ref mut next) = &mut self.next {
                    next.append(new_val);
                } else {
                    self.next = Some(Box::new(LinkedList { next: None, val: Some(new_val) }));
                }
            }
        }

    }

    pub fn pop(&mut self) -> T {
        if let None = self.val {
            panic!("Cannot pop an empty list!");            
        }
        
        if let Some(ref mut next) = &mut self.next {
            // Testing if the next block is the last one
            if let Some(ref mut _next_next) = &mut next.next {
                // Then we can continue
                return next.pop();
            } else {
                // The next one is empty!
                let result = next.val.clone().unwrap();
                self.next = None;
                return result;
            }
        } else {
            let result = self.val.clone().unwrap();
            self.val = None;
            return result;
        }
    }    
}

impl<T :std::fmt::Display + Clone> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let None = self.val {
            return write!(f, "<>");
        }

        let mut cur_pointer = self;
        let mut finished = false;
        let mut result = String::from("");
        while !finished {
            result.push_str(&cur_pointer.val.clone().unwrap().to_string());
            if let Some(ref next_element) = cur_pointer.next {
                result.push_str("->");
                cur_pointer = &next_element;
            } else {
                finished = true;
            }
        }
        return writeln!(f,"{}",&result);
    }
}