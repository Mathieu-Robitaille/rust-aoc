
type ChildNode<T> = Option<Box<BTNode<T>>>

struct BinaryTree {
    head: BTNode
}

impl BinaryTree {
    pub fn new(head: BTNode) -> self {
        BinaryTree { head: head }
    }
}

struct BTNode {
    left: ChildNode<T>,
    right: ChildNode<T>,
    id: Option<T>
}

impl BTNode<T> {
    pub fn new (id: Option<T>, l: BTNode<T>, r: BTNode<T>) -> self {
        BTNode {
            id: id,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

struct Ticket {
    encoded_position: String,
    seat_id: usize,
    seen: Option<bool>,
}

impl Ticket {
    fn new(input_encoded_position: &str) -> Ticket {
        Ticket {
            encoded_position: input_encoded_position.to_string(),
            seat_id: calc_id(input_encoded_position),
            seen: None,
        }
    }

    fn calc_id(input_encoded_position: &str) -> usize {

    }
}