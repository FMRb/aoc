use anyhow::Result;
use nom::{
    IResult, multi::separated_list1,
    character::{complete::{newline, alpha1, self, space0}, is_newline},
    bytes::complete::{tag, take_until1, take_until}, branch::alt
};

#[derive(Debug, PartialEq)]
enum System {
    RootDirectory,
    BackDirectory,
    MoveDirectory(String),
    ListSystem,
    Directory(String),
    File(String, u64),
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
enum ModeType {
    Directory,
    File,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct NodeId {
    index: usize,
}
#[derive(Debug, PartialEq)]
struct Node {
    mode: ModeType,
    size: u64,
    name: String,
    children: Vec<Option<NodeId>>,
    parent: Option<NodeId>,
}

impl Tree {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn root(&self) -> &Node {
        &self.nodes[0]
    }

    fn new_node(&mut self, mode: ModeType, name: String, size: u64) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node {
            mode,
            name,
            size,
            children: Vec::new(),
            parent: None,
        });
        return NodeId { index };
    }
    fn add_child(&mut self, parent_id: NodeId, mode: ModeType, name: String, size: u64) {
        let node_id = self.new_node(mode, name, size);
        self.nodes[node_id.index].parent = Some(NodeId { index: parent_id.index });
        self.nodes[parent_id.index].children.push(Some(node_id));
        self.nodes[parent_id.index].size += size;

        let mut parent_node = parent_id;
        while self.nodes[parent_node.index].parent != None {
            parent_node = self.nodes[parent_node.index].parent.unwrap();
            self.nodes[parent_node.index].size += size;
        }
    }

    fn find_node_directory(&self, context: NodeId, name: &str) -> NodeId {
        let directory = self
            .nodes[context.index]
            .children
            .iter()
            .find(|child| {
                if let Some(child) = child {
                    return self.nodes[child.index].name.eq(&name);
                }
                return false;
            })
            .unwrap();

        return directory.unwrap();
    }

    fn find_directories_less_size(&self, max_size: u64) -> Vec<(&String, u64)> {
        let mut valid_directories = Vec::new();
        for node in self.nodes.iter() {
            if node.mode == ModeType::Directory && node.size <= max_size {
                valid_directories.push((&node.name, node.size));
            }
        }
        valid_directories
    }
    
    fn find_directories_more_size(&self, max_size: u64) -> Vec<(&String, u64)> {
        let mut valid_directories = Vec::new();
        for node in self.nodes.iter() {
            if node.mode == ModeType::Directory && node.size >= max_size {
                valid_directories.push((&node.name, node.size));
            }
        }
        valid_directories
    }
}

fn parse_change_directory(input: &str) -> IResult<&str, System> {
    let (input, _) = space0(input)?;
    let (input, directory) = alt((tag("/"), tag(".."), alpha1))(input)?;

    Ok((input, match directory {
        "/" => System::RootDirectory,
        ".." => System::BackDirectory,
        _ => System::MoveDirectory(String::from(directory)),
    }))

}
fn parse_command(input: &str) -> IResult<&str, System> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alpha1(input)?;
    let (input, command) = match command {
        "cd" => parse_change_directory(input)?,
        "ls" => (input, System::ListSystem), 
        _ => panic!("This cannot happen"),
    };

    Ok((input, command))
}

fn parse_directory(input: &str) -> IResult<&str, System> {
    let (input, _) = tag("dir ")(input)?;
    let (input, directory_name) = alpha1(input)?; 
    Ok((input, System::Directory(String::from(directory_name))))
}

fn parse_file(input: &str) -> IResult<&str, System> {
    let (input, size) = complete::u64(input)?;
    let (input, _) = space0(input)?;
    let (input, name) = take_until("\n")(input)?;
    Ok((input, System::File(String::from(name), size)))
}

fn parse_system(input: &str) -> IResult<&str, System> {
    let (input, result) = alt((parse_command, parse_directory, parse_file))(input)?;
    Ok((input, result))
}

fn parse_file_system(input: &str) -> IResult<&str, Vec<System>> {
    let (input, output) = separated_list1(newline, parse_system)(input)?;
    Ok((input, output))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/7.input")?;
    let (_, result) = parse_file_system(&input).unwrap();

    let mut tree = Tree::new();

    let mut context = NodeId { index: 0 };

    for r in result {
        match r {
            System::RootDirectory => {
                context = tree.new_node(ModeType::Directory, String::from("/"), 0);
            },
            System::BackDirectory => {
                if let Some(parent) = tree.nodes[context.index].parent {
                    context = parent;
                } else {

                    println!("THIS IS NOT GOOD {:?}", tree.nodes[context.index]);
                }
            },
            System::MoveDirectory(name) => {
                context = tree.find_node_directory(context, &name); 
            },
            System::ListSystem => continue,
            System::Directory(name) => {
                tree.add_child(context, ModeType::Directory, name, 0)
            },
            System::File(name, size) => {
                tree.add_child(context, ModeType::File, name, size);
            },
        }
    }

    let valid_directories = tree.find_directories_less_size(100000);

    // Disk space 70000000
    // Need space 30000000
    // let free_up_space = 70000000
    let root_space = tree.root().size;
    dbg!(root_space);
    let free_space = 70000000 - root_space;
    let need_space = 30000000 - free_space;
    dbg!(free_space);
    dbg!(need_space);
    let mut valid_directories_part_2 = tree
        .find_directories_more_size(need_space);

    let result: u64 = valid_directories
        .iter()
        .map(|(_, size)| size)
        .sum();

    valid_directories_part_2
        .sort_by(|(_, size_a), (_, size_b)| size_a.cmp(&size_b));
    
    dbg!(&valid_directories_part_2);
    let result_2 = valid_directories_part_2.get(0).unwrap();
    println!("Root dir: {}", tree.root().size);
    println!("Part 1 {result}");

    println!("Part 2 {}", result_2.1);
    Ok(())
}
