use core::fmt;

use crate::Config;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

// Update this as you add more DSA topics to DSATopics
pub const TOPIC_COUNT: u32 = 18;

#[derive(PartialEq, Debug)]
pub enum DSATopic {
    // Each enum holds the boiler plate code in its String.
    LinkedList,
    DoublyLinkedList,
    Tree,
    BinaryTree,
    InOrderTreeTraversal,
    PreOrderTreeTraversal,
    PostOrderTreeTraversal,
    BFS,
    DFS,
    QuickSort,
    MergeSort,
    InsertionSort,
    BinarySearch,
    LinearSearch,
    Array,
    Vector,
    Queue,
    Stack,
}

impl fmt::Display for DSATopic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<DSATopic> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DSATopic {
        match rng.gen_range(1..=TOPIC_COUNT) {
            1 => DSATopic::LinkedList,
            2 => DSATopic::DoublyLinkedList,
            3 => DSATopic::Tree,
            4 => DSATopic::BinaryTree,
            5 => DSATopic::InOrderTreeTraversal,
            6 => DSATopic::PreOrderTreeTraversal,
            7 => DSATopic::PostOrderTreeTraversal,
            8 => DSATopic::BFS,
            9 => DSATopic::DFS,
            10 => DSATopic::QuickSort,
            11 => DSATopic::MergeSort,
            12 => DSATopic::InsertionSort,
            13 => DSATopic::BinarySearch,
            14 => DSATopic::LinearSearch,
            15 => DSATopic::Array,
            16 => DSATopic::Vector,
            17 => DSATopic::Queue,
            18 => DSATopic::Stack,
            _ => panic!("Rand picked some wack number"),
        }
    }
}

pub struct StudyTopics {
    pub dsa_selection: Vec<DSATopic>,
}

pub fn generate_study_topics(config: &Config) -> StudyTopics {
    let mut dsa_selection = Vec::new();

    while dsa_selection.len() as u32 != config.amount_of_dsa {
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
