## Introduction
In distributed systems, leader election is a critical process that ensures coordination and consistency across multiple nodes. One way to implement leader election is through the use of a reliable key-value store, such as `etcd`. In this article, I'll explore how to implement a simple leader election implementation in Rust using an etcd server.

Briefly, the 'leader election' problem in a distributed system addresses the coordination challenge among multiple nodes. The goal is to elect a leader node that makes decisions on behalf of all nodes in a cluster, avoiding conflicts, race conditions, and inconsistencies.

Etcd is a distributed key-value store providing a reliable way to store data across a cluster of nodes (machines), ensuring consistency and availability. Besides the basic CRUD operations on key-value pairs, `etcd` provdes other crucial components such `lease` and distributed shared `lock`. In simple terms, a lease is a mechanism used to manage the lifetime of a key-value pair in the etcd key-value store. You can create a lease with a specific Time-To-Live (TTL) and then associate it with a key. When the lease’s TTL expires, the key is automatically deleted. Similarly, distributed `lock` is used to coordinate access to a shared resource in a distributed system, ensuring that only one node can hold the lock at a time.

These features, when combined, form the core of a leader election service. Version 3 of etcd introduces a leader election [provides](https://etcd.io/docs/v3.5/dev-guide/api_concurrency_reference_v3/) along with the corresponding methods.


# Solution 
The solution is based on the lease acqusition and renewal mechanism.
When a node starts it spawns a separate taks that creates a lease and then calls the method `campaign` with the elase id as a parameter.
If we have multiple nodes trying to get elected as the leader, then only one node is elected and the others are blocked by the `campaign` method until the current leader quits leadership or fails (for example as a result of network partitioning) and the lease expires. As soon as it happens and the lease expires the blocked nodes wake up and retry to acquire leadership.

Once a node acquire leadership, it keep alive its leader status by periodically sending keep-alive request to the `etcd` server to renew the lease so the lease won't expire while other nodes are blocked and wait for another phase of leader election. At the same time nodes observe election proclamations in-order as made by the election’s elected leaders to be aware of the current leader node.





