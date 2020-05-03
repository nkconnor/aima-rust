# Simple Reflexes

Following our question from the last section, hopefully, we have both arrived at the conclusion
that our window agent _does not_ need to store the entire percept sequence to determine its action.
In fact, it is perfectly suited to the same reflex pattern shown in figure 2.8 for the
vacuum cleaner. It can look at a single percept, Sunny or Rainy; and Open or Close respectively.


> __function__ SIMPLE-REFLEX-AGENT(_percept_) __returns__ an action  
> &emsp;__persistent__: _rules_, a set of condition\-action rules  
> 
> &emsp;_state_ &larr; INTERPRET-INPUT(_percept_)  
> &emsp;_rule_ &larr; RULE-MATCH(_state_,_rules_)  
> &emsp;_action_ &larr; _rule_.ACTION  
> &emsp;__return__ _action_  
>
> ---
> __Figure 2.8__ A simple reflex agent. It acts according to a rule whose condition matches the current state, as defined by the percept.


In so far as this agent only has to react to a single percept and not the entire sequence, it is more simple. However,
now instead of a simple table lookup, we have two generic functions that are included: `INTERPRET-INPUT` and `RULE-MATCH`.

We can envision that the API for such an agent might look something like this:

```rust
# use aima_rust::agents::reflex::{self, SimpleReflexAgent};
# use aima_rust::agents::envs::weather::{Window, Weather};
let interpet_input = |percept| percept; // identity function

let rule_match = |percept| match percept {
    Weather::Sunny => Window::Open,
    Weather::Rainy => Window::Close
};

let agent = SimpleReflexAgent::new(interpet_input, rule_match);
```

The actual nuts and bolts to make this work in the same fashion as our prior agent gets a little bit verbose.

```rust
pub struct SimpleReflexAgent<Percept, State, Action, InterpretFn, RuleFn>
where
    InterpretFn: FnOnce(Percept) -> State,
    RuleFn: FnOnce(State) -> Action,
{
    interpret_input: InterpretFn,
    rule_match: RuleFn,
    _phantom: PhantomData<(Percept, State, Action)>,
}
```

Here we have a `struct` that can store our two functions generically over any sort of `Percept`, `State`, and
`Action`. Recall that `State` is derived from the interpret input function. That is a (maybe) lightweight
transformation of the percept to something ready for rule matching. As an example, let's introduce some more
complexity to the environment. Let's say instead of just Sunny and Rainy, it can also be Cloudy, Partly Cloudy,
or even Thunderstorm:

```rust
enum Weather {
    Sunny, Rainy, Cloudy, PartlyCloudy, Thunderstorm
}
```

Many of these sensor readings inevitably will lead to the same rational rule execution in our agent. We can create
a `State` that more succinctly represents the outside world in terms of how the agent may respond.

```rust
enum State{
    Good, Bad
}
```

Now, we can define the `INTERPRET-INPUT` as follows:

```rust
let interpret_input = |percept| match percept {
    Weather::Sunny => State::Good,
    Weather::PartlyCloudy => State::Good,
    Weather::Rainy => State::Bad,
    Weather::Thunderstorm => State::Bad,

    Weather::Cloudy => unimplimented!() // ???
};
``` 

You should notice that `interpret_input(Weather::Cloudy)` is undefined. In fact, if you run that,
the entire program will crash! The thing is, we're not quite smart enough to say if this percept signals
a good state or a bad state. Sometimes cloudiness leads to rain and other times it does not. We also haven't said much about
how often our sensors get a reading or how accurate they are. Let's introduce a third enumeration:

```rust
enum State{
    Good, Bad, Unknown(Weather)
}
```

We can now complete our function by adding a catch-all at the end for percepts we can't bucket into just
Good or Bad.

```rust
let interpret_input = |percept| match percept {
    Weather::Sunny => State::Good,
    Weather::PartlyCloudy => State::Good,
    Weather::Rainy => State::Bad,
    Weather::Thunderstorm => State::Bad,

    _ => State::Unknown(_)
};
```

With our new atmospheric conditions and simplified state interpretation in hand, let's get back to the interesting parts.
Our agent has a singular task of opening and closing the window when appropriate. To do that, we can update the `RULE-MATCH`
function.

```rust
let rule_match = |state| match state {
    State::Good => Window::Open,
    State::Bad => Window::Close,
    State::Unknown => unimplimented!() // ???
};
```

We are left in a similar scenario as last time. How can we choose an action when we are in an undefined state? We could
take the risk-averse path and always close the window whenever we are uncertain about what rule to apply. However, there
will surely be times when the window should be open and the house is getting awful stuffy! Another option would be to
leave things as they are, and if an undefined state comes along, we simply crash. Neither seems to be the makings
of a very reliable agent. 

Instead, what we propose is to allow the agent to return a successful action or an error with the reason
it could not complete its task. This may turn out to be a very useful construct, for example, what if instead of a 
simple closure, we are actually interacting with the [National Weather Service's API](https://www.weather.gov/documentation/services-web-api)
over TCP, and in-the-midst of our work, an underground internet cable is cut? Should we crash then? It could be perhaps
helpful to delegate an agent error to some other part of the system or trigger a phone call to the Window Agent customer
support center. 

These are all reactions that could be feasibly undertaken in the encapsulation of our agent's `rule_match` function, perhaps,
with a default behaviour of closing the window until some sensible percept comes along. However, let's use what seems like
better judgement and go with the approach of the agent being _fallible_. This is an area where Rust starts to shine.
If you haven't yet, go take a look at the [Rust book's chapter on error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

Note too, we haven't included an `impl SimpleReflexAgent` that defines the `run` function like we did in the last section.
Can you derive the implementation and test it with our new functions and sensor readings?