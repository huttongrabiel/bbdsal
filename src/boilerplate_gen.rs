use super::topic_select::StudyTopics;
use super::topic_select::DSATopic;

// By the time this function gets called, all files and directories should
// already be generated and all we need to do it write to them.
pub fn generate_boiler_plate(study_topics: StudyTopics) {
    for topic in study_topics.dsa_selection {
        match topic {
            DSATopic::LinkedList => {
                let _linked_list_bp = linked_list_bp();
 			},
            DSATopic::DoublyLinkedList => {
 			},
            DSATopic::Tree => {
 			},
            DSATopic::BinaryTree => {
 			},
            DSATopic::InOrderTreeTraversal => {
 			},
            DSATopic::PreOrderTreeTraversal => {
 			},
            DSATopic::PostOrderTreeTraversal => {
 			},
            DSATopic::BFS => {
 			},
            DSATopic::DFS => {
 			},
            DSATopic::QuickSort => {
 			},
            DSATopic::MergeSort => {
 			},
            DSATopic::InsertionSort => {
 			},
            DSATopic::BinarySearch => {
 			},
            DSATopic::LinearSearch => {
 			},
            DSATopic::Array => {
 			},
            DSATopic::Vector => {
 			},
            DSATopic::Queue => {
 			},
            DSATopic::Stack => {
 			},
        }
    }
}

// Each of these output boilerplate C++ code.

// TODO: Actually check this code...
fn linked_list_bp() -> String {
    let mut output = String::new();

    output.push_str("struct Node {\n");

    output.push_str("int len() {\n}\n");
    output.push_str("void insert(Node* n) {\n}\n");
    output.push_str("void erase() {\n}\n");
    // TODO: Double check that front returns a pointer to a Node
    output.push_str("Node* front() {\n}\n");
    output.push_str("void print() {\n}\n");
    output.push_str("Node* reverse() {\n}\n");

    output.push_str("};\n");

    output
}
