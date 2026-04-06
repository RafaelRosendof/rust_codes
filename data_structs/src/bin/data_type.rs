
// in this file gonna have the data type of the data_structs crate
#[derive(Debug)]

struct Figas{
    id: i32,
    age: i32,
    height: f32,
}

/*
This is gonna be a linked list implementation
[1] -> [2] -> [3] -> [4] -> [5]
*/

struct Node{
    data: Figas,
    next: Option<Box<Node>>,
}

struct LinkedList{
    head: Option<Box<Node>>,
}


impl LinkedList{
    fn new() -> Self{
        LinkedList{
            head: None,
        }
    }


    fn push(&mut self, id: i32, age: i32, height: f32){
        let new_data = Figas {
            id,
            age,
            height,
        };

        let new_node = Box::new(Node{
            data: new_data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }


   fn pop(&mut self, id: i32) -> Option<Figas>{
        let mut current = &mut self.head;

        loop{

            match current{

                None => return None,

                Some(node) if node.data.id == id => {
                    let mut remove_node = current.take().unwrap();

                    *current = remove_node.next.take();

                    return Some(remove_node.data);
                }

                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
        
   }

   fn print_list(&self){
        let mut current = &self.head;
        while let Some(node) = current{
            println!("id: {}, age: {}, height: {}", node.data.id, node.data.age, node.data.height);
            current = &node.next;
        }
   }
}

fn main(){
    let mut list = LinkedList::new();

    list.push(1, 25, 5.9);
    list.push(2, 30, 6.0);
    list.push(3, 22, 5.7);

    println!("Linked List:");
    list.print_list();

    println!("\nPopping id 2:");
    if let Some(data) = list.pop(2){
        println!("Popped data: id: {}, age: {}, height: {}", data.id, data.age, data.height);
    } else {
        println!("Data with id 2 not found");
    }

    println!("\nLinked List after popping:");
    list.print_list();
}