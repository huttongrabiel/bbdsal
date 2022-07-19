use super::topic_select::DSATopic;

/// Writes C code for the given data structure/algorithm.
///
/// # Examples
///
/// fn main() {
///     let topic = DSATopic::LinkedList;
///     generate_boiler_plate(&topic);
/// }
pub fn generate_boiler_plate(topic: &DSATopic) -> String {
    match topic {
        DSATopic::LinkedList => linked_list_bp(),
        DSATopic::DoublyLinkedList => doubly_linked_list_bp(),
        DSATopic::Tree => tree_bp(),
        DSATopic::BinaryTree => binary_tree_bp(),
        DSATopic::BinarySearchTree => binary_search_tree_bp(),
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

    output.push_str("#include <stdio.h>\n");
    output.push_str("#include <stdlib.h>\n\n");

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

    output.push_str(
        "\
void free_tree(struct Node* root) {
    if (root != NULL) {
        free_tree(root->left);
        free_tree(root->right);
        free(root);
    }
}

",
    );

    output.push_str("int main() {\n");
    output.push_str("    ");
    output.push_str("struct Node* root = create_node(8);\n");
    output.push_str("    ");
    output.push_str("insert_left(root, 4);\n");
    output.push_str("    ");
    output.push_str("insert_right(root, 6);\n");
    output.push_str("    ");
    output.push_str("free_tree(root);\n");
    output.push_str("}\n");

    output
}

fn binary_search_tree_bp() -> String {
    let mut output = String::new();

    output.push_str(
        "\
struct Node {

};

struct Node* insert_node(struct Node* root, struct Node* node) {

}

struct Node* maximum(struct Node* root) {

}

struct Node* minimum(struct Node* root) {

}

void inorder_traversal(struct Node* root) {
    // Due to the structure of a binary search tree, an inorder
    // will print the elements in sorted order.
}

int main() {
    return 0;
}\
        ",
    );

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

    output.push_str("#include <stdio.h>\n");
    output.push_str("#include <stdlib.h>\n\n");

    output.push_str(
        "\
int partition(int* arr, int low, int high) {

}

void quick_sort(int* arr, int low, int high) {

}

int main() {
    int const arr_size = 4;
    int* arr = malloc(arr_size * sizeof(int));

    arr[0] = 5;
    arr[1] = 2;
    arr[2] = -1;
    arr[3] = 4;

    quick_sort(arr, 0, arr_size-1);

    for (int i = 0; i < arr_size; i++) {
        printf(\"%d\\n\", arr[i]);
    }

    free(arr);
}
                    ",
    );

    output
}

fn merge_sort_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n\n");

    output
}

fn insertion_sort_bp() -> String {
    let mut output = String::new();

    output.push_str(
        "\
#include <stdio.h>
#include <stdlib.h>

int* insertion_sort(int arr[], int arr_len) {
    int* sorted = malloc(sizeof(int) * arr_len);

    return sorted;
}

// Try and do this one, it's less expensive.
void inplace_insertion_sort(int* arr, int arr_len) {

}

int main() {
    // Copy insertion sort

    int arr[5] = { -2, 7, 19, 4, 1 };

    int arr_len = 5;

    int* sorted_arr = insertion_sort(arr, arr_len);

    for (int i = 0; i < arr_len-1; i++) {
        if (sorted_arr[i+1] < sorted_arr[i]) {
            fprintf(stderr, \"\\033[01;31mTest Failed: \\033[0mArray not sorted\");
            return 1;
        }
    }

    // In place insertion sort

    int* arr2 = malloc(sizeof(int) * 5);

    for (int i = 0; i < arr_len; i++) {
        arr2[i] = arr[i];
    }

    inplace_insertion_sort(arr2, 5);

    for (int i = 0; i < arr_len-1; i++) {
        if (arr2[i+1] < arr2[i]) {
            fprintf(stderr, \"\\033[01;31mIn Place Test Failed: \\033[0mArray not sorted\");
            return 1;
        }
    }

    printf(\"\\033[01;32mAll tests pass!\\033[0m\");

    free(arr2);
    free(sorted_arr);

    return 0;
}
        ",
    );

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
        "fprintf(stderr, \"\\033[01;31mTEST FAILED: \\033[0mExpected 4 found %d\\n\", res);"
    );
    output.push_str("\n        return 1;");
    output.push_str("\n    }\n");
    output.push_str(
        "    printf(\"\\033[01;32mTest Passed: \\033[0mArray sorted!\");",
    );
    output.push_str("\n    return 0;\n");
    output.push_str("}");

    output
}

fn linear_search_bp() -> String {
    let mut output = String::new();

    output.push_str("#include <stdio.h>\n");

    output.push_str(
        "\
#include <stdlib.h>

int linear_search(int* arr, int arr_len, int target) {
    // Return index if found, otherwise return -1.
}

int main() {
    // Tests

    int* arr = malloc(sizeof(int) * 5);

    arr[0] = 5;
    arr[1] = 7;
    arr[2] = -2;
    arr[3] = 1;
    arr[4] = 18;

    // Should return index 1.
    int search1 = linear_search(arr, 5, 7);

    if (search1 != 1) {
        fprintf(
            stderr,
            \"\\033[01;31mTest Failed: \\033[0mIncorrect index for search1.\"
        );
        return -1;
    }

    // Should return -1.
    int search2 = linear_search(arr, 5, 80);

    if (search2 != -1) {
        fprintf(
            stderr,
            \"\\033[01;31mTest Failed: \\033[0mIncorrect index for search2.\"
        );
        return -1;
    }

    printf(\"\\033[01;32mAll tests pass!\\033[0m\");

    free(arr);

    return 0;
}",
    );

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
