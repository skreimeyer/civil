[![Build Status](https://travis-ci.org/skreimeyer/civil.svg?branch=master)](https://travis-ci.org/skreimeyer/civil)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![](https://img.shields.io/crates/d/civil.svg?style=flat)

~~__This is a work in progress__~~
 __This is no longer under active development__

Here's my heartache-filled blogpost:
I started this project to
1. study for the PE exam in a novel way
2. build proficiency in Rust.
+ Because Rust ostensibly provides security other languages don't.
+ Because Rust is high performance with "zero cost abstractions"
+ Because Rust is "The C++ killer"; it's the future. Just RIIR, man.
+ Because I thought the frustration would all be worth it when some kid in the boonies could learn all about how bridges are built from some website built off this library compiled to WASM and dolled up with D3.

So it turns out there aren't any of the critical linear algebra libraries needed for solvers, etc. that are native to Rust. So I'm fighting a no-compromise compiler for the right to smugly say, "I am 100% confident this software will work, bug-free," which now has to have a big asterisk next to that statement. I have total confidence in BLAS and LAPACK for being error-free, but if you're forcing yourself into a *language* level guarantee that certain errors *can't* happen, then falling back to "dude, just trust me. The person who wrote this dependency is really smart" seems like it betrays the entire point. If you trust the smart people behind BLAS, why not the smart people who wrote gcc? Why not trust some random dude's npm package? And it's not even just that I'm mad that somebody didn't lay 100 man-years of linear algebra libraries at my feet in the language that I want; Rust's standard library regularly uses 'unsafe' operations for performance, apparently, so why am I stressing over safety and purity again?

Chalk it up to my ignorance and inexperience (I'm not a professional dev), but this code snippet is what was the last straw.
```
let mut augmented_matrix: Vec<Vec<f32>>;
  for (index,row) in matrix.iter().enumerate() {
      let mut the_unit = dbg!(unit_matrix[index].clone());
      let mut tmp = dbg!(row.clone());
      // Because why would this obvious operation ever work?: let combined = dbg!(tmp.extend(the_unit));
      let combined = dbg!(tmp.append(&mut the_unit));
  ```
So I try to implement my own linear algebra solver, right? Nothing fancy, just a Gauss-Jordan elimination algorithm. You've seen it on Rosetta Code, I'm sure. Above is the part that took the last of my patience. A matrix is nothing extraordinary, so I use nested vectors. That seems obvious, right? I tried creating the augmented matrix in a fancy map operation. No luck. Then I spent an entire evening breaking this thing down into progressively simpler parts and littering the code with dbg! and println! statements everywhere. In the code above, `the_unit` is a row from a unit vector, like [1,0]. `tmp` is a row from our input matrix, maybe [1,2]. They're cloned, so ownership shouldn't be an issue. Dbg! says 'I'm a vector'. What do we get if we append or extend one vector to another? `()`. Not a compiler error. Not a warning. The operation, which should only ever return a vector, returns nothing, and I don't find out until I get type-mismatch errors further in the code.

I desperately miss languages that usually behave in obvious ways. I've never even been that bothered by having to use a debugger to play detective in Python. If I'm not *really* making good on the whole Rust safety guarantee anyway (because everything that matters is `unsafe` or an FFI to a C/C++ library), and I'm a couple months into this language and not close to having a single useful product, and every few lines of code is a new litany of compiler errors until I have internalized vast knowledge about how the compiler is manipulating memory, then I think it's time to walk away from this sunk cost. I'm not going to get too deep into why all this TIMTOWTDI nonsense very cleverly shown [here](http://antoyo.ml/evolution-rust-programmer) is such a problem. If you need to read equally valid by wildly different code styles up to having to learn some procedural macro DSL just to understand your dependencies, then you probably won't ever bother to understand or help maintain those packages. Oh, and you will have an **absurd** number of dependencies if you do anything complex or network-based because the core developers decided a weak standard library can be shored up by a package manager that makes publication low-effort. If you think that this is a repetition of javascript and npm's mistakes, then you would be right.

I'm done griping. I'll try again with something more fun to write in. Right now that looks like Go.

I did like the crab mascot.
---

# Civil

This package is meant to provide solutions to common civil engineering problems. Everything is very much a work in progress. Ideally, on maturity, this library should support developers interested in making applications that assist with tasks like determining the geometric design of streets, sizing culverts and basic economic analysis for construction projects. The ultimate goal is to have a simple API for common civil engineering design equations so that an application developer can focus more on application-specific implementation rather than on researching methods.

A good example of the intended effect would be if a web developer could write a short function in Rust that uses this API to, say, calculate the flow of water in an open channel given channel dimensions and roughness, then compile that to WASM and call that function from JS with user inputs to produce a graphical representation of the channel in a `<canvas></canvas>` element. Another good use case would be the functions in this library being used to support a module or extension in a CAD suite.

The aim of Civil is to be comprehensive enough to be useful, but simple enough to be flexible and lightweight. Consequently, complex tasks, like reading in digital elevation models, delineating watersheds and transforming geometric objects to be written to some very vendor-specific file types (I'm looking at you .DWG) is out of scope for this project.

## Why?

> Why this library?

1. There are no similar FOSS libraries for Rust that fill this role.

> Why make a library at all?

2. Engineers and designers solve important, tangible problems. Modern infrastructure like water and sanitation enable a quality of life that was impossible just a few generations years ago. Lowering the barriers to providing these benefits to more people makes the world a better place. Robust, freely available software lowers the barriers, and currently there are not the multitude of tools available such as those for problem domains like, dev-ops, for example.

> Why Rust?

3. Rust provides a combination of safety and performance guarantees that are hard to find elsewhere (or at least, never achieved much popularity). I think the need for total reliability of engineering design software should be self-explanatory.

## The Road to 0.1.0
Checklist:
- ~~Scaffold for finished program is near complete. Modules have a sensible layout and major logical groupings have been made.~~
- ~~Module files properly reference other shell module files~~
- ~~At least one useful thing exposed~~
- ~~Sensible TO-DOs where they belong for reference~~

## The Road to 1.0.0
Checklist:
- All essential modules and functions have been written
- 100% function documentation
- 90% code coverage or better
- Partial benchmarking of functions to support implementation style
- All unstable features are migrated to a development branch.

## Beyond!
Checklist:
- Usage examples: civil as an API for simple CLI tools, WASM targets, etc.
- Markdown book to clarify usage
- Optimization
- New features

## Contributing
There is basically no aspect of this project where contribution isn't welcomed. If you see a gap in the code, make a pull request. I'm not a professional developer, so if you see some pretty obvious meta-programming faux pax, bring it up.

Things this project needs:
- Everything.
- Tests
- Benchmarks
- Documentation

> What if I don't know anything about civil engineering?

There are some [excellent texts](https://www.amazon.com/Civil-Engineering-Formulas-Tyler-Hicks/dp/0071614699) that thoroughly list most of the functions appropriate for this library. You don't necessarily need to fully understand their context and usage as long as you can translate an equation into rust with sensible inputs and outputs.

The Do's:
1. The file hierarchy should be clear and have intuitive organization based on logical groupings of tasks.
2. Clear documentation should be provided with every function or struct.
3. Clear documentation also means citation. If you add a function, then also provide a source. IEEE style citations are always good.
4. The build target is stable Rust. We all like nightly features, but the end user should not be forced to abandon the stable release to use this library.
5. Modules should be as independent as possible.
6. Write tests.
7. Formulas should be unit-system agnostic where possible, and should handle differing unit systems (imperial or metric) appropriately. The preference would be for functions to have their unit system as a suffix. There may be a cleaner implementation for this, but for now, explicit seems safer.

The Dont's:
1. Do not introduce dependencies that are not actually necessary. Using the new hotness in async should be left up to the end user. Very few of these functions actually require anything more sophisticated than High School algebra and geometry.
2. Do not create interdependencies between top-level modules. Your geotech functions shouldn't break your hydrology functions.
