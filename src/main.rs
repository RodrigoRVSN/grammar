use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref RULES: HashMap<&'static str, (String, String)> = {
        let mut rules = HashMap::new();
        rules.insert("S", ("b".to_string(), "aSc".to_string()));
        rules
    };
}

fn main() {
    let root = "S";

    substitution(root, 0);
}

// todo: change it to non alphabetical sentences
fn is_lowercase(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_lowercase() {
            return false;
        }
    }
    true
}

fn substitution(root: &str, current_try: i32) {
    for (key, (rule1, rule2)) in RULES.iter() {
        if is_lowercase(root) {
            println!("Chain: {}", root);
        }
        if root.contains(key) {
            let current = current_try + 1;
            if current == 20 {
                return;
            }
            let new_root1 = root.replace(key, rule1);
            let new_root2 = root.replace(key, rule2);
            substitution(&new_root1, current);
            substitution(&new_root2, current);
        } else {
            // println!("{} is not a sentence!", root)
        }
    }
}
