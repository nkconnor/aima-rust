#![doc(html_playground_url = "https://play.rust-lang.org/")]
#![feature(trait_alias)]
//! ![Build] ![Crate]
//!
//! [Build]: https://github.com/nkconnor/aima-rust/workflows/build/badge.svg
//! [Crate]: https://img.shields.io/crates/v/aima-rust
//!
//! Rust implementation of programs in [**Artifical Intelligence: A Modern Approach**](http://aima.cs.berkeley.edu/).
//! For more complete examples please see the [Java](http://j) and [Python](http://p) repositories.
//! Rust is a systems programming language that aims to be [fast, reliable, and productive](https://www.rust-lang.org/),
//! and is already being used in major initiatives like HuggingFace's
//! [state of the art NLP tokenizers](https://github.com/huggingface/tokenizers) and Argo AI's [autonomous driving platform](https://www.argo.ai/).
//! It has many features that make it an attractive choice for developing AI systems. One example is
//! the basic safety guarantees provided:
//!
//! 1) Programs that compile in standard Rust are guaranteed to be free of memory corruption. If you weren't aware,
//! memory-corruption results in **undefined program behaviour**.
//! 2) Concurrent mutable access _does not even compile_. This in most cases eliminates concerns of "data races".
//!
//! Furthermore, higher-level abstractions in the language make it possible to develop similar ad-hoc safety guarantees for
//! your own programs.
//!
//!
//! ## Getting Started
//!
//! If you are not already familiar with Rust, this may not be the best starting point. There is an
//! [excellent introductory book here that you can get started with](https://docs.rust-lang.org) and
//! use alongside this repository. The documentation is gentle in the introductory sections but
//! quickly builds up to assume that you have at minimum beginner knowledge of programming in Rust.
//!
//!
//! _Instructions on how to get set-up and use this project as a companion to the book goes here_
//!
//! ## Index of Algorithms
//!
//! | **Figure**  | **Name**                          | **Module Link**
//! |:------------|:----------------------------------|:------------------------------|
//! | 2           | Random-Vacuum-Agent               | `RandomVacuumAgent`           |
//! | 2           | Model-Based-Vacuum-Agent          | `ModelBasedVacuumAgent`       |
//! | 2.1         | Environment                       | `Environment`                 |
//! | 2.1         | Agent                             | `Agent`                       |
//! | 2.3         | Table-Driven-Vacuum-Agent         | `TableDrivenVacuumAgent`      |
//! | 2.7        | Table-Driven-Agent                | [`TableDrivenAgent`](table/index.html)                   |
//! | 2.8         | Reflex-Vacuum-Agent               | `ReflexVacuumAgent`           |
//! | 2.10        | Simple-Reflex-Agent               | `SimpleReflexAgent`           |
//! | 2.12        | Model-Based-Reflex-Agent          | `ReflexAgentWithState`        |
//! | 3           | Problem                           | `Problem`                     |
//! | 3           | Node                              | `Node`                        |
//! | 3           | Queue                             | `Queue`                       |
//! | 3.1         | Simple-Problem-Solving-Agent      | `SimpleProblemSolvingAgent`   |
//! | 3.2         | Romania                           | `romania`                     |
//! | 3.7         | Tree-Search                       | `depth/breadth_first_tree_searc
//! | 3.7         | Graph-Search                      | `depth/breadth_first_graph_sear
//! | 3.11        | Breadth-First-Search              | `breadth_first_graph_search`  |
//! | 3.14        | Uniform-Cost-Search               | `uniform_cost_search`         |
//! | 3.17        | Depth-Limited-Search              | `depth_limited_search`        |
//! | 3.18        | Iterative-Deepening-Search        | `iterative_deepening_search`  |
//! | 3.22        | Best-First-Search                 | `best_first_graph_search`     |
//! | 3.24        | A\*-Search                        | `astar_search`                |
//! | 3.26        | Recursive-Best-First-Search       | `recursive_best_first_search` |
//! | 4.2         | Hill-Climbing                     | `hill_climbing`               |
//! | 4.5         | Simulated-Annealing               | `simulated_annealing`         |
//! | 4.8         | Genetic-Algorithm                 | `genetic_algorithm`           |
//! | 4.11        | And-Or-Graph-Search               | `and_or_graph_search`         |
//! | 4.21        | Online-DFS-Agent                  | `online_dfs_agent`            |
//! | 4.24        | LRTA\*-Agent                      | `LRTAStarAgent`               |
//! | 5.3         | Minimax-Decision                  | `minimax_decision`            |
//! | 5.7         | Alpha-Beta-Search                 | `alphabeta_search`            |
//! | 6           | CSP                               | `CSP`                         |
//! | 6.3         | AC-3                              | `AC3`                         |
//! | 6.5         | Backtracking-Search               | `backtracking_search`         |
//! | 6.8         | Min-Conflicts                     | `min_conflicts`               |
//! | 6.11        | Tree-CSP-Solver                   | `tree_csp_solver`             |
//! | 7           | KB                                | `KB`                          |
//! | 7.1         | KB-Agent                          | `KB_AgentProgram`             |
//! | 7.7         | Propositional Logic Sentence      | `Expr`                        |
//! | 7.10        | TT-Entails                        | `tt_entails`                  |
//! | 7.12        | PL-Resolution                     | `pl_resolution`               |
//! | 7.14        | Convert to CNF                    | `to_cnf`                      |
//! | 7.15        | PL-FC-Entails?                    | `pl_fc_entails`               |
//! | 7.17        | DPLL-Satisfiable?                 | `dpll_satisfiable`            |
//! | 7.18        | WalkSAT                           | `WalkSAT`                     |
//! | 7.20        | Hybrid-Wumpus-Agent               | `HybridWumpusAgent`           |
//! | 7.22        | SATPlan                           | `SAT_plan`                    |
//! | 9           | Subst                             | `subst`                       |
//! | 9.1         | Unify                             | `unify`                       |
//! | 9.3         | FOL-FC-Ask                        | `fol_fc_ask`                  |
//! | 9.6         | FOL-BC-Ask                        | `fol_bc_ask`                  |
//! | 10.1        | Air-Cargo-problem                 | `air_cargo`                   |
//! | 10.2        | Spare-Tire-Problem                | `spare_tire`                  |
//! | 10.3        | Three-Block-Tower                 | `three_block_tower`           |
//! | 10.7        | Cake-Problem                      | `have_cake_and_eat_cake_too`  |
//! | 10.9        | Graphplan                         | `GraphPlan`                   |
//! | 10.13       | Partial-Order-Planner             | `PartialOrderPlanner`         |
//! | 11.1        | Job-Shop-Problem-With-Resources   | `job_shop_problem`            |
//! | 11.5        | Hierarchical-Search               | `hierarchical_search`         |
//! | 11.8        | Angelic-Search                    | `angelic_search`              |
//! | 11.10       | Doubles-tennis                    | `double_tennis_problem`       |
//! | 13          | Discrete Probability Distribution | `ProbDist`                    |
//! | 13.1        | DT-Agent                          | `DTAgent`                     |
//! | 14.9        | Enumeration-Ask                   | `enumeration_ask`             |
//! | 14.11       | Elimination-Ask                   | `elimination_ask`             |
//! | 14.13       | Prior-Sample                      | `prior_sample`                |
//! | 14.14       | Rejection-Sampling                | `rejection_sampling`          |
//! | 14.15       | Likelihood-Weighting              | `likelihood_weighting`        |
//! | 14.16       | Gibbs-Ask                         | `gibbs_ask`                   |
//! | 15.4        | Forward-Backward                  | `forward_backward`            |
//! | 15.6        | Fixed-Lag-Smoothing               | `fixed_lag_smoothing`         |
//! | 15.17       | Particle-Filtering                | `particle_filtering`          |
//! | 16.9        | Information-Gathering-Agent       | `InformationGatheringAgent`   |
//! | 17.4        | Value-Iteration                   | `value_iteration`             |
//! | 17.7        | Policy-Iteration                  | `policy_iteration`            |
//! | 17.9        | POMDP-Value-Iteration             | `pomdp_value_iteration`       |
//! | 18.5        | Decision-Tree-Learning            | `DecisionTreeLearner`         |
//! | 18.8        | Cross-Validation                  | `cross_validation`            |
//! | 18.11       | Decision-List-Learning            | `DecisionListLearner`         |
//! | 18.24       | Back-Prop-Learning                | `BackPropagationLearner`      |
//! | 18.34       | AdaBoost                          | `AdaBoost`                    |
//! | 19.2        | Current-Best-Learning             | `current_best_learning`       |
//! | 19.3        | Version-Space-Learning            | `version_space_learning`      |
//! | 19.8        | Minimal-Consistent-Det            | `minimal_consistent_det`      |
//! | 19.12       | FOIL                              | `FOIL_container`              |
//! | 21.2        | Passive-ADP-Agent                 | `PassiveADPAgent`             |
//! | 21.4        | Passive-TD-Agent                  | `PassiveTDAgent`              |
//! | 21.8        | Q-Learning-Agent                  | `QLearningAgent`              |
//! | 22.1        | HITS                              | `HITS`                        |
//! | 23          | Chart-Parse                       | `Chart`                       |
//! | 23.5        | CYK-Parse                         | `CYK_parse`                   |
//! | 25.9        | Monte-Carlo-Localization          | `monte_carlo_localization`    |
//! ## License
//!
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//!
//!
//! ## Contribute
//!
//! Whether you are just getting started in Rust and AI, or a complete veteran, we would be happy for you to help with project.
//! Note that unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `aima-rust`
//! by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!



pub mod agents;