use std::{collections::HashMap, hash::Hash}; 


struct TrieNode<K, V> 
where K: Hash + Eq + From<String>, V: Hash + Eq + Clone
{
    children: HashMap<K, TrieNode<K, V>>,
    value: Option<V>,
}

impl <K, V>TrieNode<K, V> 
where K: Hash + Eq + From<String>, V: Hash + Eq + Clone
{
    pub fn create_default() -> Self {
        TrieNode {children: HashMap::new(), value: None}
    }
}
pub struct Trie<K, V> 
where K: Hash + Eq + From<String>, V: Hash + Eq + Clone
{
    root: TrieNode<K, V>
}

impl <K, V>Trie<K, V> 
where K: Hash + Eq + From<String>, V: Hash + Eq + Clone
{
    pub fn new() -> Self {
        Trie { root: TrieNode::create_default() }
    }

    pub fn get<I>(&self, key: I) -> Option<V> 
    where I: IntoIterator<Item = K>
    {
        let mut cur: &TrieNode<K, V> = &self.root;
        let key = key.into_iter();
        for part in key {
            match cur.children.get(&part) {
                Some(v) => cur = v,
                None => return None
            }
        }
        cur.value.clone()
    }

    pub fn contains<I>(&self, key: I) -> bool 
    where I: IntoIterator<Item = K>
    {
        self.get(key).is_some()
    }

    pub fn best_match<I>(&self, key: I) -> Option<V> 
    where I: IntoIterator<Item = K>
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

    pub fn insert<I>(&mut self, key: I, value: V) 
    where I: IntoIterator<Item = K>
    {
        let mut cur = &mut self.root;
        for part in key {
            cur = cur.children.entry(part).or_insert(TrieNode::create_default());
        }
        cur.value = Some(value);
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
        assert_eq!(trie.get(path_to_key_iter(src_path1)), Some(dest_path1.clone()));
        assert_eq!(trie.get(path_to_key_iter(src_path3)), Some(dest_path2.clone()));
        assert_eq!(trie.best_match(path_to_key_iter(src_path1)), Some(dest_path1.clone()));
        assert_eq!(trie.best_match(path_to_key_iter(longer_path1)), Some(dest_path2.clone()));
        assert!(!trie.contains(path_to_key_iter(src_path2)));
    }
}
