# orange

## What kind of juice do you like?

Welcome dear reader, to the Orange Project! I think you have some questions,
just take a seat while I mix some fruits for you?

### Where "orange" come from?

All that story start with a challenge, an idea we have between coworkers before
our morning coffee. We were talking about [mint](https://www.mint-lang.com/),
and [elm](https://elm-lang.org), and [react](https://reactjs.org/), and the
perfect framework to build perfect webapp. Of course it was just a dream, we
already knew that such framework is impossible, and then we switch to the old
days, with jquery, scriptaculous, and vanilla, and [hyperapp](https://hyperapp.dev/),
and with all this name dropping we tought "why so many frameworks?". Obviously,
the right answer was another question "why we haven't our own framework yet?".

This was the beginning of the challenge: two weeks to build everything, from
scratch. So the challenge began on February the 10th. Let see how far can we go.

### Why "orange"?

(Rust -> Red) -> (Coffee -> Morning -> OrangeJuice) -> Orange

Connected words, you know...

### What is "orange"?

It started as a desire to use [mint](https://www.mint-lang.com), but on Windows.
As we want to explore the concept between mixing [elm](https://elm-lang.org) and
[react](https://reactjs.org), but cannot do it with mint, we decided to do the
same, but with [rust](https://www.rust-lang.org/) as our tooling langage.

So "orange" is like a mix betweeen elm, react, mint, and hyperapp, with components,
modules, stores, heavily fonctional based, FRP in mind.

## Mixing fruits



TODO : make a basic example for orange



## The CLI, your new favorite kitchen

Currently missing (fruits sometimes grow slowly):

* `orange crop`
* `orange harvest`
* `orange dilute`

### `orange init` command

You want to create a new recipe for a brand new juice you have in mind? Let's go!

You just have to use orange init in an empty directory to generate a new juice
which will take the name of the current directory as its own name.

If you specify a name, two possibility:

* You're in an empty directory with the same name, it's perfect! The juice will be
initialize in this directory
* The directory doesn't have the same name, it's ok too, orange will create a new
directory for your new recipe

for people who love command line:

`orange init [JUICE_NAME]`

### `orange pour` command

Served with some ice, or at ambient temperature, as you wish.

With the `pour` subcommand you can launch a server to help you build your new recipe
without using external tools. Orange CLI is an all-in-one ;-)!

Default port: 4213

TODO : talking about some options, but when they'll be ready

command line:

`orange pour [OPTIONS]`

### `orange press` command

TODO : just build it before talking about it sounds a really good idea.

## You want to press fruits with us?

Perfect!

We don't have communication platform yet, but put a PR and we'll see what we can do
together ;-) .
