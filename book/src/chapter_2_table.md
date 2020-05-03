# Table Driven Agent

> __function__ TABLE-DRIVEN-AGENT(_percept_) __returns__ an action  
> &emsp;__persistent__: _percepts_, a sequence, initially empty  
> &emsp;&emsp;&emsp;&emsp;&emsp;&emsp;_table_, a fully specified table of actions, indexed by percept sequences, <br/>
> 
> &emsp;append _percept_ to the end of _percepts_  
> &emsp;_action_ &larr; LOOKUP(_percepts_, _table_)  
> &emsp;__return__ _action_
> 
> ---
> __Figure 2.3__ The TABLE-DRIVEN-AGENT program is invoked for each new percept and returns an action each time. It retains the complete percept sequence in memory.


The table driven agent program is fairly simple to implement. We need a function that
on each percept, stores the percept, and then resolves an action using a lookup table of
the complete percept history.


We can use a simple Hash Map to represent the table:

```rust
# #![feature(trait_alias)]
# use std::collections::HashMap;
trait Lookup = Eq + std::hash::Hash;

type Table<Percept: Lookup, Action> = HashMap<Vec<Percept>, Action>;
```

This data structure may look a little intimidating if you are just getting started with
Rust. We've defined a short-hand for `std::collections::HashMap` called `Table` that is generic
over the `Percepts` received and `Actions` produced. The trait `Lookup` is defined to satisfy
some requirements of the HashMap structure, specifically, that Percepts can be _hashed_ and
they can be _compared for equivalence_.

Now, we'll create some boilerplate to instantiate and run new programs. We'll use a struct
to store the percept history and table of sequence to actions.

```rust
# use aima_rust::agents::table::{Lookup, Table};
struct TableDrivenAgent<Percept: Lookup, Action>{
    percepts: Vec<Percept>,
    table: Table<Percept, Action>
}
```

Next, let's implement our program.

```rust
# use aima_rust::agents::table::{Lookup, Table};
# struct TableDrivenAgent<Percept: Lookup, Action>{
#    percepts: Vec<Percept>,
#    table: Table<Percept, Action>
# }
impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
    // Run is called for each percept received from the environment
    fn run(&mut self, percept: Percept) -> &Action {
        // Push the percept into our history sequence
        self.percepts.push(percept);
        // Lookup an action for the history sequence and return it
        self.table.get(&self.percepts).unwrap()
    }
}
```

Great. We've created a table driven agent program. One last thing we forgot to do, we need
to be able to create the program given a table!

```rust
# use aima_rust::agents::table::{Table, Lookup};
# struct TableDrivenAgent<Percept: Lookup, Action>{
#     percepts: Vec<Percept>,
#     table: Table<Percept, Action>
# }
impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
  fn new(table: Table<Percept, Action>) -> Self {
      TableDrivenAgent {
          table, percepts: Vec::new()
      }
  }
}
```

This will give us a short-hand initialization for programs that looks like
```rust
let agent = TableDrivenAgent::new(table);
```

Let's put our newly defined table driven agent code into practice: imagine that far off in the
future windows to our houses are intended to open and close automatically based on the outside
conditions. For example, we may open the windows in the Summer to let in fresh air on a sunny day.
When it rains, we shut the windows to keep the house dry.

To keep things simple let's say our agent should open the window when it is sunny and
close the window when it is rainy.

```rust
// The agent's percepts
#[derive(PartialEq, Eq, Hash, Clone)]
enum Weather {
    Sunny, Rainy
}
// The agent's actions
enum Window {
    Open, Close
}
```

We'll create a simple table to deal with the changing outside conditions. First, let's
set-up the initial conditions.

```rust
# use aima_rust::agents::table::Table;
# use aima_rust::agents::envs::weather::{Weather, Window};
let mut table = Table::new();
table.insert(vec![Weather::Sunny], Window::Open);
table.insert(vec![Weather::Rainy], Window::Close);
```

When our agent boots up, it should be able to perceive the weather and act rationally. We'll
be optimistic and test the agent out with a sensor reading of a warm breezy atmosphere.

```rust
# use aima_rust::agents::table::{TableDrivenAgent, Table};
# use aima_rust::agents::envs::weather::{Weather, Window};
# let mut table = Table::new();
# table.insert(vec![Weather::Sunny], Window::Open);
# table.insert(vec![Weather::Rainy], Window::Close);
let mut agent = TableDrivenAgent::new(table);

let action = agent.run(Weather::Sunny);
assert_eq!(*action, Window::Open)
```

Ok great, the window is doing exactly what it should be doing. Now let's make it work for more than
one time step. Our table agent should be able to react to all percept sequences. One way to create this table
is to look at the longest sequences in the table and consider what can happen next: sun or rain, and 
then add that percept sequence into the table.

```rust,no_run
# #![feature(trait_alias)]
# use std::collections::HashMap;
# trait Lookup = Eq + std::hash::Hash;
#
# type Table<Percept: Lookup, Action> = HashMap<Vec<Percept>, Action>;
# struct TableDrivenAgent<Percept: Lookup, Action>{
#     percepts: Vec<Percept>,
#     table: Table<Percept, Action>
# }
# impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
#     // Run is called for each percept received from the environment
#     fn run(&mut self, percept: Percept) -> &Action {
#         // Push the percept into our history sequence
#         self.percepts.push(percept);
#         // Lookup an action for the history sequence and return it
#         self.table.get(&self.percepts).unwrap()
#     }
# }
# impl<Percept: Lookup, Action> TableDrivenAgent<Percept, Action> {
#   fn new(table: Table<Percept, Action>) -> Self {
#       TableDrivenAgent {
#           table, percepts: Vec::new()
#       }
#   }
# }
# // The agent's percepts
# #[derive(PartialEq, Eq, Hash, Clone)]
# enum Weather {
#     Sunny, Rainy
# }
# // The agent's actions
# enum Window {
#     Open, Close
# }
# let mut table = Table::new();
loop {
    // Find the longest sequences in the table
    let max_length = table.keys()
        .map(|ks| ks.len())
        .max().unwrap_or(0);
    
    // & all sequences of that length 
    let sequences_to_extend: Vec<Vec<Weather>> = table.keys()
        .filter(|k| k.len() == max_length)
        .cloned()
        .collect()

    // Extend them with the next percept permutations
    // and add back into the table
    sequences_to_extend.iter().for_each(|percepts| {
      let mut rainy_path = percepts.clone();
      rainy_path.push(Weather::Rainy);
      table.insert(rainy_path, Window::Close);

      let mut sunny_path = percepts.clone();
      sunny_path.push(Weather::Sunny);
      table.insert(sunny_path, Window::Open);
   });
}
```

If you have been complicit to this point, running the preceding block in the playground should
have resulted in a `KILL TIMEOUT` error, and locally you would have surely exceeded the memory
limits of your computer! Our loop, while conceptually correct in creating the agent table for
any weather that could happen, is an irrational agent implementation. Recall from the book:

> It is instructive to consider why the table-driven approach to agent construction is doomed
> to failure. Let \\(P\\) be the set of possible percepts and let \\(T\\) be the lifetime of the agent (the t
> number of percepts it will receive). The lookup table will contain \\(\sum_{t=1}^T |P|^t\\) entries.

In our case, we've asked the agent to handle all possible percepts for the rest of time. Sparing the 
proof this results in a lookup table that is infinite, and the capacity to do such a computation is
impossible. What simple changes could be made to our agent such that it can act rationally according
to the weather but not use excessive resources?