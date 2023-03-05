# Bang

You either shoot the problem in the foot, or shoot yourself in the foot

## Ideas that might be added

* Programıng language that itself is just an AST, single format for the AST, the AST is what's being version tracked, the actual code could be presented in many various ways without issues, everyone has their own viewing tiles, think unison?
* Easy modifiability:  https://macoy.me/blog/programming/SelfModifyingApplications
* Unprintable strings (for secrets) (shreded once deconstructed?)
* Native Pipes (`|>` or `|` as a pipe symbol) (maybe support for branched pipes?)
* Easy nested maps: for easy nested maps within pipes for instances, maybe the following syntax. Could be adapted (Surrounded with one pair of [] means a map, being surrounded with two pairs of [[]] means a map on a map)

```ocaml
[11, 22, 33, 44]
   |> [to_char_arr]
   |> [[fun c: c - 1]]
```

* Easy syntactic sugar for a function which runs itself and passes the output
  of itself to itself, is this some sort of reduce call? Could it be thought of
  as such? Is there a nicer way to represent it? (might be useful for functional game state steping functions/clocks)
* Easy introspection of pipes (think Debug.log/#[derive(Debug)] as a function to map that would print whatever data is in the pipe and return it to the pipe)
* Easy dict creation, easy dict modification, everything is a dict?
* Maybe immutable
* Easy way to break out of nested loops, a way to break out of an if list, without labels/goto
* A single term to create data structures, and the only data structures a user can make are product and sum types, they way they are defined will be the same, a product type is a field of a sum type with only one field in a way
* Add support for branded/nominal types (helpful in simulating state machines maybe? Can have security benefis?)
* Add currying
* Coroutines? https://ayazhafiz.com/articles/23/a-lambda-calculus-with-coroutines-and-heapless-closures
* Generate Train diagrams from the grammar https://en.wikipedia.org/wiki/Syntax_diagram like json?
* Env as variables? Passing Env around?
* Everything is a function? prematives are always functions which if called
  return themselves? (but then how do you ever access them and perform
  operations on them?)


## Todo

- [X] Write simple evaluator
- [X] Evaluate infix arithmetic operations
- [X] Evaluate let statements
- [ ] Evaluate fun definitions
- [ ] Evaluate fun calls
- [ ] Add "(" expr ")"
- [ ] Add pipes

## Resources to draw insperation from

* https://blessed.rs/crates
* https://ayazhafiz.com/articles/22/why-dont-more-languages-offer-flow-typing#what-is-flow-typing
* https://ayazhafiz.com/articles/23/a-lambda-calculus-with-coroutines-and-heapless-closures
* https://www.unison-lang.org/
* https://www.cell-lang.net/index.html
* https://github.com/titzer/virgil
* https://zserge.com/posts/too-many-forths/
* https://janet-lang.org/
* https://wren.io
* https://moonscript.org/
* https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/
* http://wiki.c2.com/?ClosuresAndObjectsAreEquivalent
* https://hirrolot.github.io/posts/why-static-languages-suffer-from-complexity.html
* http://docs.idris-lang.org/en/latest/tutorial/syntax.html
* https://leondaz.io/interpreters-and-wtfs
* https://grain-lang.org/docs/
* https://mlochbaum.github.io/BQN/
* https://github.com/robpike/ivy
* https://github.com/yeslogic/fathom
* https://diataxis.fr/
* https://borretti.me/article/introducing-austral
* https://toitlang.org/
* https://nelua.io/
* https://cs.lmu.edu/~ray/notes/languagedesignnotes/
* paper: The Seven Virtues of Simple Type Theory
* Programming, language : How to implement dependent types in 80 lines of code: https://gist.github.com/Hirrolot/27e6b02a051df333811a23b97c375196
* https://math.andrej.com/2018/08/25/how-to-implement-type-theory-in-an-hour/
* https://ratfactor.com/forth/forth_talk_2023.html
