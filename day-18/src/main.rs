use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build_tree(m: &[char]) -> TreeNode {
    let mut stack = vec![vec![]];
    for c in m {
        match c {
            '[' => {
                stack.push(vec![]);
            }
            ']' => {
                let mut parent = stack.pop().unwrap();
                (*stack.last_mut().unwrap()).push(TreeNode {
                    val: -1,
                    right: Some(Rc::new(RefCell::new(parent.pop().unwrap()))),
                    left: Some(Rc::new(RefCell::new(parent.pop().unwrap()))),
                });
            }
            ',' => {}
            _ => {
                (*stack.last_mut().unwrap())
                    .push(TreeNode::new(c.to_string().parse::<i32>().unwrap()));
            }
        }
    }
    stack.pop().unwrap().pop().unwrap()
}

fn parse(arg: &str) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let text: String = fs::read_to_string(arg).unwrap();
    let maps: Vec<Vec<char>> = text
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect();
    let mut results = vec![];
    for m in maps {
        results.push(Some(Rc::new(RefCell::new(build_tree(&m)))));
    }
    results
}

fn add(
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = TreeNode {
        val: -1,
        left,
        right,
    };
    let mut node = Some(Rc::new(RefCell::new(root)));
    loop {
        if explode(&mut node, 1).is_none() && !split(&mut node) {
            break;
        }
    }
    node
}

fn explode(node: &mut Option<Rc<RefCell<TreeNode>>>, depth: i32) -> Option<(i32, i32)> {
    if let Some(n) = node {
        let mut n = n.borrow_mut();
        if depth >= 5
            && n.val == -1
            && n.left.as_ref().unwrap().borrow().val != -1
            && n.right.as_ref().unwrap().borrow().val != -1
        {
            n.val = 0;
            let left = n.left.as_ref().unwrap().borrow().val;
            let right = n.right.as_ref().unwrap().borrow().val;
            n.left = None;
            n.right = None;
            Some((left, right))
        } else if let Some((l_l, l_r)) = explode(&mut n.left, depth + 1) {
            reflect(&mut n.right, l_r, "left");
            Some((l_l, 0))
        } else if let Some((r_l, r_r)) = explode(&mut n.right, depth + 1) {
            reflect(&mut n.left, r_l, "right");
            Some((0, r_r))
        } else {
            None
        }
    } else {
        None
    }
}

fn reflect(node: &mut Option<Rc<RefCell<TreeNode>>>, val: i32, lr: &str) {
    if let Some(n) = node {
        let mut n = n.borrow_mut();
        if n.val != -1 {
            n.val += val;
        } else if lr == "left" {
            reflect(&mut n.left, val, lr);
        } else {
            reflect(&mut n.right, val, lr);
        }
    }
}

fn split(node: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(n) = node {
        let mut n = n.borrow_mut();
        if n.val != -1 {
            if n.val > 9 {
                n.left = Some(Rc::new(RefCell::new(TreeNode::new(n.val / 2))));
                n.right = Some(Rc::new(RefCell::new(TreeNode::new((n.val + 1) / 2))));
                n.val = -1;
                true
            } else {
                false
            }
        } else {
            split(&mut n.left) || split(&mut n.right)
        }
    } else {
        false
    }
}

fn magnitude(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(num) = root {
        let n = num.borrow();
        if n.val != -1 {
            n.val
        } else {
            magnitude(&n.left) * 3 + magnitude(&n.right) * 2
        }
    } else {
        0
    }
}

fn debug(root: &Option<Rc<RefCell<TreeNode>>>, show_log: bool) {
    if !show_log {
        return;
    }
    if let Some(num) = root {
        let n = num.borrow();
        if n.val != -1 {
            print!("{}", n.val);
        } else {
            print!("[");
            debug(&n.left, show_log);
            print!(",");
            debug(&n.right, show_log);
            print!("]");
        }
    }
}

fn part1(arg: &str, show_log: bool) -> i32 {
    let data = parse(arg);
    let mut result = data[0].clone();
    debug(&result, show_log);
    for d in data.into_iter().skip(1) {
        result = add(result, d);
        debug(&result, show_log);
    }
    debug(&result, show_log);
    magnitude(&result)
}

fn part2(arg: &str, show_log: bool) -> i32 {
    let text: String = fs::read_to_string(arg).unwrap();
    let maps: Vec<Vec<char>> = text
        .split_terminator('\n')
        .map(|s| s.chars().collect())
        .collect();
    let mut result = 0;
    for ix in 0..maps.len() {
        for iy in 0..maps.len() {
            if ix == iy {
                continue;
            }
            let y = add(
                Some(Rc::new(RefCell::new(build_tree(&maps[ix])))),
                Some(Rc::new(RefCell::new(build_tree(&maps[iy])))),
            );
            debug(&y, show_log);
            let m = magnitude(&y);
            if show_log {
                println!("  magnitude = {}", m);
            }
            result = result.max(m);
        }
    }
    result
}

fn main() {
    println!("{:?}", part1("./data.txt", false));
    println!("{:?}", part2("./data.txt", false));
}
