//! # Agents
//!
//! This module covers material in Chapters 1 & 2.
//!
//! ## Index of Algorithms
//!
//! | **Figure** | **Name**                          | **Module Link**
//! |:-----------|:----------------------------------|:-------------------------------
//! | 2          | Random-Vacuum-Agent               | `RandomVacuumAgent`                                      |
//! | 2          | Model-Based-Vacuum-Agent          | `ModelBasedVacuumAgent`                                  |
//! | 2.1        | Environment                       | `Environment`                                            |
//! | 2.1        | Agent                             | `Agent`                                                  |
//! | 2.3        | Table-Driven-Vacuum-Agent         | `TableDrivenVacuumAgent`                                 |
//! | 2.7        | Table-Driven-Agent                | [`TableDrivenAgent`](table/index.html)                   |
//! | 2.8        | Reflex-Vacuum-Agent               | `ReflexVacuumAgent`                                      |
//! | 2.10       | Simple-Reflex-Agent               | `SimpleReflexAgent`                                      |
//! | 2.12       | Model-Based-Reflex-Agent          | `ReflexAgentWithState`                                   |
//!
/// # Table Driven Agent
///
/// The table driven agent program is fairly simple to design. We need a function that
/// on each percept, stores the percept, and then resolves an action using a table of
/// percept sequence to action.
///
/// We can use a simple Hash Map to represent the table:
///
/// ```
/// # #![feature(trait_alias)]
/// # use std::collections::HashMap;
/// trait Lookup = Eq + std::hash::Hash;
///
/// type Table<Percept: Lookup, Action> = HashMap<Vec<Percept>, Action>;
/// ```
///
/// This simple data structure may look a little intimidating if you are just getting started with
/// Rust. We've defined a short-hand for `std::collections::HashMap` called `Table` that is generic
/// over the `Percepts` received and `Actions` produced. The trait `Lookup` is defined to satisfy
/// some requirements of the HashMap structure, specifically, that Percepts can be _hashed_ and
/// they can be _compared for equivalence_.
///
/// Now, we'll create some boilerplate to instantiate and run new programs. We'll use a struct
/// to store the percept history and table of sequence to actions.
/// ```
/// # use aima_rust::agents::table::{Lookup, Table};
/// struct TableDrivenAgent<Percept: Lookup, Action>{
///     percepts: Vec<Percept>,
///     table: Table<Percept, Action>
/// }
/// ```
///
/// Next, let's implement our program.
///
/// ```
/// # use aima_rust::agents::table::{Lookup, Table};
///
/// # struct TableDrivenAgent<Percept: Lookup, Action>{
/// #    percepts: Vec<Percept>,
/// #    table: Table<Percept, Action>
/// # }
///
/// impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
///     // Run is called for each percept received from the environment
///     fn run(&mut self, percept: Percept) -> &Action {
///         // Push the percept into our history sequence
///         self.percepts.push(percept);
///         // Lookup an action for the history sequence and return it
///         self.table.get(&self.percepts).unwrap()
///     }
/// }
/// ```
///
/// Great. We've created a table driven agent program. One last thing we forgot to do, we need
/// to be able to create the program given a table!
///
/// ```
/// # use aima_rust::agents::table::{Table, Lookup};
/// # struct TableDrivenAgent<Percept: Lookup, Action>{
/// #     percepts: Vec<Percept>,
/// #     table: Table<Percept, Action>
/// # }
/// impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
///   fn new(table: Table<Percept, Action>) -> Self {
///       TableDrivenAgent {
///           table, percepts: Vec::new()
///       }
///   }
/// }
/// ```
///
/// This will give us a short-hand initialization for programs that looks like
/// `let agent = TableDrivenAgent::new(table)`.
///
/// Let's put our newly defined table driven agent code into practice: imagine that far off in the
/// future windows to our houses are intended to open and close automatically based on the outside
/// conditions. For example, we may open the windows in the Summer to let in fresh air on a sunny day.
/// When it rains, we shut the windows to keep the house dry.
///
/// To keep things simple let's say our agent should open the window when it is sunny and
/// close the window when it is rainy.
///
/// ```
/// // The agent's percepts
/// #[derive(PartialEq, Eq, Hash, Clone)]
/// enum Weather {
///     Sunny, Rainy
/// }
///
/// // The agent's actions
/// enum Window {
///     Open, Close
/// }
/// ```
///
/// We'll create a simple table to deal with the changing outside conditions. First, let's
/// set-up the initial conditions.
///
/// ```
/// # use aima_rust::agents::table::Table;
/// # use aima_rust::agents::envs::weather::{Weather, Window};
/// let mut table = Table::new();
/// table.insert(vec![Weather::Sunny], Window::Open);
/// table.insert(vec![Weather::Rainy], Window::Close);
/// ```
///
/// When our agent boots up, it should be able to perceive the weather and act rationally. We'll
/// be optimistic and test the agent out with a sensor reading of a warm breezy atmosphere.
///
/// ```
/// # use aima_rust::agents::table::{TableDrivenAgent, Table};
/// # use aima_rust::agents::envs::weather::{Weather, Window};
/// # let mut table = Table::new();
/// # table.insert(vec![Weather::Sunny], Window::Open);
/// # table.insert(vec![Weather::Rainy], Window::Close);
/// let mut agent = TableDrivenAgent::new(table);
/// let weather = Weather::Sunny;
/// let action = agent.run(weather);
///
/// assert_eq!(*action, Window::Open)
/// ```
///
/// Ok great the window is doing exactly what it should be. Now let's make it work for more than
/// one time step.
///
/// ```no_run
/// #![feature(trait_alias)]
/// # use std::collections::HashMap;
/// # trait Lookup = Eq + std::hash::Hash;
/// #
/// # type Table<Percept: Lookup, Action> = HashMap<Vec<Percept>, Action>;
/// # struct TableDrivenAgent<Percept: Lookup, Action>{
/// #     percepts: Vec<Percept>,
/// #     table: Table<Percept, Action>
/// # }
/// # impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
/// #     // Run is called for each percept received from the environment
/// #     fn run(&mut self, percept: Percept) -> &Action {
/// #         // Push the percept into our history sequence
/// #         self.percepts.push(percept);
/// #         // Lookup an action for the history sequence and return it
/// #         self.table.get(&self.percepts).unwrap()
/// #     }
/// # }
/// # impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
/// #   fn new(table: Table<Percept, Action>) -> Self {
/// #       TableDrivenAgent {
/// #           table, percepts: Vec::new()
/// #       }
/// #   }
/// # }
///#
/// # // The agent's percepts
/// # #[derive(PartialEq, Eq, Hash, Clone)]
/// # enum Weather {
/// #     Sunny, Rainy
/// # }
///#
/// # // The agent's actions
/// # enum Window {
/// #     Open, Close
/// # }
///#
/// # let mut table = Table::new();
/// loop {
///     // Our table program should be able to react to all sequences. One way to create this table
///     // is to look at the longest sequences and consider what can happen next: sun or rain, and
///     // then add that percept sequence into the table.
///
///     // Find the longest sequences in the table
///     let max_length = table.keys().map(|ks| ks.len()).max().unwrap_or(0);
///     let sequences_to_extend: Vec<Vec<Weather>> = table.keys().filter(|k| k.len() == max_length).cloned().collect();
///
///     // Extend them with the next percept permutations
///     sequences_to_extend.iter().for_each(|percepts| {
///
///       let mut rainy_path = percepts.clone();
///       rainy_path.push(Weather::Rainy);
///
///       let mut sunny_path = percepts.clone();
///       sunny_path.push(Weather::Sunny);
///
///       table.insert(rainy_path, Window::Close);
///       table.insert(sunny_path, Window::Open);
///
///    });
/// }
///```
///
/// If you have been complicit to this point, running the following block in the playground should
/// have resulted in a `KILL TIMEOUT` error, and locally you would have surely exceeded the memory
/// limits of your computer! Our loop, while conceptually correct in creating the agent table for
/// any weather that could happen, is an irrational agent implementation. Recall from the book:
///
/// > It is instructive to consider why the table-driven approach to agent construction is doomed
/// > to failure. Let $P$ be the set of possible percepts and let $T$ be the lifetime of the agent (the total
/// > number of percepts it will receive). The lookup table will contain $\sum_{t=1}^T |P|^t$ entries.
///
/// In our case, we've asked the agent to handle all possible percepts for the rest of time. Sparing the proof,
/// this results in a lookup table that is infinite, and the capacity to do such a computation is
/// impossible. What simple changes could be made to our agent such that it can act rationally according
/// to the weather but not use excessive resources?
///
pub mod table {

    use std::collections::HashMap;

    /// For a percept to be stored in a [`Table`](table.html), they must satisify this trait
    /// constraint. That is, they must implement `Eq` and `Hash`.
    ///
    /// # Examples
    ///
    /// Usually to implement `Lookup`, you only need to add a small amount of boilerplate to
    /// your percepts, let's take the example of Weather:
    ///
    /// ```
    /// #[derive(PartialEq, Eq, Hash)]
    /// enum Weather {
    ///     Sunny, Rainy, Cloudy, PartiallyCloudy, Tornado
    /// }
    /// ```
    ///
    /// The `derive` macro annotations give us an automatic `Lookup` implementation for the
    /// `Weather` percept.
    pub trait Lookup = Eq + std::hash::Hash;

    /// The table in figure 2.8 that holds the action for a given percept sequence
    pub type Table<Percept: Lookup, Action> = HashMap<Vec<Percept>, Action>;

    /// The table driven agent. See the module documentation for a derivation of this
    /// struct and example usage.
    pub struct TableDrivenAgent<Percept: Lookup, Action> {
        percepts: Vec<Percept>,
        table: Table<Percept, Action>,
    }

    impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
        /// Creates a new TableDrivenAgent given the table of Percept Sequence to Action
        pub fn new(table: Table<Percept, Action>) -> Self {
            TableDrivenAgent {
                table,
                percepts: Vec::new(),
            }
        }

        /// Stores the percept and looks up the table specified action for the complete sequence
        /// of percepts received.
        pub fn run(&mut self, percept: Percept) -> &Action {
            self.percepts.push(percept);
            self.table.get(&self.percepts).unwrap()
        }
    }
}

/// # Simple Reflex Agent
///
/// Following our question from the last section, hopefully, we have both arrived at the conclusion
/// that our window agent _does not_ need to store the entire percept sequence to determine its action.
/// In fact, it is perfectly suited to the same reflex pattern shown in figure 2.8 for the
/// vacuum cleaner. It can look at a single percept, Sunny or Rainy; and Open or Close respectively.
pub mod reflex {
    use std::marker::PhantomData;

    pub fn identity<T>(t: T) -> T {
        t
    }

    /// Figure 2.10
    //pub trait SimpleReflexAgent<Percept, State, Action> {
    //    fn interpret_input(percept: Percept) -> State;
    //}

    ///// When the percept is equivalent to the state, we can provide a default implementation with
    ///// `interpret_input` returning the percept.
    //impl<Percept, Action, Agent: SimpleReflexAgent<Percept, Percept, Action>> SimpleReflexAgent<Percept, Percept, Action> for Agent {
    //    fn interpret_input(percept: Percept) -> Percept {
    //        percept
    //    }
    //}

    /// Figure 2.10
    ///
    /// # Examples
    ///
    /// ```
    /// # use aima_rust::agents::reflex::{self, SimpleReflexAgent};
    /// # use aima_rust::agents::envs::weather::{Window, Weather};
    /// let interpet_input = reflex::identity;
    /// let rule_match = |percept| {
    ///     match percept {
    ///         Weather::Sunny => Window::Open,
    ///         Weather::Rainy => Window::Close
    ///     }
    /// };
    ///
    /// let agent = SimpleReflexAgent::new(interpet_input, rule_match);
    ///
    /// ```
    pub struct SimpleReflexAgent<Percept, State, Action, InterpretInput, RuleMatch>
    where
        InterpretInput: FnOnce(Percept) -> State,
        RuleMatch: FnOnce(State) -> Action,
    {
        interpret_input: InterpretInput,
        rule_match: RuleMatch,
        _phantom: PhantomData<(Percept, State, Action)>,
    }

    impl<Percept, State, Action, InterpretInput, RuleMatch>
        SimpleReflexAgent<Percept, State, Action, InterpretInput, RuleMatch>
    where
        InterpretInput: FnOnce(Percept) -> State,
        RuleMatch: FnOnce(State) -> Action,
    {
        pub fn new(interpret_input: InterpretInput, rule_match: RuleMatch) -> Self {
            SimpleReflexAgent {
                interpret_input, rule_match,
                _phantom: PhantomData::default()
            }
        }
    }


}


pub mod envs {
    pub mod weather {
        #[derive(PartialEq, Eq, Hash)]
        pub enum Weather {
            Sunny, Rainy
        }

        #[derive(Debug, PartialEq, Eq)]
        pub enum Window {
            Open, Close
        }
    }
}