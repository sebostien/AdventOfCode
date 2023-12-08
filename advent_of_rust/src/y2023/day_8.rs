pub fn get_solution() -> crate::Solution<usize, u64> {
    crate::Solution {
        date: (2023, 8),
        part_1: Box::new(part_1),
        part_2: Box::new(part_2),
        answer: (18_827, 20_220_305_520_997),
    }
}

const START: Node = Node::from_bytes(&[b'A', b'A', b'A']);
const GOAL: Node = Node::from_bytes(&[b'Z', b'Z', b'Z']);

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
struct Node(u32);

impl Node {
    const fn from_bytes(value: &[u8]) -> Self {
        Self(u32::from_be_bytes([0, value[0], value[1], value[2]]))
    }

    const fn is_start(self) -> bool {
        (self.0 & 0xFF) == (b'A' as u32)
    }

    const fn is_goal(self) -> bool {
        (self.0 & 0xFF) == (b'Z' as u32)
    }
}

struct Map {
    inst: Vec<bool>,
    nodes: Vec<(Node, (Node, Node))>,
}

impl std::str::FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let inst = lines.next().unwrap().chars().map(|c| c == 'R').collect();
        let mut nodes = Vec::new();

        for line in lines.filter(|s| !s.is_empty()) {
            let (id, paths) = line.split_once(" = ").unwrap();
            let (left, right) = paths.split_once(',').unwrap();
            let left = left
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .collect::<String>();
            let right = right
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .collect::<String>();

            nodes.push((
                Node::from_bytes(id.as_bytes()),
                (
                    Node::from_bytes(left.as_bytes()),
                    Node::from_bytes(right.as_bytes()),
                ),
            ));
        }

        Ok(Self { inst, nodes })
    }
}

fn part_1(input: &str) -> anyhow::Result<usize> {
    let map: Map = input.parse()?;
    let mut steps = 0;
    let mut curr = &START;
    while curr != &GOAL {
        for &i in &map.inst {
            let (_, (l, r)) = map.nodes.iter().find(|(c, _)| c == curr).unwrap();
            if i {
                curr = r;
            } else {
                curr = l;
            }
        }
        steps += map.inst.len();
    }

    Ok(steps)
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn part_2(input: &str) -> anyhow::Result<u64> {
    let map: Map = input.parse()?;
    let currs = map
        .nodes
        .iter()
        .filter_map(|(n, _)| if n.is_start() { Some(*n) } else { None })
        .collect::<Vec<_>>();

    let mut paths = vec![];
    for mut curr in currs {
        let mut path_len = 0;
        for &i in map.inst.iter().cycle() {
            path_len += 1;
            let (_, (l, r)) = map.nodes.iter().find(|(c, _)| *c == curr).unwrap();
            if i {
                curr = *r;
            } else {
                curr = *l;
            }
            if curr.is_goal() {
                paths.push(path_len);
                break;
            }
        }
    }

    // Find smallest multiple
    Ok(paths
        .into_iter()
        .reduce(|a, b| (a * b) / gcd(a, b))
        .unwrap())
}
