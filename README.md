# VecMap<K, V>: the Map API for Vec<(K, V)>
A std::vec::Vec based Map, motivated by the fact that, for some key types,
iterating over a vector can be faster than other methods for small maps.

Most of the operations on this map implementation work in O(n), including
some of the ones that are O(1) in HashMap. However, optimizers can work magic with
contiguous arrays like Vec, and so for small sets (up to 256 elements for integer keys,
for example), iterating through a vector actually yields better performance than the
less branch- and cache-predictable hash maps.

# Features
__VecMap__ provides similar guarantees as HashMap, but does have some light differences: for one, the keys need neither to be hashable nor sortable, just equality is enough.

While I certainly do not encourage it, as long as you don't break unicity, you may even use the unsafe API or other mecanisms to mutate the keys in the map  

Like HashMap, VecMap doesn't guarantee pointer stability: growing capacity may relocate the vector's content, and item removal WILL relocate the last element of the vector. 

# When to use it
You may want to use a typedef to allow yourself to experiment and validate that it's good for your use-case, but as a rule of thumb: if you don't plan on storing more than a hundred elements in your map, but still want to express in your code that it IS a map, you should probably go with a VecMap.

## Why are the iterators over VecMap boxed?
In short: because of closures, feel free to use `map.ìnner()` to get unboxed iterators. But don't worry about the boxing: boxed iteration over a vector is still faster than iteration in a HashMap.