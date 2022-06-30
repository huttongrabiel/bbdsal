use super::topic_select::DSATopic;

// By the time this function gets called, all files and directories should
// already be generated and all we need to do it write to them.
pub fn generate_boiler_plate(topic: &DSATopic) -> String {
    match topic {
        DSATopic::LinkedList => linked_list_bp(),
        DSATopic::DoublyLinkedList => doubly_linked_list_bp(),
        DSATopic::Tree => tree_bp(),
        DSATopic::BinaryTree => binary_tree_bp(),
        DSATopic::InOrderTreeTraversal => inorder_tree_traversal_bp(),
        DSATopic::PreOrderTreeTraversal => preorder_tree_traversal_bp(),
        DSATopic::PostOrderTreeTraversal => postorder_tree_traversal_bp(),
        DSATopic::BFS => bfs_bp(),
        DSATopic::DFS => dfs_bp(),
        DSATopic::QuickSort => quick_sort_bp(),
        DSATopic::MergeSort => merge_sort_bp(),
        DSATopic::InsertionSort => insertion_sort_bp(),
        DSATopic::BinarySearch => binary_search_bp(),
        DSATopic::LinearSearch => linear_search_bp(),
        DSATopic::Array => array_bp(),
        DSATopic::Vector => vector_bp(),
        DSATopic::Queue => queue_bp(),
        DSATopic::Stack => stack_bp(),
    }
}

// Each of these output boilerplate C++ code.

// TODO: Actually check this code...
fn linked_list_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");
    output.push_str("struct Node {\n");
    output.push_str("};\n\n");

    output.push_str("int len(struct Node* head) {\n}\n\n");
    output.push_str("void append(struct Node* head, struct Node* n) {\n}\n\n");
    output.push_str(
        "void insert(struct Node* head, struct Node* n, int pos) {\n}\n\n",
    );
    output.push_str("void erase(struct Node* head, int pos) {\n}\n\n");
    output.push_str("void print(struct Node* head) {\n}\n\n");
    output.push_str("struct Node* reverse(struct Node* head) {\n}\n\n");

    output.push_str("int main() {\n    return 0;\n}\n");

    output
}

fn doubly_linked_list_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");
    output.push_str("struct Node {\n");
    output.push_str("};\n\n");

    output.push_str("int len(struct Node* head) {\n}\n\n");
    output.push_str("void append(struct Node* head, struct Node* n) {\n}\n\n");
    output.push_str(
        "void insert(struct Node* head, struct Node* n, int pos) {\n}\n\n",
    );
    output.push_str("void erase(struct Node* head, int pos) {\n}\n\n");
    output.push_str("void print(struct Node* head) {\n}\n\n");
    output.push_str("struct Node* reverse(struct Node* head) {\n}\n\n");

    output.push_str("int main() {\n    return 0;\n}\n");

    output
}

fn tree_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn binary_tree_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");
    output.push_str("struct Node {\n\n");
    output.push_str("};\n\n");
    output.push_str("struct Node* create_node(int value) {\n\n");
    output.push_str("}\n\n");

    output.push_str(
        "struct Node* insert_left(struct Node* node, int value) {\n\n",
    );
    output.push_str("}\n\n");

    output.push_str(
        "struct Node* insert_right(struct Node* node, int value) {\n\n",
    );
    output.push_str("}\n\n");

    output.push_str("void inorder_traversal(struct Node* node) {\n\n");
    output.push_str("}\n\n");

    output.push_str("void preorder_traversal(struct Node* node) {\n\n");
    output.push_str("}\n\n");

    output.push_str("void postorder_traversal(struct Node* node) {\n\n");
    output.push_str("}\n\n");

    output.push_str("int main() {\n");
    output.push_str("    ");
    output.push_str("struct Node* root = create_node(8);\n");
    output.push_str("    ");
    output.push_str("insert_left(root, 4);\n");
    output.push_str("    ");
    output.push_str("insert_right(root, 6);\n");
    output.push_str("}\n");

    output
}

fn inorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn preorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn postorder_tree_traversal_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn bfs_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn dfs_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn quick_sort_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn merge_sort_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn insertion_sort_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn binary_search_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output.push_str(
        "int binary_search(int *arr, int arr_length, int target) {\n\n",
    );
    output.push_str("}\n\n");
    output.push_str("int main() {\n");
    output.push_str("    ");
    output.push_str("// Quick test for ya...\n");
    output.push_str("    ");
    output.push_str("int arr[5] = { -2, 1, 4, 17, 90 };\n\n");
    output.push_str("    ");
    output.push_str("int res = binary_search(arr, 5, 90);\n\n");
    output.push_str("    ");
    output.push_str("if (res != -4) {\n");
    output.push_str("    ");
    output.push_str("    ");
    output.push_str(
        "printf(\"\\033[01;31mTEST FAILED: \\033[0mExpected 4 found %d\\n\", res);"
    );
    output.push_str("\n    }\n");
    output.push_str("}");

    output
}

fn linear_search_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn array_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn vector_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn queue_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn stack_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}
