use crate::Config;
use crate::boilerplate_gen;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

// Update this as you add more DSA topics to DSATopics
pub const TOPIC_COUNT: u32 = 18;

#[derive(PartialEq, Debug)]
pub enum DSATopic {
    // Each enum holds the boiler plate code in its String.
    LinkedList(String),
    DoublyLinkedList(String),
    Tree(String),
    BinaryTree(String),
    InOrderTreeTraversal(String),
    PreOrderTreeTraversal(String),
    PostOrderTreeTraversal(String),
    BFS(String),
    DFS(String),
    QuickSort(String),
    MergeSort(String),
    InsertionSort(String),
    BinarySearch(String),
    LinearSearch(String),
    Array(String),
    Vector(String),
    Queue(String),
    Stack(String),
}

impl Distribution<DSATopic> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DSATopic {
        match rng.gen_range(1..=TOPIC_COUNT) {
            1 => {
                let bp_code = boilerplate_gen::linked_list_bp();
                DSATopic::LinkedList(bp_code)
            },
            2 => DSATopic::DoublyLinkedList(todo!()),
            3 => DSATopic::Tree(todo!()),
            4 => DSATopic::BinaryTree(todo!()),
            5 => DSATopic::InOrderTreeTraversal(todo!()),
            6 => DSATopic::PreOrderTreeTraversal(todo!()),
            7 => DSATopic::PostOrderTreeTraversal(todo!()),
            8 => DSATopic::BFS(todo!()),
            9 => DSATopic::DFS(todo!()),
            10 => DSATopic::QuickSort(todo!()),
            11 => DSATopic::MergeSort(todo!()),
            12 => DSATopic::InsertionSort(todo!()),
            13 => DSATopic::BinarySearch(todo!()),
            14 => DSATopic::LinearSearch(todo!()),
            15 => DSATopic::Array(todo!()),
            16 => DSATopic::Vector(todo!()),
            17 => DSATopic::Queue(todo!()),
            18 => DSATopic::Stack(todo!()),
            _ => panic!("Rand picked some wack number"),
        }
    }
}

pub struct StudyTopics {
    pub dsa_selection: Vec<DSATopic>,
}

pub fn generate_study_topics(config: &Config) -> StudyTopics {
    let mut dsa_selection = Vec::new();

    let len: u32 = dsa_selection.len().try_into().unwrap();

    while len != config.amount_of_dsa {
        let topic: DSATopic = rand::random();
        if !dsa_selection.contains(&topic) {
            dsa_selection.push(topic);
        }
    }

    dbg!(&dsa_selection);

    StudyTopics {
        dsa_selection,
    }
}
