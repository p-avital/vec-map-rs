# VecMap<K, V>: the Map API, but with linear search
A std::vec::Vec based Map, motivated by the fact that, for some key types,
iterating over a vector can be faster than other methods for small maps.

Most of the operations on this map implementation work in O(n), including
some of the ones that are O(1) in HashMap. However, optimizers can work magic with
contiguous arrays like Vec, and so for small sets (up to 256 elements for integer keys,
for example), iterating through a vector actually yields better performance than the
less branch- and cache-predictable hash maps.

# Features
__VecMap__ provides similar guarantees as HashMap, but does have some light differences: for one, the keys need neither to be hashable nor sortable, just equality is enough.

Like HashMap, VecMap doesn't guarantee pointer stability: growing capacity may relocate the vector's content, and item removal WILL relocate the last element of the vector. 

# When to use it
You may want to use a typedef to allow yourself to experiment and validate that it's good for your use-case, but as a rule of thumb: if you don't plan on storing more than a hundred elements in your map, but still want to express in your code that it IS a map, you should probably go with a VecMap.

# How does it compare with `linear_map`
While it is very similar (after all, we share the same API), there is one key difference: `linear_map`'s internal structure is a `Vec<(K, V)>`, whereas `vector_map` uses `struct {keys: Vec<K>, values: Vec<V>}`.

Considering that the most common operation for both these implementations is the linear search for a key, `VecMap` has the advantage of packing its keys tighter, requiring fewer cache requests for the same number of keys tested.

This makes `VecMap` slightly faster than `LinearMap` for some operations, especially when `V` is much bigger than `K`. However, you should still test both for your own application to see which is more suited to your application.

# You use contracts, do I pay for them?
Not unless you specifically enable them, using this crate's `enable_contracts` feature. Since most of the contracts need to check if the map contains a key, they would otherwise each run their own key search, which is not a very efficient thing to do.