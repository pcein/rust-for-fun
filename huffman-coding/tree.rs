
use std::collections::HashMap;

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    freq: i32,
    ch: Option<char>,
    left: Link,
    right: Link
}
    
fn new_node(freq: i32, ch: Option<char>) -> Node {
    Node {
        freq: freq, ch: ch,
        left: None, right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

fn frequency(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;  
    }
    h
}
 
fn assign_codes(p: &Box<Node>, 
                h: &mut HashMap<char, String>,
                s: String ) {

    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            assign_codes(l, h, (s.clone() + "0"));
        }
        if let Some(ref r) = p.right {
            assign_codes(r, h, (s.clone() + "1"));
        }
    }
}
 
fn encode_string(s: &str, h: &HashMap<char, String>) -> String {
    let mut r = "".to_string();
    let mut t:Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    r
}
 
fn decode_string(s: &str, root: &Box<Node>) -> String {

    let mut retval = "".to_string();
    let mut nodeptr = root;

    for x in s.chars() {
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        } else {
            if let Some(ref r) = nodeptr.right {
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root;
        }
    }
    retval
}
            
fn main() {
    let msg = "Huffman coding is fun!";
    let h = frequency(msg);

    let mut p:Vec<Box<Node>> = 
                      h.iter()
                      .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
                      .collect();
    while p.len() > 1 {
        p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
        let a = p.pop().unwrap();
        let b = p.pop().unwrap();
        let mut c = new_box(new_node(a.freq + b.freq, None));
        c.left = Some(a);
        c.right = Some(b);
        p.push(c);
    }

    let root = p.pop().unwrap();
    let mut h:HashMap<char, String> = HashMap::new();

    assign_codes(&root, &mut h, "".to_string()); 
    let enc = encode_string(msg, &h);
    println!("decoded = {:?}", decode_string(&enc, &root));
}
