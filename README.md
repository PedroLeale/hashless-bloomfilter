# hashless-bloomfilter

This is my first project in Rust, which was part of my scientific initiation. The main objective of this data structure is to be utilized in Bitcoin forensics, working alongside other data structures such as a search tree. It is a hashless version of a bloom filter designed to optimize performance.

The underlying idea is to leverage the base58-encoded Bitcoin addresses as integers and perform basic mathematical operations instead of using traditional hash functions, with the aim of enhancing performance. However, I was not successful in achieving faster performance compared to regular bloom filters. As a result, I proceeded to the second part of my scientific initiation using a traditional bloom filter, which can be found at this repository: https://github.com/PedroLeale/Hybrid-DataStructure

I have plans to revisit and further optimize this code in the future.