//! # Automotive Troubleshooting Guide
//!
//! This module implements an interactive decision tree-based diagnostic system
//! for identifying and resolving common automobile problems.
//!
//! ## Features
//!
//! - **Binary Decision Tree**: Navigates through yes/no questions to identify issues
//! - **Interactive Prompting**: Guides users through the troubleshooting process
//! - **Comprehensive Coverage**: Addresses multiple potential car problems including:
//!   - Battery and electrical system issues
//!   - Starting and ignition problems
//!   - Fuel delivery complications
//! - **Solution-Oriented**: Provides specific actions to resolve identified problems
//! - **Graph-based Structure**: Uses petgraph for efficient decision tree representation
use petgraph::{
    graph::{DefaultIx, NodeIndex},
    visit::EdgeRef,
    Graph,
};
use std::io::Write;

type TroubleshootDecisionTree = Graph<String, bool>;
type DecisionTreeNode = NodeIndex<DefaultIx>;

fn create_troubleshoot_tree() -> (DecisionTreeNode, TroubleshootDecisionTree) {
    let mut decision_tree: TroubleshootDecisionTree = Graph::new();
    let base = decision_tree.add_node("Is the car silent when you turn the key?".to_string());
    let l1_a = decision_tree.add_node("Are the battery terminals corroded?".to_string());
    let l1_b = decision_tree.add_node("Does the car make a clicking noise?".to_string());
    let l2_a = decision_tree.add_node("Clean terminals and try starting again.".to_string());
    let l2_b = decision_tree.add_node("Replaces cables and try again.".to_string());
    let l2_c = decision_tree.add_node("Replace the battery.".to_string());
    let l2_d = decision_tree.add_node("Does the car crank up but fail to start?".to_string());
    let l3_a = decision_tree.add_node("Check spark plug connections.".to_string());
    let l3_b = decision_tree.add_node("Does the engine start and then die?".to_string());
    let l4_a = decision_tree.add_node("Does your car have fuel injection?".to_string());
    let l4_b =
        decision_tree.add_node("No further questions. Please consult a mechanic.".to_string());
    let l5_a =
        decision_tree.add_node("Check to ensure the choke is opening and closing.".to_string());
    let l5_b = decision_tree.add_node("Get it in for service.".to_string());

    decision_tree.extend_with_edges([
        (base, l1_a, true),
        (base, l1_b, false),
        (l1_a, l2_a, true),
        (l1_a, l2_b, false),
        (l1_b, l2_c, true),
        (l1_b, l2_d, false),
        (l2_d, l3_a, true),
        (l2_d, l3_b, false),
        (l3_b, l4_a, true),
        (l3_b, l4_b, false),
        (l4_a, l5_a, false),
        (l4_a, l5_b, true),
    ]);
    (base, decision_tree)
}

fn prompt_for_answer(prompt: &str) -> bool {
    loop {
        print!("{prompt} (yes/no): ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Invalid input. Please enter 'yes' or 'no'."),
        }
    }
}

fn main() {
    let is_question = |node: &str| node.contains('?');
    let (mut root, decision_tree) = create_troubleshoot_tree();

    loop {
        let current = decision_tree[root].clone();
        if is_question(&current) {
            let answer = prompt_for_answer(current.as_str());
            let next_node = decision_tree
                .edges(root)
                .find(|edge| edge.weight() == &answer)
                .map(|edge| edge.target())
                .expect("No matching edge found");

            root = next_node;
        } else {
            println!("{}", current);
            break;
        }
    }
}
