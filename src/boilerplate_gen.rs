// Each of these output boilerplate C++ code.

// TODO: Actually check this code...
pub fn linked_list_bp() -> String {
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
