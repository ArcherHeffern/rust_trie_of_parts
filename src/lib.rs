use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Default)]
struct TrieNode<K, V>
where
    K: Hash + Eq + Default,
    V: Hash + Eq + Clone + Default,
{
    children: HashMap<K, TrieNode<K, V>>,
    value: Option<V>,
}

pub struct Trie<K, V>
where
    K: Hash + Eq + Default,
    V: Hash + Eq + Clone + Default,
{
    root: TrieNode<K, V>,
}

impl<K, V> Trie<K, V>
where
    K: Hash + Eq + Default,
    V: Hash + Eq + Clone + Default,
{
    /// Create empty Trie
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    /// Get a copy of the value associated with the key in O(len(key)) time
    pub fn get<I>(&self, key: I) -> Option<V>
    where
        I: IntoIterator<Item = K>,
    {
        self.traverse(key).and_then(|node| node.value.clone())
    }

    /// Check if Trie contains the key in O(len(key)) time
    pub fn contains<I>(&self, key: I) -> bool
    where
        I: IntoIterator<Item = K>,
    {
        self.traverse(key)
            .map_or(false, |node| node.value.is_some())
    }

    /// ### About
    /// Finds the value of the longest entry with prefix key
    ///
    /// ### Example
    /// Assume trie contains the following keys and values of the form (key) -> value
    /// - (four, score, and) -> seven
    /// - (four, score, and, seven) -> years
    ///
    /// The query `best_match ["four", "score", "and", "seven", "years", "ago"]` will return `years`
    pub fn best_match<I>(&self, key: I) -> Option<V>
    where
        I: IntoIterator<Item = K>,
    {
        let mut cur: &TrieNode<K, V> = &self.root;
        let mut cur_match = None;
        for part in key {
            if let Some(v) = cur.children.get(&part) {
                cur = v;
                if let Some(new_match) = cur.value.as_ref() {
                    cur_match.replace(new_match.clone());
                }
            } else {
                break;
            }
        }
        cur_match
    }

    /// Inserts key and value into Trie, overriding any previous value
    pub fn insert<I>(&mut self, key: I, value: V)
    where
        I: IntoIterator<Item = K>,
    {
        let mut cur = &mut self.root;
        for part in key {
            cur = cur.children.entry(part).or_insert(TrieNode::default());
        }
        cur.value = Some(value);
    }

    /// Helper function to traverse the Trie
    fn traverse<I>(&self, key: I) -> Option<&TrieNode<K, V>>
    where
        I: IntoIterator<Item = K>,
    {
        let mut cur: &TrieNode<K, V> = &self.root;
        for part in key {
            match cur.children.get(&part) {
                Some(v) => cur = v,
                None => return None,
            }
        }
        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    fn path_to_key_iter(path: &Path) -> Vec<String> {
        path.components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn test_all() {
        let mut trie: Trie<String, PathBuf> = Trie::new();
        let src_path1 = Path::new("/etc/bin/echos");
        let src_path2 = Path::new("/etc/bin/echo");
        let src_path3 = Path::new("/etc/bin/echo/hello.txt");

        let dest_path1 = PathBuf::from("usr/cat");
        let dest_path2 = PathBuf::from("usr/tar");

        let longer_path1 = Path::new("/etc/bin/echo/hello.txt/jello");
        trie.insert(path_to_key_iter(src_path1), dest_path1.clone());
        trie.insert(path_to_key_iter(src_path3), dest_path2.clone());

        assert!(trie.contains(path_to_key_iter(src_path1)));
        assert!(trie.contains(path_to_key_iter(src_path3)));
        assert_eq!(
            trie.get(path_to_key_iter(src_path1)),
            Some(dest_path1.clone())
        );
        assert_eq!(
            trie.get(path_to_key_iter(src_path3)),
            Some(dest_path2.clone())
        );
        assert_eq!(
            trie.best_match(path_to_key_iter(src_path1)),
            Some(dest_path1.clone())
        );
        assert_eq!(
            trie.best_match(path_to_key_iter(longer_path1)),
            Some(dest_path2.clone())
        );
        assert!(!trie.contains(path_to_key_iter(src_path2)));
    }
}
