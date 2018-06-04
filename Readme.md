# Rust CLI Formatter Thingy

One thing I'd really like is to make structured and human-focused output of CLI
apps easy. While trying to come up with a `HumanCliOutput` trait (kinda like
`Display`), I once again came across the fact that this code would be awfully
imperative. So, I tried writing a JSX-like declarative DSL.

What you see here is a weird little experiment just to see if this was possible
with `macro_rules`. Turns out it probably is, but it was too hard for me to do
on my lunch break. So, I settled for less XML-like input. It's still a bad idea
to do this (e.g., because white pace needs to be specially inserted).

Have a look at the example to see what it looks like.

A good implementation of this would use a proc-macro (using the derive macro
trick to work on stable probably).

This is also pretty CLI specific. I hope we can abstract this to be easily
adaptable to other output formats.
