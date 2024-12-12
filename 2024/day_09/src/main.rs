use std::fs;


#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeType {
    File,
    Free,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeFlag {
    Ignore,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Node {
    id: u128,
    repeat: u128,
    is: NodeType,
    flag: NodeFlag,
}


fn disk_str_to_vec( disk: &str ) -> Vec<Node> {
    let mut nodes = Vec::new();

    for ( i, c ) in disk.chars().enumerate() {
        let node = Node {
            id: ( i / 2 ).try_into().unwrap(),
            repeat: c.to_digit( 10 ).unwrap().try_into().unwrap(),
            is: if i % 2 == 0 { NodeType::File } else { NodeType::Free },
            flag: NodeFlag::None,
        };

        nodes.push( node );
    }

    nodes
}


fn defrag_1( disk: &str ) -> Vec<Node> {
    let mut nodes = disk_str_to_vec( disk );
    let mut new = Vec::new();
    let mut start = 0;

    for i in ( 0..nodes.len() ).rev() {
        let mut back = nodes[i];

        if back.is == NodeType::Free {
            continue;
        }

        for j in start..nodes.len() {
            let front = &mut nodes[j];

            if front.is == NodeType::File {
                new.push( front.clone() );
                start = j + 1;
                continue;
            }

            // Fill free space with file space
            if front.repeat >= back.repeat {
                new.push( Node {
                    id: back.id,
                    repeat: back.repeat,
                    is: NodeType::File,
                    flag: NodeFlag::None,
                } );
                front.repeat -= back.repeat;
                back.repeat = 0;

                start = j;
            }
            else if front.repeat != 0 {
                new.push( Node {
                    id: back.id,
                    repeat: front.repeat,
                    is: NodeType::File,
                    flag: NodeFlag::None,
                } );

                back.repeat -= front.repeat;
                front.repeat = 0;
                start = j;
            }

            if start >= i - 1 || back.repeat == 0 {
                break;
            }
        }

        if start >= i - 1 {
            if back.repeat > 0 {
                new.push( Node {
                    id: back.id,
                    repeat: back.repeat,
                    is: NodeType::File,
                    flag: NodeFlag::None,
                } );
            }

            break;
        }
    }

    new
}


fn free_node( list: &mut Vec<Node>, node: &Node ) {
    for n in list.into_iter().rev() {
        if n.id == node.id && n.is == NodeType::File {
            n.is = NodeType::Free;
            n.flag = NodeFlag::Ignore;
            break;
        }
    }
}


fn defrag_2( disk: &str ) -> Vec<Node> {
    let nodes = disk_str_to_vec( disk );
    let mut new = nodes.clone();

    for i in ( 0..nodes.len() ).rev() {
        let node = nodes[i];

        if node.is == NodeType::Free {
            continue;
        }

        for j in 0..new.len() {
            let free = &mut new[j];

            if free.is == NodeType::File || free.flag == NodeFlag::Ignore {
                continue;
            }

            if free.repeat >= node.repeat {
                free.repeat -= node.repeat;
                new.insert( j, node.clone() );
                free_node( &mut new, &node );

                break;
            }
        }
    }

    new
}


fn calc_checksum( disk: &Vec<Node> ) -> u128 {
    let mut index = 0;
    let mut sum: u128 = 0;

    for node in disk {
        if node.is == NodeType::File {
            for i in index..( index + node.repeat ) {
                sum += i * node.id;
            }
        }

        index += node.repeat;
    }

    sum
}


fn main() {
    let binding = fs::read_to_string( "input.txt" )
        .expect( "Failed to read file." );
    let contents = binding.trim();

    let disk_1 = defrag_1( &contents );
    println!( "(Part 1) The checksum is: {}", calc_checksum( &disk_1 ) );

    let disk_2 = defrag_2( &contents );
    println!( "(Part 2) The checksum is: {}", calc_checksum( &disk_2 ) );
}
