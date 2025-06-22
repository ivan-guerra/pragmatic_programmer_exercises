//! # Mad Libs Adventure Game
//!
//! This module implements an interactive, branching story Mad Libs game that combines
//! user-generated creative input with a decision tree narrative structure.
//!
//! ## Features
//!
//! - **Interactive Storytelling**: Users progress through a branching narrative based on yes/no decisions
//! - **Mad Libs Integration**: Each story node contains placeholders for nouns, verbs, adjectives, and adverbs
//! - **Decision Tree Structure**: Uses petgraph to model the story as a directed graph with boolean edge weights
//! - **Customizable Experience**: Each playthrough creates a unique story based on user input and choices
//! - **Template-Based Text**: Story templates dynamically incorporate user-provided words
//! - **Multiple Endings**: The narrative branches to different conclusions based on user decisions
use petgraph::{
    graph::{DefaultIx, NodeIndex},
    visit::EdgeRef,
    Graph,
};
use std::fmt::Display;

#[derive(Debug, Default, Clone)]
struct MadLib {
    noun: String,
    verb: String,
    adjective: String,
    adverb: String,
    story_template: String,
}

impl MadLib {
    fn new(story_template: String) -> Self {
        MadLib {
            noun: String::new(),
            verb: String::new(),
            adjective: String::new(),
            adverb: String::new(),
            story_template,
        }
    }

    fn prompt_for_blanks(&mut self) {
        let mut input = String::new();
        for placeholder in ["{noun}", "{verb}", "{adjective}", "{adverb}"] {
            if self.story_template.contains(placeholder) {
                println!(
                    "Please enter a {}:",
                    placeholder.trim_matches('{').trim_matches('}')
                );
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let trimmed_input = input.trim().to_string();
                match placeholder {
                    "{noun}" => self.noun = trimmed_input,
                    "{verb}" => self.verb = trimmed_input,
                    "{adjective}" => self.adjective = trimmed_input,
                    "{adverb}" => self.adverb = trimmed_input,
                    _ => panic!("Unexpected placeholder: {}", placeholder),
                }
                input.clear();
            }
        }
    }
}

impl Display for MadLib {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let final_story = self
            .story_template
            .replace("{noun}", &self.noun)
            .replace("{verb}", &self.verb)
            .replace("{adjective}", &self.adjective)
            .replace("{adverb}", &self.adverb);
        write!(f, "{}", final_story)
    }
}

fn create_madlibs_decision_tree() -> (NodeIndex<DefaultIx>, Graph<MadLib, bool>) {
    let mut decision_tree: Graph<MadLib, bool> = Graph::new();
    let base = decision_tree.add_node(MadLib::new(
        "Did you ever {verb} a {adjective} {noun} before breakfast?".to_string(),
    ));
    let branch_a = decision_tree.add_node(MadLib::new(
        "Did the wizard offer you a {noun} in return?".to_string(),
    ));
    let branch_b = decision_tree.add_node(MadLib::new(
        "Were you instead chased by a {adjective} {noun} on a bicycle?".to_string(),
    ));
    let branch_a_1 = decision_tree.add_node(MadLib::new(
        "Did you accept the {noun} and use it to unlock a secret door?".to_string(),
    ));
    let branch_a_2 = decision_tree.add_node(MadLib::new(
        "Did you politely decline and invite the {noun} to a game of football?".to_string(),
    ));
    let branch_b_1 = decision_tree.add_node(MadLib::new(
        " Did the {noun} demand you answer a riddle about flowers?".to_string(),
    ));
    let branch_b_2 = decision_tree.add_node(MadLib::new(
        "Did you quietly sneak into a {noun}'s house instead?".to_string(),
    ));
    let question_8 = decision_tree.add_node(MadLib::new(
        "Did the correct answer to the riddle open a portal to {noun}?".to_string(),
    ));
    let question_8_1 = decision_tree.add_node(MadLib::new(
        "THE END: You are crowned ruler of the land. Enjoy your reign!".to_string(),
    ));
    let question_8_2 = decision_tree.add_node(MadLib::new(
        "THE END: You are turned into a talking {noun}. Enjoy your new life!".to_string(),
    ));
    let question_9 = decision_tree.add_node(MadLib::new(
        "Did your spontaneous decision cause a {adjective} {noun} to unfold?".to_string(),
    ));
    let question_9_1 = decision_tree.add_node(MadLib::new(
        "THE END: You save the town, accidentally.".to_string(),
    ));
    let question_9_2 = decision_tree.add_node(MadLib::new(
        "THE END: You are blamed for everything and sent to {noun}.".to_string(),
    ));
    let question_10 = decision_tree.add_node(MadLib::new(
        "Did you find a dusty {noun} that spoke in riddles?".to_string(),
    ));
    let question_10_1 = decision_tree.add_node(MadLib::new(
        "THE END: It grants you three oddly specific wishes.".to_string(),
    ));
    let question_10_2 = decision_tree.add_node(MadLib::new(
        "THE END: You wake up. It was all a dream... or was it?".to_string(),
    ));
    decision_tree.extend_with_edges([
        (base, branch_a, true),
        (base, branch_b, false),
        (branch_a, branch_a_1, true),
        (branch_a, branch_a_2, false),
        (branch_b, branch_b_1, true),
        (branch_b, branch_b_2, false),
        (branch_a_1, question_8, true),
        (branch_a_1, question_9, false),
        (branch_a_2, question_9, true),
        (branch_a_2, question_10, false),
        (branch_b_1, question_8, true),
        (branch_b_1, question_10, false),
        (branch_b_2, question_9, true),
        (branch_b_2, question_10, false),
        (question_8, question_8_1, true),
        (question_8, question_8_2, false),
        (question_9, question_9_1, true),
        (question_9, question_9_2, false),
        (question_10, question_10_1, true),
        (question_10, question_10_2, false),
    ]);
    (base, decision_tree)
}

fn main() {
    println!("Welcome to Mad Libs!");
    println!("You will be asked a series of questions to fill in the blanks for a story.");

    let (root, decision_tree) = create_madlibs_decision_tree();
    let mut current = root;
    loop {
        let mut madlib = decision_tree[current].clone();
        madlib.prompt_for_blanks();

        if decision_tree.edges(current).count() == 0 {
            println!("{}", madlib);
            break;
        }

        let mut input = String::new();
        loop {
            println!("{}", madlib);
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let answer = input.trim().to_lowercase();
            if answer == "yes" || answer == "no" {
                break;
            } else {
                println!("Please enter 'yes' or 'no'.");
                input.clear();
            }
        }

        current = decision_tree
            .edges(current)
            .find(|edge| *edge.weight() == (input == "yes"))
            .map(|edge| edge.target())
            .expect("No matching edge found");
    }
}
