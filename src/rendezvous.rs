use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

/// A hasher implementation for the Rendezvous (HRW - Highest Random Weight) hashing algorithm.
///
/// Rendezvous hashing provides a way to consistently distribute keys across a set of nodes,
/// with minimal redistribution when nodes are added or removed. It's particularly useful
/// for distributed systems that need to balance load across multiple servers.
///
/// This implementation works with any hasher that implements `std::hash::Hasher`.
#[derive(Debug, Clone)]
pub struct RendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    build_hasher: B,
    _marker: PhantomData<H>,
}

impl<H, B> RendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    /// Creates a new `RendezvousHasher` with the provided build hasher.
    #[inline]
    pub fn new(build_hasher: B) -> Self {
        Self {
            build_hasher,
            _marker: PhantomData,
        }
    }

    /// Selects the preferred node for a given key from a list of nodes.
    ///
    /// This method computes a hash score for each node combined with the key,
    /// and returns the node with the highest score.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of nodes to choose from
    ///
    /// # Returns
    ///
    /// A reference to the selected node, or None if the nodes slice is empty
    #[inline]
    pub fn select<'a, K, N>(&self, key: &K, nodes: &'a [N]) -> Option<&'a N>
    where
        K: Hash,
        N: Hash,
    {
        nodes
            .iter()
            .enumerate()
            .max_by_key(|(_, node)| {
                let mut hasher = self.build_hasher.build_hasher();
                key.hash(&mut hasher);
                node.hash(&mut hasher);
                hasher.finish()
            })
            .map(move |(_, node)| node)
    }

    /// Selects the preferred node for a given key from a list of nodes, returning the index
    /// of the selected node.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of nodes to choose from
    ///
    /// # Returns
    ///
    /// The index of the selected node, or None if the nodes slice is empty
    #[inline]
    pub fn select_index<K, N>(&self, key: &K, nodes: &[N]) -> Option<usize>
    where
        K: Hash,
        N: Hash,
    {
        nodes
            .iter()
            .enumerate()
            .max_by_key(|(_, node)| {
                let mut hasher = self.build_hasher.build_hasher();
                key.hash(&mut hasher);
                node.hash(&mut hasher);
                hasher.finish()
            })
            .map(|(idx, _)| idx)
    }

    /// Ranks all nodes for a given key, returning them sorted by preference
    /// (highest score to lowest).
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of nodes to rank
    ///
    /// # Returns
    ///
    /// A vector of references to nodes, sorted by preference
    #[inline]
    pub fn rank<'a, K, N>(&self, key: &K, nodes: &'a [N]) -> Vec<&'a N>
    where
        K: Hash,
        N: Hash,
    {
        let mut ranked: Vec<_> = nodes.iter().collect();
        ranked.sort_unstable_by_key(|node| {
            let mut hasher = self.build_hasher.build_hasher();
            key.hash(&mut hasher);
            node.hash(&mut hasher);
            std::cmp::Reverse(hasher.finish())
        });
        ranked
    }
}

// Convenience constructor for using the rendezvous hasher with the standard library's default hasher
pub fn with_default_hasher<H, B>() -> RendezvousHasher<H, B>
where
    H: Hasher + Default,
    B: BuildHasher<Hasher = H> + Default,
{
    RendezvousHasher::new(B::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fnv::Fnv1aHasher64;
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasherDefault;

    #[test]
    fn test_select_with_default_hasher() {
        let hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());
        let nodes = vec!["node1", "node2", "node3", "node4"];

        // The same key should always select the same node
        let node = hasher.select(&"test_key", &nodes).unwrap();
        let node2 = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(node, node2);

        // Different keys may select different nodes
        let node_a = hasher.select(&"key_a", &nodes);
        let node_b = hasher.select(&"key_b", &nodes);
        println!("node_a: {:?}, node_b: {:?}", node_a, node_b);
    }

    #[test]
    fn test_select_with_fnv_hasher() {
        let hasher =
            RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
                Fnv1aHasher64,
            >::default());
        let nodes = vec!["node1", "node2", "node3", "node4", "node5"];

        // Test with multiple keys
        let keys = vec!["key1", "key2", "key3", "key4", "key5"];
        for key in &keys {
            let node = hasher.select(key, &nodes).unwrap();
            println!("Key: {}, Selected node: {}", key, node);
        }

        // Test consistency
        let node1 = hasher.select(&"consistent_key", &nodes).unwrap();
        let node2 = hasher.select(&"consistent_key", &nodes).unwrap();
        assert_eq!(node1, node2);
    }

    #[test]
    fn test_node_removal() {
        let hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());
        let nodes = vec!["node1", "node2", "node3", "node4", "node5"];

        // Get node assignments for 100 keys
        let keys: Vec<String> = (0..100).map(|i| format!("key_{}", i)).collect();
        let mut assignments = Vec::new();

        for key in &keys {
            let node = hasher.select(key, &nodes).unwrap();
            assignments.push((key, *node));
        }

        // Remove one node
        let reduced_nodes = vec!["node1", "node2", "node3", "node4"];

        // Count how many keys got reassigned
        let mut reassigned = 0;
        for (key, original_node) in &assignments {
            let new_node = hasher.select(key, &reduced_nodes).unwrap();
            if *new_node != *original_node {
                reassigned += 1;
            }
        }

        // With 5 nodes, removing 1 should reassign approximately 1/5 of the keys
        println!(
            "Reassigned {}/{} keys after removing a node",
            reassigned,
            keys.len()
        );
        assert!(reassigned > 0);
        // This is a probabilistic test, but we expect around 20% to be reassigned
        assert!(reassigned < 40); // Should be around 20, but adding buffer for randomness
    }

    #[test]
    fn test_rank() {
        let hasher =
            RendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(BuildHasherDefault::<
                Fnv1aHasher64,
            >::default());
        let nodes = vec!["node1", "node2", "node3", "node4"];

        // Get the ranked nodes for a key
        let ranked = hasher.rank(&"test_ranking", &nodes);

        // All nodes should be present in the ranking
        assert_eq!(ranked.len(), nodes.len());

        // First node in ranking should match the selected node
        let selected = hasher.select(&"test_ranking", &nodes).unwrap();
        assert_eq!(ranked[0], selected);

        // Rankings should be stable
        let ranked2 = hasher.rank(&"test_ranking", &nodes);
        assert_eq!(ranked, ranked2);
    }

    #[test]
    fn test_select_index() {
        let hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());
        let nodes = vec!["node1", "node2", "node3", "node4"];

        let idx = hasher.select_index(&"test_key", &nodes).unwrap();
        let node = nodes[idx];

        // The selected node should match the one returned by select()
        let direct_node = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(&node, direct_node);
    }

    #[test]
    fn test_empty_nodes() {
        let hasher = RendezvousHasher::<_, RandomState>::new(RandomState::new());
        let empty: Vec<&str> = vec![];

        assert_eq!(hasher.select(&"key", &empty), None);
        assert_eq!(hasher.select_index(&"key", &empty), None);
        assert!(hasher.rank(&"key", &empty).is_empty());
    }
}
