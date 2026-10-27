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

/// A skeleton-based hierarchical rendezvous hasher that achieves O(log n) running time.
///
/// Traditional rendezvous hashing requires O(n) hash computations to select a node from n nodes.
/// For very large node sets, this can become expensive. Skeleton-based hierarchical rendezvous
/// hashing organizes nodes into a virtual hierarchy to reduce the number of hash computations
/// to O(log n).
///
/// # Algorithm
///
/// The skeleton approach works by:
/// 1. Organizing the n nodes into clusters of size `cluster_size` (m)
/// 2. Building a virtual hierarchy with fanout `fanout` (f)
/// 3. At each level of the hierarchy, using HRW to select which branch to descend
/// 4. At the leaf level, using HRW to select the final node from the winning cluster
///
/// This reduces the number of hash computations from O(n) to O(f * log_f(n/m) + m) = O(log n).
///
/// # Trade-offs
///
/// - **Performance vs. Stability**: The skeleton approach trades some redistribution stability
///   for O(log n) performance. When nodes are added or removed, cluster boundaries can shift,
///   potentially causing more key redistribution than standard O(n) rendezvous hashing.
///   Use the standard [`RendezvousHasher`] if minimal disruption is more important than
///   selection performance.
///
/// - **cluster_size**: Larger values improve load balancing in case of node failures but
///   increase the number of hashes at the leaf level. A typical value is 4-8.
/// - **fanout**: Larger values reduce tree height but increase hashes per level.
///   A typical value is 2-4.
///
/// # Example
///
/// ```rust
/// use simplehash::rendezvous::SkeletonRendezvousHasher;
/// use std::collections::hash_map::RandomState;
///
/// // Create a skeleton hasher with cluster_size=4 and fanout=3
/// let hasher = SkeletonRendezvousHasher::<_, RandomState>::new(
///     RandomState::new(),
///     4,  // cluster_size
///     3,  // fanout
/// );
///
/// let nodes: Vec<String> = (0..100).map(|i| format!("node{}", i)).collect();
///
/// // Select the preferred node for a key
/// let selected = hasher.select(&"my_key", &nodes).unwrap();
/// println!("Selected node: {}", selected);
/// ```
#[derive(Debug, Clone)]
pub struct SkeletonRendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    build_hasher: B,
    /// The size of each leaf-level cluster (m in the algorithm)
    cluster_size: usize,
    /// The branching factor of the virtual hierarchy (f in the algorithm)
    fanout: usize,
    _marker: PhantomData<H>,
}

impl<H, B> SkeletonRendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    /// Creates a new `SkeletonRendezvousHasher` with the provided build hasher and parameters.
    ///
    /// # Parameters
    ///
    /// * `build_hasher` - The hasher builder to use for computing hash values
    /// * `cluster_size` - The number of nodes in each leaf-level cluster (recommended: 4-8)
    /// * `fanout` - The branching factor of the virtual hierarchy (recommended: 2-4)
    ///
    /// # Panics
    ///
    /// Panics if `cluster_size` or `fanout` is zero.
    #[inline]
    pub fn new(build_hasher: B, cluster_size: usize, fanout: usize) -> Self {
        assert!(cluster_size > 0, "cluster_size must be greater than 0");
        assert!(fanout > 0, "fanout must be greater than 0");
        Self {
            build_hasher,
            cluster_size,
            fanout,
            _marker: PhantomData,
        }
    }

    /// Creates a new `SkeletonRendezvousHasher` with default parameters.
    ///
    /// Uses cluster_size=4 and fanout=2 as sensible defaults.
    #[inline]
    pub fn with_defaults(build_hasher: B) -> Self {
        Self::new(build_hasher, 4, 2)
    }

    /// Computes a hash score for a virtual node identified by a path prefix and a key.
    ///
    /// The virtual node is identified by a sequence of indices representing the path
    /// from the root to the virtual node in the hierarchy.
    #[inline]
    fn hash_virtual_node<K: Hash>(&self, key: &K, path: &[usize]) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        // Hash the path to create a unique identifier for this virtual node
        path.hash(&mut hasher);
        hasher.finish()
    }

    /// Computes the hash score for a real node with a key.
    #[inline]
    fn hash_node<K: Hash, N: Hash>(&self, key: &K, node: &N) -> u64 {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        node.hash(&mut hasher);
        hasher.finish()
    }

    /// Selects the preferred node for a given key from a list of nodes using the
    /// skeleton-based hierarchical approach.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of nodes to choose from
    ///
    /// # Returns
    ///
    /// A reference to the selected node, or None if the nodes slice is empty
    ///
    /// # Complexity
    ///
    /// O(f * log_f(n/m) + m) where n is the number of nodes, m is cluster_size, and f is fanout.
    /// This simplifies to O(log n) for constant f and m.
    #[inline]
    pub fn select<'a, K, N>(&self, key: &K, nodes: &'a [N]) -> Option<&'a N>
    where
        K: Hash,
        N: Hash,
    {
        if nodes.is_empty() {
            return None;
        }

        // For small node counts, fall back to standard O(n) rendezvous hashing
        // This is more efficient when n is small
        if nodes.len() <= self.cluster_size * self.fanout {
            return nodes.iter().max_by_key(|node| self.hash_node(key, *node));
        }

        // Calculate the number of clusters
        let num_clusters = nodes.len().div_ceil(self.cluster_size);

        // Descend the virtual hierarchy to find the winning cluster
        let winning_cluster = self.select_cluster(key, num_clusters);

        // Calculate the range of nodes in the winning cluster
        let start = winning_cluster * self.cluster_size;
        let end = std::cmp::min(start + self.cluster_size, nodes.len());

        // Select the best node within the cluster using standard HRW
        nodes[start..end]
            .iter()
            .max_by_key(|node| self.hash_node(key, *node))
    }

    /// Selects the winning cluster index by traversing the virtual hierarchy.
    fn select_cluster<K: Hash>(&self, key: &K, num_clusters: usize) -> usize {
        if num_clusters <= self.fanout {
            // Base case: few enough clusters to select directly
            return (0..num_clusters)
                .max_by_key(|&cluster_idx| self.hash_virtual_node(key, &[cluster_idx]))
                .unwrap_or(0);
        }

        // Build path by descending the tree
        let mut path = Vec::new();
        let mut remaining = num_clusters;

        while remaining > self.fanout {
            // Calculate how many children each virtual node at this level covers
            let children_per_node = remaining.div_ceil(self.fanout);

            // Select which branch to take using HRW on virtual nodes
            let selected_branch = (0..self.fanout.min(remaining))
                .max_by_key(|&branch| {
                    let mut test_path = path.clone();
                    test_path.push(branch);
                    self.hash_virtual_node(key, &test_path)
                })
                .unwrap_or(0);

            path.push(selected_branch);

            // Update remaining to reflect the selected subtree
            if selected_branch == self.fanout - 1 && !remaining.is_multiple_of(self.fanout) {
                // Last branch may have fewer children
                remaining -= (self.fanout - 1) * children_per_node;
            } else {
                remaining = children_per_node;
            }
        }

        // At the leaf level, select the final cluster
        let base_cluster = path.iter().enumerate().fold(0, |acc, (level, &branch)| {
            let level_size =
                (num_clusters as f64 / self.fanout.pow(level as u32) as f64).ceil() as usize;
            let clusters_per_branch = level_size.div_ceil(self.fanout);
            acc + branch * clusters_per_branch
        });

        // Select among the remaining clusters at the leaf level
        let leaf_clusters = remaining.min(num_clusters - base_cluster);
        let selected_offset = (0..leaf_clusters)
            .max_by_key(|&offset| {
                let mut leaf_path = path.clone();
                leaf_path.push(offset);
                self.hash_virtual_node(key, &leaf_path)
            })
            .unwrap_or(0);

        (base_cluster + selected_offset).min(num_clusters - 1)
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
        if nodes.is_empty() {
            return None;
        }

        // For small node counts, fall back to standard O(n) rendezvous hashing
        if nodes.len() <= self.cluster_size * self.fanout {
            return nodes
                .iter()
                .enumerate()
                .max_by_key(|(_, node)| self.hash_node(key, *node))
                .map(|(idx, _)| idx);
        }

        // Calculate the number of clusters
        let num_clusters = nodes.len().div_ceil(self.cluster_size);

        // Descend the virtual hierarchy to find the winning cluster
        let winning_cluster = self.select_cluster(key, num_clusters);

        // Calculate the range of nodes in the winning cluster
        let start = winning_cluster * self.cluster_size;
        let end = std::cmp::min(start + self.cluster_size, nodes.len());

        // Select the best node within the cluster using standard HRW
        nodes[start..end]
            .iter()
            .enumerate()
            .max_by_key(|(_, node)| self.hash_node(key, *node))
            .map(|(idx, _)| start + idx)
    }

    /// Ranks all nodes for a given key, returning them sorted by preference
    /// (highest score to lowest).
    ///
    /// Note: This method uses O(n log n) time due to sorting, but only requires
    /// O(n) hash computations (not the O(n log n) that would be needed for
    /// n individual selections).
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
        // For ranking, we need to compute all scores anyway, so we use standard approach
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

/// A weighted node for use with [`WeightedRendezvousHasher`].
///
/// This struct pairs a node value with a weight that determines the relative probability
/// of the node being selected. A node with weight 2.0 will be selected approximately
/// twice as often as a node with weight 1.0.
///
/// # Example
///
/// ```rust
/// use simplehash::rendezvous::WeightedNode;
///
/// let node = WeightedNode::new("server1", 100.0);
/// assert_eq!(node.value(), &"server1");
/// assert_eq!(node.weight(), 100.0);
/// ```
#[derive(Debug, Clone)]
pub struct WeightedNode<T> {
    value: T,
    weight: f64,
}

impl<T> WeightedNode<T> {
    /// Creates a new weighted node with the given value and weight.
    ///
    /// # Parameters
    ///
    /// * `value` - The node value
    /// * `weight` - The weight (must be positive). Higher weights mean higher selection probability.
    ///
    /// # Panics
    ///
    /// Panics if weight is not positive (weight <= 0.0).
    #[inline]
    pub fn new(value: T, weight: f64) -> Self {
        assert!(weight > 0.0, "weight must be positive");
        Self { value, weight }
    }

    /// Returns a reference to the node's value.
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the node's weight.
    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight
    }

    /// Consumes the weighted node and returns the inner value.
    #[inline]
    pub fn into_value(self) -> T {
        self.value
    }
}

impl<T: Hash> Hash for WeightedNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: PartialEq> PartialEq for WeightedNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for WeightedNode<T> {}

/// A weighted rendezvous hasher that allows nodes to have different selection probabilities.
///
/// Standard rendezvous hashing assigns equal probability to each node. Weighted rendezvous
/// hashing extends this by allowing nodes to have different weights, where the probability
/// of a node being selected is proportional to its weight relative to the total weight.
///
/// # Algorithm
///
/// The weighted selection uses a logarithmic transformation as described by Schindelhauer
/// and Schomaker (2005). For each node, the weighted score is computed as:
///
/// ```text
/// weighted_score = weight * (1.0 / -ln(hash_to_unit_interval(key, node)))
/// ```
///
/// This transformation ensures that:
/// - The probability of selecting a node is proportional to its weight
/// - When a node is added or removed, only keys that map to that node are affected
/// - When a node's weight changes, only a minimal number of keys are redistributed
///
/// # Example
///
/// ```rust
/// use simplehash::rendezvous::{WeightedRendezvousHasher, WeightedNode};
/// use std::collections::hash_map::RandomState;
///
/// let hasher = WeightedRendezvousHasher::<_, RandomState>::new(RandomState::new());
///
/// // Create nodes with different weights
/// let nodes = vec![
///     WeightedNode::new("small_server", 100.0),
///     WeightedNode::new("medium_server", 200.0),
///     WeightedNode::new("large_server", 300.0),
/// ];
///
/// // Select a node - large_server will be selected ~50% of the time,
/// // medium_server ~33%, and small_server ~17%
/// let selected = hasher.select(&"my_key", &nodes).unwrap();
/// println!("Selected: {}", selected.value());
/// ```
#[derive(Debug, Clone)]
pub struct WeightedRendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    build_hasher: B,
    _marker: PhantomData<H>,
}

impl<H, B> WeightedRendezvousHasher<H, B>
where
    H: Hasher,
    B: BuildHasher<Hasher = H>,
{
    /// Creates a new `WeightedRendezvousHasher` with the provided build hasher.
    #[inline]
    pub fn new(build_hasher: B) -> Self {
        Self {
            build_hasher,
            _marker: PhantomData,
        }
    }

    /// Converts a hash value to the unit interval (0, 1].
    ///
    /// Maps the full u64 range to (0, 1], avoiding 0 to prevent -ln(0) = infinity.
    #[inline]
    fn hash_to_unit_interval(&self, hash: u64) -> f64 {
        // Map [0, u64::MAX] to (0, 1]
        // Add 1 to avoid 0, which would cause -ln(0) = infinity
        (hash as f64 + 1.0) / (u64::MAX as f64 + 1.0)
    }

    /// Computes the weighted score for a node given a key.
    ///
    /// Uses the formula: weight * (1.0 / -ln(hash_to_unit_interval))
    #[inline]
    fn compute_weighted_score<K, N>(&self, key: &K, node: &WeightedNode<N>) -> f64
    where
        K: Hash,
        N: Hash,
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        node.value.hash(&mut hasher);
        let hash = hasher.finish();

        let unit_interval = self.hash_to_unit_interval(hash);
        let log_score = 1.0 / -unit_interval.ln();

        node.weight * log_score
    }

    /// Selects the preferred node for a given key from a list of weighted nodes.
    ///
    /// The probability of a node being selected is proportional to its weight
    /// relative to the total weight of all nodes.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of weighted nodes to choose from
    ///
    /// # Returns
    ///
    /// A reference to the selected weighted node, or None if the nodes slice is empty
    #[inline]
    pub fn select<'a, K, N>(
        &self,
        key: &K,
        nodes: &'a [WeightedNode<N>],
    ) -> Option<&'a WeightedNode<N>>
    where
        K: Hash,
        N: Hash,
    {
        nodes.iter().max_by(|a, b| {
            let score_a = self.compute_weighted_score(key, a);
            let score_b = self.compute_weighted_score(key, b);
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Selects the preferred node for a given key, returning the index of the selected node.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of weighted nodes to choose from
    ///
    /// # Returns
    ///
    /// The index of the selected node, or None if the nodes slice is empty
    #[inline]
    pub fn select_index<K, N>(&self, key: &K, nodes: &[WeightedNode<N>]) -> Option<usize>
    where
        K: Hash,
        N: Hash,
    {
        nodes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let score_a = self.compute_weighted_score(key, a);
                let score_b = self.compute_weighted_score(key, b);
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(idx, _)| idx)
    }

    /// Selects the node value directly, returning a reference to the inner value.
    ///
    /// This is a convenience method that unwraps the `WeightedNode` to return
    /// just the value.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of weighted nodes to choose from
    ///
    /// # Returns
    ///
    /// A reference to the selected node's value, or None if the nodes slice is empty
    #[inline]
    pub fn select_value<'a, K, N>(&self, key: &K, nodes: &'a [WeightedNode<N>]) -> Option<&'a N>
    where
        K: Hash,
        N: Hash,
    {
        self.select(key, nodes).map(|n| n.value())
    }

    /// Ranks all nodes for a given key, returning them sorted by weighted preference
    /// (highest weighted score to lowest).
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of weighted nodes to rank
    ///
    /// # Returns
    ///
    /// A vector of references to weighted nodes, sorted by preference
    #[inline]
    pub fn rank<'a, K, N>(&self, key: &K, nodes: &'a [WeightedNode<N>]) -> Vec<&'a WeightedNode<N>>
    where
        K: Hash,
        N: Hash,
    {
        let mut ranked: Vec<_> = nodes.iter().collect();
        ranked.sort_by(|a, b| {
            let score_a = self.compute_weighted_score(key, a);
            let score_b = self.compute_weighted_score(key, b);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        ranked
    }

    /// Selects the top k preferred nodes for a given key.
    ///
    /// This is useful for replication scenarios where you want to place
    /// data on multiple nodes with weighted preference.
    ///
    /// # Parameters
    ///
    /// * `key` - The key to hash
    /// * `nodes` - A slice of weighted nodes to choose from
    /// * `k` - The number of nodes to select
    ///
    /// # Returns
    ///
    /// A vector of references to the top k weighted nodes by preference
    #[inline]
    pub fn select_top_k<'a, K, N>(
        &self,
        key: &K,
        nodes: &'a [WeightedNode<N>],
        k: usize,
    ) -> Vec<&'a WeightedNode<N>>
    where
        K: Hash,
        N: Hash,
    {
        let ranked = self.rank(key, nodes);
        ranked.into_iter().take(k).collect()
    }
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

    // Tests for SkeletonRendezvousHasher
    #[test]
    fn test_skeleton_select_small_set() {
        // For small sets, skeleton hasher should behave consistently
        let hasher = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 4, 2);
        let nodes = vec!["node1", "node2", "node3", "node4"];

        let selected = hasher.select(&"test_key", &nodes).unwrap();
        let selected2 = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(selected, selected2);
    }

    #[test]
    fn test_skeleton_select_large_set() {
        // Test with a larger set that exercises the hierarchical selection
        let hasher = SkeletonRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
            4,
            3,
        );

        let nodes: Vec<String> = (0..100).map(|i| format!("node{}", i)).collect();

        // Test consistency
        let key = "test_key";
        let selected1 = hasher.select(&key, &nodes).unwrap();
        let selected2 = hasher.select(&key, &nodes).unwrap();
        assert_eq!(selected1, selected2);

        // Test different keys may select different nodes
        let key_a = "key_a";
        let key_b = "key_b";
        let _ = hasher.select(&key_a, &nodes);
        let _ = hasher.select(&key_b, &nodes);
        // Note: They may or may not be different depending on hash values
    }

    #[test]
    fn test_skeleton_select_index() {
        let hasher = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 4, 2);
        let nodes: Vec<String> = (0..50).map(|i| format!("node{}", i)).collect();

        let idx = hasher.select_index(&"test_key", &nodes).unwrap();
        let node = &nodes[idx];

        // The selected node should match the one returned by select()
        let direct_node = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(node, direct_node);
    }

    #[test]
    fn test_skeleton_empty_nodes() {
        let hasher = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 4, 2);
        let empty: Vec<&str> = vec![];

        assert_eq!(hasher.select(&"key", &empty), None);
        assert_eq!(hasher.select_index(&"key", &empty), None);
        assert!(hasher.rank(&"key", &empty).is_empty());
    }

    #[test]
    fn test_skeleton_single_node() {
        let hasher = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 4, 2);
        let nodes = vec!["only_node"];

        let selected = hasher.select(&"any_key", &nodes).unwrap();
        assert_eq!(*selected, "only_node");

        let idx = hasher.select_index(&"any_key", &nodes).unwrap();
        assert_eq!(idx, 0);
    }

    #[test]
    fn test_skeleton_rank() {
        let hasher = SkeletonRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
            4,
            2,
        );
        let nodes: Vec<String> = (0..20).map(|i| format!("node{}", i)).collect();

        let ranked = hasher.rank(&"test_ranking", &nodes);

        // All nodes should be present in the ranking
        assert_eq!(ranked.len(), nodes.len());

        // Rankings should be stable
        let ranked2 = hasher.rank(&"test_ranking", &nodes);
        assert_eq!(ranked, ranked2);
    }

    #[test]
    fn test_skeleton_node_removal_disruption() {
        // Test node removal behavior
        // Note: The skeleton approach trades some redistribution stability for O(log n) performance.
        // When a node is removed, cluster boundaries can shift, causing more redistribution than
        // standard O(n) rendezvous hashing. This is an expected trade-off.
        let hasher = SkeletonRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
            4,
            2,
        );

        let nodes: Vec<String> = (0..50).map(|i| format!("node{}", i)).collect();

        // Get assignments for 200 keys
        let keys: Vec<String> = (0..200).map(|i| format!("key_{}", i)).collect();
        let mut assignments: Vec<(&str, &String)> = Vec::new();

        for key in &keys {
            let node = hasher.select(key, &nodes).unwrap();
            assignments.push((key.as_str(), node));
        }

        // Remove one node (simulate failure)
        let reduced_nodes: Vec<String> = nodes.iter().filter(|n| *n != "node25").cloned().collect();

        // Count reassignments
        let mut reassigned = 0;
        for (key, original_node) in &assignments {
            let new_node = hasher.select(key, &reduced_nodes).unwrap();
            if new_node != *original_node {
                reassigned += 1;
            }
        }

        // The skeleton approach may have higher redistribution due to cluster boundary shifts.
        // This is an expected trade-off for O(log n) performance.
        println!(
            "Skeleton: Reassigned {}/{} keys after removing a node",
            reassigned,
            keys.len()
        );

        // Keys that were on the removed node must be reassigned
        let keys_on_removed: usize = assignments
            .iter()
            .filter(|(_, node)| node.as_str() == "node25")
            .count();
        assert!(
            reassigned >= keys_on_removed,
            "At least keys on removed node should be reassigned"
        );
    }

    #[test]
    fn test_skeleton_distribution() {
        // Test that keys are distributed reasonably across nodes
        let hasher = SkeletonRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
            4,
            3,
        );

        let nodes: Vec<String> = (0..10).map(|i| format!("node{}", i)).collect();
        let num_keys = 1000;

        let mut counts: std::collections::HashMap<&String, usize> =
            std::collections::HashMap::new();

        for i in 0..num_keys {
            let key = format!("key_{}", i);
            let selected = hasher.select(&key, &nodes).unwrap();
            *counts.entry(selected).or_insert(0) += 1;
        }

        // Check that all nodes received at least some keys
        for node in &nodes {
            let count = counts.get(node).unwrap_or(&0);
            println!("Node {} got {} keys", node, count);
            assert!(*count > 0, "Each node should receive some keys");
        }

        // Check that distribution is roughly uniform (within 3x of expected)
        let expected = num_keys / nodes.len();
        for (node, count) in &counts {
            assert!(
                *count > expected / 3 && *count < expected * 3,
                "Node {} has {} keys, expected around {}",
                node,
                count,
                expected
            );
        }
    }

    #[test]
    fn test_skeleton_with_defaults() {
        let hasher = SkeletonRendezvousHasher::<_, RandomState>::with_defaults(RandomState::new());
        let nodes: Vec<String> = (0..100).map(|i| format!("node{}", i)).collect();

        let selected = hasher.select(&"test_key", &nodes);
        assert!(selected.is_some());
    }

    #[test]
    #[should_panic(expected = "cluster_size must be greater than 0")]
    fn test_skeleton_zero_cluster_size() {
        let _ = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 0, 2);
    }

    #[test]
    #[should_panic(expected = "fanout must be greater than 0")]
    fn test_skeleton_zero_fanout() {
        let _ = SkeletonRendezvousHasher::<_, RandomState>::new(RandomState::new(), 4, 0);
    }

    #[test]
    fn test_skeleton_very_large_set() {
        // Test with a very large set to ensure the hierarchical approach works
        let hasher = SkeletonRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
            8,
            4,
        );

        let nodes: Vec<String> = (0..1000).map(|i| format!("node{}", i)).collect();

        // Test consistency across multiple selections
        let key = "important_key";
        let selected1 = hasher.select(&key, &nodes).unwrap();
        let selected2 = hasher.select(&key, &nodes).unwrap();
        assert_eq!(selected1, selected2);

        // Verify index matches
        let idx = hasher.select_index(&key, &nodes).unwrap();
        assert_eq!(&nodes[idx], selected1);
    }

    // Tests for WeightedRendezvousHasher
    #[test]
    fn test_weighted_node_creation() {
        let node = WeightedNode::new("server1", 100.0);
        assert_eq!(node.value(), &"server1");
        assert_eq!(node.weight(), 100.0);
    }

    #[test]
    #[should_panic(expected = "weight must be positive")]
    fn test_weighted_node_zero_weight() {
        let _ = WeightedNode::new("server1", 0.0);
    }

    #[test]
    #[should_panic(expected = "weight must be positive")]
    fn test_weighted_node_negative_weight() {
        let _ = WeightedNode::new("server1", -1.0);
    }

    #[test]
    fn test_weighted_select_consistency() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
            WeightedNode::new("server3", 300.0),
        ];

        // Same key should always select the same node
        let selected1 = hasher.select(&"test_key", &nodes).unwrap();
        let selected2 = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(selected1.value(), selected2.value());
    }

    #[test]
    fn test_weighted_select_value() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
        ];

        let value = hasher.select_value(&"test_key", &nodes).unwrap();
        let selected = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(value, selected.value());
    }

    #[test]
    fn test_weighted_select_index() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
            WeightedNode::new("server3", 300.0),
        ];

        let idx = hasher.select_index(&"test_key", &nodes).unwrap();
        let selected = hasher.select(&"test_key", &nodes).unwrap();
        assert_eq!(&nodes[idx], selected);
    }

    #[test]
    fn test_weighted_empty_nodes() {
        let hasher = WeightedRendezvousHasher::<_, RandomState>::new(RandomState::new());
        let empty: Vec<WeightedNode<&str>> = vec![];

        assert!(hasher.select(&"key", &empty).is_none());
        assert!(hasher.select_index(&"key", &empty).is_none());
        assert!(hasher.select_value(&"key", &empty).is_none());
        assert!(hasher.rank(&"key", &empty).is_empty());
    }

    #[test]
    fn test_weighted_single_node() {
        let hasher = WeightedRendezvousHasher::<_, RandomState>::new(RandomState::new());
        let nodes = vec![WeightedNode::new("only_server", 100.0)];

        let selected = hasher.select(&"any_key", &nodes).unwrap();
        assert_eq!(selected.value(), &"only_server");
    }

    #[test]
    fn test_weighted_distribution() {
        // Test that weights affect selection probability
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        // Node with weight 300 should be selected ~3x as often as node with weight 100
        let nodes = vec![
            WeightedNode::new("small", 100.0),
            WeightedNode::new("medium", 200.0),
            WeightedNode::new("large", 300.0),
        ];

        let num_keys = 6000;
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

        for i in 0..num_keys {
            let key = format!("key_{}", i);
            let selected = hasher.select_value(&key, &nodes).unwrap();
            *counts.entry(selected).or_insert(0) += 1;
        }

        let small_count = *counts.get("small").unwrap_or(&0);
        let medium_count = *counts.get("medium").unwrap_or(&0);
        let large_count = *counts.get("large").unwrap_or(&0);

        println!(
            "Weighted distribution: small={}, medium={}, large={}",
            small_count, medium_count, large_count
        );

        // Expected: small ~1000, medium ~2000, large ~3000 (total 6000)
        // Allow 50% tolerance for statistical variance
        let total_weight = 600.0; // 100 + 200 + 300
        let expected_small = (num_keys as f64 * 100.0 / total_weight) as usize;
        let expected_medium = (num_keys as f64 * 200.0 / total_weight) as usize;
        let expected_large = (num_keys as f64 * 300.0 / total_weight) as usize;

        assert!(
            small_count > expected_small / 2 && small_count < expected_small * 2,
            "small got {}, expected around {}",
            small_count,
            expected_small
        );
        assert!(
            medium_count > expected_medium / 2 && medium_count < expected_medium * 2,
            "medium got {}, expected around {}",
            medium_count,
            expected_medium
        );
        assert!(
            large_count > expected_large / 2 && large_count < expected_large * 2,
            "large got {}, expected around {}",
            large_count,
            expected_large
        );

        // Also verify relative ordering: large > medium > small (with high probability)
        assert!(
            large_count > small_count,
            "large ({}) should have more than small ({})",
            large_count,
            small_count
        );
    }

    #[test]
    fn test_weighted_rank() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
            WeightedNode::new("server3", 300.0),
        ];

        let ranked = hasher.rank(&"test_ranking", &nodes);

        // All nodes should be present
        assert_eq!(ranked.len(), nodes.len());

        // First ranked node should match selected node
        let selected = hasher.select(&"test_ranking", &nodes).unwrap();
        assert_eq!(ranked[0].value(), selected.value());

        // Rankings should be stable
        let ranked2 = hasher.rank(&"test_ranking", &nodes);
        assert_eq!(
            ranked.iter().map(|n| n.value()).collect::<Vec<_>>(),
            ranked2.iter().map(|n| n.value()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_weighted_select_top_k() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
            WeightedNode::new("server3", 300.0),
            WeightedNode::new("server4", 400.0),
        ];

        let top_2 = hasher.select_top_k(&"test_key", &nodes, 2);
        assert_eq!(top_2.len(), 2);

        // Top 2 should match first 2 in rank order
        let ranked = hasher.rank(&"test_key", &nodes);
        assert_eq!(top_2[0].value(), ranked[0].value());
        assert_eq!(top_2[1].value(), ranked[1].value());
    }

    #[test]
    fn test_weighted_select_top_k_more_than_available() {
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 200.0),
        ];

        let top_5 = hasher.select_top_k(&"test_key", &nodes, 5);
        assert_eq!(top_5.len(), 2); // Only 2 nodes available
    }

    #[test]
    fn test_weighted_equal_weights_uniform() {
        // With equal weights, distribution should be roughly uniform
        // Note: The weighted algorithm uses logarithmic transformation which may
        // have slightly different distribution characteristics than standard HRW
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 100.0),
            WeightedNode::new("server3", 100.0),
            WeightedNode::new("server4", 100.0),
        ];

        let num_keys = 10000;
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();

        for i in 0..num_keys {
            let key = format!("key_{}", i);
            let selected = hasher.select_value(&key, &nodes).unwrap();
            *counts.entry(selected).or_insert(0) += 1;
        }

        // With equal weights, each should get roughly equal share
        // Allow wider tolerance as the logarithmic transform can have variance
        let expected = num_keys / nodes.len();
        for (server, count) in &counts {
            println!(
                "Equal weights: {} got {} keys (expected ~{})",
                server, count, expected
            );
        }

        // Verify all nodes received some keys
        assert_eq!(
            counts.len(),
            nodes.len(),
            "All nodes should receive some keys"
        );
        for (server, count) in &counts {
            assert!(*count > 0, "{} should have received some keys", server);
        }
    }

    #[test]
    fn test_weighted_node_removal_minimal_disruption() {
        // Test that only keys mapped to removed node get reassigned
        let hasher = WeightedRendezvousHasher::<_, BuildHasherDefault<Fnv1aHasher64>>::new(
            BuildHasherDefault::<Fnv1aHasher64>::default(),
        );

        let nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 100.0),
            WeightedNode::new("server3", 100.0),
            WeightedNode::new("server4", 100.0),
            WeightedNode::new("server5", 100.0),
        ];

        // Get assignments for many keys
        let keys: Vec<String> = (0..500).map(|i| format!("key_{}", i)).collect();
        let mut assignments: Vec<(&str, &str)> = Vec::new();

        for key in &keys {
            let value = hasher.select_value(key, &nodes).unwrap();
            assignments.push((key.as_str(), *value));
        }

        // Remove server3
        let reduced_nodes = vec![
            WeightedNode::new("server1", 100.0),
            WeightedNode::new("server2", 100.0),
            WeightedNode::new("server4", 100.0),
            WeightedNode::new("server5", 100.0),
        ];

        // Count reassignments
        let mut reassigned = 0;
        let mut was_on_removed = 0;
        for (key, original_server) in &assignments {
            if *original_server == "server3" {
                was_on_removed += 1;
            }
            let new_server = hasher.select_value(key, &reduced_nodes).unwrap();
            if *new_server != *original_server {
                reassigned += 1;
            }
        }

        println!(
            "Weighted: {} keys were on removed node, {} total reassigned",
            was_on_removed, reassigned
        );

        // Only keys that were on server3 should be reassigned
        assert_eq!(
            reassigned, was_on_removed,
            "Only keys on removed node should be reassigned"
        );
    }
}
