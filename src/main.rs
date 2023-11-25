mod linkedlist;

use linkedlist::LinkedList;

fn main() {
    let mut s = LinkedList::new(Some(3));

    s.display();
    s.append(4);
    s.append(5);    
    println!("{}",s);
    println!("Length of list is {}", s.len());
    let result = s.pop();
    println!("Result of pop is: {}", result);
    println!("{}",s);
    println!("Result of pop is: {}", s.pop());
    println!("Length of list is {}", s.len());
    println!("{}",s);
    
}
