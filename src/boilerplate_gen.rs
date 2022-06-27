use super::topic_select::DSATopic;

// By the time this function gets called, all files and directories should
// already be generated and all we need to do it write to them.
pub fn generate_boiler_plate(topic: &DSATopic) -> String {
    match topic {
        DSATopic::LinkedList => {
            linked_list_bp()
        },
        DSATopic::DoublyLinkedList => {
            doubly_linked_list_bp()
        },
        DSATopic::Tree => {
            tree_bp()
        },
        DSATopic::BinaryTree => {
            binary_tree_bp()
        },
        DSATopic::InOrderTreeTraversal => {
            inorder_tree_traversal_bp()
        },
        DSATopic::PreOrderTreeTraversal => {
            preorder_tree_traversal_bp()
        },
        DSATopic::PostOrderTreeTraversal => {
            postorder_tree_traversal_bp()
        },
        DSATopic::BFS => {
            bfs_bp()
        },
        DSATopic::DFS => {
            dfs_bp()
        },
        DSATopic::QuickSort => {
            quick_sort_bp()
        },
        DSATopic::MergeSort => {
            merge_sort_bp()
        },
        DSATopic::InsertionSort => {
            insertion_sort_bp()
        },
        DSATopic::BinarySearch => {
            binary_search_bp()
        },
        DSATopic::LinearSearch => {
            linear_search_bp()
        },
        DSATopic::Array => {
            array_bp()
        },
        DSATopic::Vector => {
            vector_bp()
        },
        DSATopic::Queue => {
            queue_bp()
        },
        DSATopic::Stack => {
            stack_bp()
        },
    }
}

// Each of these output boilerplate C++ code.

// TODO: Actually check this code...
fn linked_list_bp() -> String {
    let mut output = String::new();

    output.push_str("struct Node {\n");
    output.push_str("};\n");

    output.push_str("int len(struct Node* head) {\n}\n");
    output.push_str("void append(struct Node* head, struct Node* n) {\n}\n");
    output.push_str("void erase() {\n}\n");
    output.push_str("struct Node* front(struct Node* head) {\n}\n");
    output.push_str("void print(struct Node* head) {\n}\n");
    output.push_str("struct Node* reverse(struct Node* head) {\n}\n");

    output.push_str("int main() {\n    return 0;\n}");

    output
}

fn doubly_linked_list_bp() -> String {
    let mut output = String::new();

    output.push_str("struct Node {\n");
    output.push_str("};\n");

    output.push_str("int len(struct Node& head) {\n}\n");
    output.push_str("void append(structNode* head, struct Node* n) {\n}\n");
    output.push_str("void erase() {\n}\n");
    output.push_str("struct Node* front(struct Node* head) {\n}\n");
    output.push_str("void print(struct Node* head) {\n}\n");
    output.push_str("struct Node* reverse(struct Node* head) {\n}\n");

    output.push_str("int main() {\n    return 0;\n}");

    output
}

fn tree_bp() -> String {
    let mut output = String::new();

    output
}

fn binary_tree_bp() -> String {
    let mut output = String::new();

    output
}

fn inorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output
}

fn preorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output
}

fn postorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output
}

fn bfs_bp() -> String {
    let mut output = String::new();

    output
}

fn dfs_bp() -> String {
    let mut output = String::new();

    output
}

fn quick_sort_bp() -> String {
    let mut output = String::new();

    output
}

fn merge_sort_bp() -> String {
    let mut output = String::new();

    output
}

fn insertion_sort_bp() -> String {
    let mut output = String::new();

    output
}

fn binary_search_bp() -> String {
    let mut output = String::new();

    output
}

fn linear_search_bp() -> String {
    let mut output = String::new();

    output
}

fn array_bp() -> String {
    let mut output = String::new();

    output
}

fn vector_bp() -> String {
    let mut output = String::new();

    output
}

fn queue_bp() -> String {
    let mut output = String::new();

    output
}

fn stack_bp() -> String {
    let mut output = String::new();

    output
}
