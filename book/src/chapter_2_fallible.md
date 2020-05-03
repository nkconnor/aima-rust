# Fallibility

In the last section, we left off with the idea that our reflex agent cannot always respond with an
action. We could put the [Table Driven Agent](chapter_2_table.md) in the same situation by relaxing 
the constraints of the table somewhat. Let's say it is 1970 and we are the designers of the
Window Agent. At the time, the table approach seems most reasonable, and given our computational
limits we can only generate a table that will that includes percept sequences up to the year 2000. Our
product comes with a lifetime guarantee, so, as part of that we include a little red blinking light
that comes on if the product needs maintenance. If our agent encounters a percept sequence in Y2K, we can
return an error, and allow the parent system to turn on the red blinking light.

Let's bring back our `TableDrivenAgent` implementation:

```rust
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
#   fn new(table: Table<Percept, Action>) -> Self {
#       TableDrivenAgent {
#           table, percepts: Vec::new()
#       }
#   }
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

We can update the `run` signature quite easily to return Rust's `std::result::Result` type. First, let's
create a couple symbols to represent our agent's result and potential errors.

```rust
enum Error {
    RequestedY2K
}

type Result<A> = std::result::Result<A, Error>;
```

Now, let's update the `run` implementation:

```rust
  fn run(&mut self, percept: Percept) -> Result<&Action> {
      self.percepts.push(percept);
      self.table.get(&self.percepts)
        // if the action is found in the table,
        // return Result::Ok(action)
        .map(Ok)
        // elsewise, return Result::Errr(error)
        .unwrap_or(Err(Error::RequestedY2K))
  }
```
 You might be wondering why we could have so naively written
a crashable program in the first place. One of the principles for using Rust was indeed that it _is error 
resilient_! If you've read the Rust book chapter on error handling, then you realize that `panic!` crashing
is in the toolkit of an agent designer. What you might not have known, is that `unwrap()` functions almost
always have a `panic!` clause. When we retrieve an item from the `Table` using `get`, it returns an `Option`.
Unwrap on an option that contains `None` results in a `panic!`. This is always layed out in the documentation 
for a function with a heading **# Panics** as [you can see here](https://doc.rust-lang.org/beta/std/option/enum.Option.html#method.unwrap).

Our table driven agent is now relaxed so that _any_ table can be included and used as a lookup
without allowing the program to crash. A caller of the agent can interpret the result as an action or error
and delegate next steps appropriately. For example,

```rust
agent.run(Weather::Cloudy)
    .map_err(|err| start_red_blinking_light())
```

Well, it's 1960, so this is still a leap-forward compared to the competition. That reminds us, we haven't introduced
fallibility to the Simple Reflex Agent. Can you pause and do that now?