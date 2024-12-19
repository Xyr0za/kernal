use std::collections::HashMap;
use std::any::Any;

// Define Variable Environment

pub struct Environment {
    variables: HashMap<String, i32>, // Stores variable values
}

impl Environment {
    pub fn new() -> Self { // Creation of new environment
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn clone(self) -> Environment {
        self
    }

    // Environment interaction functions

    pub fn set_variable(&mut self, name: String, value: i32) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<i32> {
        self.variables.get(name).cloned()
    }
}

#[derive(PartialEq, Debug)]
pub enum NodeType{ // Types of Nodes/ Tokens that are accepted
    Identifier,
    Integer,
    Number,
    Sum,
    Shape,
    Assignment,
    Display,
}

// Node Definition

pub struct Node {
    node_type: NodeType,

    value_str: String,
    value_int: i32,
    value_float: f32,

    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn identifier(value_str: String) -> Self {
        Self {
            node_type: NodeType::Identifier,

            value_str,
            value_int: 0,
            value_float: 0.0,
            left: None,
            right: None,
        }
    }

    pub fn assignment(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            node_type: NodeType::Assignment,

            value_str: "0".to_string(),
            value_int: 0,
            value_float: 0.0,
            left,
            right,
        }
    }

    pub fn display(left: Option<Box<Node>>) -> Self {
        Self {
            node_type: NodeType::Display,

            value_str: "0".to_string(),
            value_int: 0,
            value_float: 0.0,

            left,
            right: None,
        }
    }

    pub fn integer(value_int: i32) -> Self {
        Self {
            node_type: NodeType::Integer,

            value_str: value_int.to_string(),
            value_int,
            value_float: value_int as f32,
            left: None,
            right: None,
        }
    }

    pub fn sum(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            node_type: NodeType::Sum,

            value_str: "0".to_string(),
            value_int: 0,
            value_float: 0.0,


            left,
            right,
        }
    }

    pub fn new(node_type: NodeType, value_str: String, value_int: i32, value_float: f32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            node_type,
            value_str,
            value_int,
            value_float,
            left,
            right,
        }
    }

    // The monstrosity of the eval function

    pub fn eval(&self, environment: &mut Environment) ->  Box<dyn Any>{
        match self.node_type {
            NodeType::Integer => {
                Box::new(self.value_int)
            }

            NodeType::Identifier => {
                Box::new(environment.get_variable(&self.value_str).unwrap())
            }

            NodeType::Sum => {
                let mut l1 = 0;
                let mut r1 = 0;

                if let Some(left) = &self.left {
                    if let Some(value) = left.eval(environment).downcast_ref::<i32>() {
                        l1 = *value; // Retrieve the value and update `l1`
                    } else {
                    }
                }

                if let Some(right) = &self.right {
                    if let Some(value) = right.eval(environment).downcast_ref::<i32>() {
                        r1 = *value; // Retrieve the value and update `r1`
                    } else {
                    }
                }

                let result = l1 + r1;

                Box::new(result)
            }

            NodeType::Assignment => {
                let mut identifier_value = 0;
                if let Some(right) = &self.left {
                    if let Some(value) = right.eval(environment).downcast_ref::<i32>() {
                        identifier_value = *value;
                    } else {
                    }
                }

                environment.set_variable(self.right.as_ref().unwrap().value_str.to_string(), identifier_value);
                Box::new("Ok")
            }

            NodeType::Display => {
                let mut display_val = 0;
                if let Some(right) = &self.left {
                    if let Some(value) = right.eval(environment).downcast_ref::<i32>() {
                        display_val = *value;
                    } else {
                    }
                }

                println!("{}", display_val);
                Box::new("Ok")
            }

            _ => Box::new("Edge Case"),
        }
    }
}

// AST CONSTRUCTION

pub fn build_ast(tokens: &Vec<(String, String)>) -> Option<Node> {
    if tokens.is_empty() {
        return None;
    }

    let mut stack: Vec<Node> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        let current_token_type = token.0.clone();
        let current_token_value = token.1.clone();

        if current_token_type == "DIGIT" {
            stack.push(Node::integer(current_token_value.parse::<i32>().unwrap()));
        }

        if current_token_type == "IDENTIFIER" {
            stack.push(Node::identifier(current_token_value.to_string()));
        }

        if current_token_type == "SUM" {
            let node_right = stack.remove(0);
            let node_left = stack.remove(0);

            stack.push(Node::sum(Some(Box::new(node_right)), Some(Box::new(node_left))));
        }

        if current_token_type == "ASSIGNMENT" {
            let node_right = stack.remove(0);
            let node_left = stack.remove(0);

            stack.push(Node::assignment(Some(Box::new(node_right)), Some(Box::new(node_left))));
        }

        if current_token_type == "DISPLAY" {
            let node_right = stack.remove(0);

            stack.push(Node::display(Some(Box::new(node_right))));
        }

        i += 1;
    }


    let return_node = stack.pop(); // Removes and returns the last element as `Option<Node>`.
    Some(return_node.unwrap())
}
// TODO : SHUNTING YARD ALGORITHM