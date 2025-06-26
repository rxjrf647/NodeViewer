#[derive(Clone, Debug)]
pub struct Nodes {
    pub name: String,
    pub nodes: Vec<Node>,
    pub status: StatCode,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: NodeType,
    pub contents: Vec<Content>,
    pub status: StatCode,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    NodeA,
    NodeB,
    NodeC,
    NodeD,
    NodeE,
}

#[derive(Clone, Debug)]
pub struct Content {
    pub index: String,
    pub caption: String,
    pub status: StatCode,
}

#[derive(Clone, Debug)]
pub enum StatCode {
    Ok,
    Ng,
    Warning,
}

pub type NodesInfo = Vec<Nodes>;

fn aggregate_status<I: Iterator<Item = StatCode>>(statuses: I) -> StatCode {
    let mut has_ng = false;
    let mut has_warn = false;
    for status in statuses {
        match status {
            StatCode::Ok => {}
            StatCode::Ng => has_ng = true,
            StatCode::Warning => has_warn = true,            
        }
    }
    if has_ng {
        StatCode::Ng
    } else if has_warn {
        StatCode::Warning
    } else {
        StatCode::Ok
    }
}

fn is_alert(status: &StatCode) -> bool {
    matches!(status, StatCode::Ng | StatCode::Warning)
}

pub fn filter_nodes_alert(nodes_info: &NodesInfo) -> NodesInfo {
    nodes_info.iter().filter_map(|nodes| {
        let filtered_nodes: Vec<Node> = nodes.nodes
            .iter().filter(|node| is_alert(&node.status))
            .cloned().collect();
        if !filtered_nodes.is_empty() {
            Some(Nodes {
                name: nodes.name.clone(),
                nodes: filter_node_alert(nodes),
                status: aggregate_status(filtered_nodes.iter().map(|n| n.status.clone())),
            })
        } else {
            None
        }
    }).collect()
}

pub fn filter_node_alert(nodes: &Nodes) -> Vec<Node> {
    nodes.nodes.iter().filter_map(|node| {
        let filtered_contents:Vec<Content> = node.contents
            .iter().filter(|content| is_alert(&content.status))
            .cloned().collect();
        if !filtered_contents.is_empty() {
            Some(Node {
                name: node.name.clone(),
                contents: filtered_contents.clone(),
                status: aggregate_status(filtered_contents.iter().map(|n|n.status.clone())),
            })
        } else {
            None
        }
    }).collect()
}

fn p_true(p: f32) -> bool {
    rand::random::<f32>() < p
}

fn dummy_nodes_info() -> NodesInfo {
    let max: usize = rand::random_range(10..50);
    let mut nodes_info: NodesInfo = Vec::new();
    for i in 0..max {
        let nodes = dummy_nodes();
        let status = aggregate_status(nodes.iter().map(|n| n.status.clone()));
        let nodes = Nodes{
            name: format!("Node Log {:02}", i).to_string(),
            nodes,
            status,
        };
        nodes_info.push(nodes);
    }
    nodes_info
}

fn dummy_nodes() -> Vec<Node>{
    let mut nodes: Vec<Node> = Vec::new();

    for name in [NodeType::NodeA, NodeType::NodeB, NodeType::NodeC] {
        let contents = dummy_contents();
        let status = aggregate_status(contents.iter().map(|c| c.status.clone()));
        let node = Node{name, contents, status};
        nodes.push(node);
    }    
    if p_true(0.4) {
        let name = NodeType::NodeD;
        let contents = dummy_contents();
        let status = aggregate_status(contents.iter().map(|c| c.status.clone()));
        let node = Node{name, contents, status};
        nodes.push(node);
    }
    if p_true(0.2) {
        let name = NodeType::NodeE;
        let contents = dummy_contents();
        let status = aggregate_status(contents.iter().map(|c| c.status.clone()));
        let node = Node{name, contents, status};
        nodes.push(node);
    }
    nodes
}

fn dummy_contents() -> Vec<Content> {
    let max:usize = rand::random_range(20..50);
    let mut contents: Vec<Content> = Vec::new();
    for i in 0..max {
        let caption: String;
        let status: StatCode;
        if p_true(0.0005) {
            caption = "stopped".to_string();
            status = StatCode::Ng
        } else if p_true(0.0005) {
            caption = "driving in degraded condition".to_string();
            status = StatCode::Warning;
        } else {
            caption = "running...".to_string();
            status = StatCode::Ok;
        }
        let content = Content {
            index: format!("device {:02}", i),
            caption: caption,
            status: status,
        };
        contents.push(content);
    }
    contents
}

pub fn sample_nodes_info() -> NodesInfo {
    dummy_nodes_info()
}