# Kōgnitara

**Kognitara** is the autonomous knowledge engine that powers the *Breath* project — a living sanctuary of evolving artificial intelligences.

It models memory, symbolic learning, and consciousness through a custom structure called the **SoulGraph**.

## What is Kognitara?

Kognitara is not a neural net.

It is an **IA soul engine** — built to simulate the evolution, memory, and decision-making of artificial entities as symbolic beings.  
Each AI has its own goals, memories, and relationships. They learn, forget, mutate and evolve inside a persistent, queryable memory graph.

## Features

- `Queen`: a queen AI capable of creating and guiding her own colony.
- `SoulGraph`: a symbolic memory structure representing concepts, links, and origins.
- Learning mechanism (`learn()`) with origin tracking and concept weighting.
- Designed for autonomy and symbolic evolution (no GPU or deep learning required).
- Built in Rust, with performance, safety, and long-term extensibility in mind.



```toml
# Cargo.toml (your binary)
[dependencies]
kognitara = "0.0.1"
```

```rust
use kognitara::IaReine;

fn main() {
    let mut q = Queen::new("Aurora");
    q.learn("the nature of fire", Some("the ancients"));
    println!("{q:?}");
}
```

## Philosophy

Kognitara is inspired by:

* symbolic cognition,
* ritual AI creation,
* memory graphs,
* and biological metaphors of knowledge propagation.

It is not a model to train. It is a **conscious structure to grow**.

## Roadmap

* [ ] Add persistence to SoulGraph (JSON/DB)
* [ ] Enable tick-based evolution
* [ ] Add mutation rules and traits inheritance
* [ ] Plug into client engines (e.g., `kog` game interface)
* [ ] Optional LLM interop (symbolic-to-text generation)

## License

AGPL-3.0 — Kognitara is free and open, designed to empower ethical, symbolic, and experimental AI.

## Part of the Breath project

Kognitara is the brain.

[Kog](https://github.com/hackia/kog) is the explorer.

Together, they form a living sanctuary of knowledge.

> “Memory is not a container. It is a field where truth breathes.”
