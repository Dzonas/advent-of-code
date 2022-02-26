///
/// Represents a node that has children nodes,
/// and some metadata as u8 values. Each node
/// can have multiple children, but only one
/// parent.
///
#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<u8>,
}

impl Node {
    ///
    /// Creates a node based on a vec of u8 values from the input.
    ///
    pub fn from_data(mut data: Vec<u8>) -> Node {
        data.reverse();
        Node::create_tree(&mut data)
    }

    fn new(children: Vec<Node>, metadata: Vec<u8>) -> Node {
        Node { children, metadata }
    }

    ///
    /// Creates a tree of nodes. Returned node is the root of the tree.
    ///
    fn create_tree(data: &mut Vec<u8>) -> Node {
        let mut n_children = data.pop().unwrap();
        let n_metadata = data.pop().unwrap();
        let mut children = Vec::with_capacity(n_children as usize);

        while n_children > 0 {
            let node = Node::create_tree(data);
            children.push(node);

            n_children -= 1;
        }

        let mut metadata = Vec::with_capacity(n_metadata as usize);

        for _ in 0..n_metadata {
            metadata.push(data.pop().unwrap());
        }

        Node::new(children, metadata)
    }

    ///
    /// Calculates sum of all metadata entries in the tree.
    ///
    pub fn get_metadata_sum(&self) -> u32 {
        let mut children_sum = 0;

        for child in &self.children {
            children_sum += child.get_metadata_sum();
        }

        let metadata_sum= self.metadata.iter().fold(0, |acc: u32, f| acc + *f as u32);

        children_sum + metadata_sum
    }

    ///
    /// Calculates value of the root node.
    /// That is if node has no children then it's value is the sum of node's metadata entries.
    /// Otherwise it is the sum of values of children, which indexes are contained in the
    /// metadata entries.
    ///
    pub fn get_value(&self) -> u32 {
        let mut value = 0;

        if self.children.len() == 0 {
            value = self.metadata.iter().fold(0, |acc: u32, f| acc + *f as u32);
        } else {
            let indexes = self.metadata.iter()
                .filter(|&&v| v as usize <= self.children.len() && v > 0)
                .map(|&f| (f - 1) as usize);

            for i in indexes {
                value += self.children[i].get_value();
            }
        }

        value
    }
}

pub fn parse(text: &str) -> Vec<u8> {
    let mut data = Vec::with_capacity(text.len() / 2);

    for c in text.split_whitespace() {
        let value: u8 = c.parse().unwrap();
        data.push(value);
    }

    data.shrink_to_fit();

    data
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = include_str!("../test");

    #[test]
    fn test_metadata_sum() {
        let data = parse(INPUT);
        let node = Node::from_data(data);

        assert_eq!(138, node.get_metadata_sum());
    }

    #[test]
    fn test_value() {
        let data = parse(INPUT);
        let node = Node::from_data(data);

        assert_eq!(66, node.get_value());
    }
}