[![Build Status](https://travis-ci.org/skreimeyer/civil.svg?branch=master)](https://travis-ci.org/skreimeyer/civil)
[![codecov](https://codecov.io/gh/skreimeyer/civil/branch/master/graph/badge.svg)](https://codecov.io/gh/skreimeyer/civil)

__This is a work in progress__

# Civil

This package is meant to provide solutions to common civil engineering problems. Everything is very much a work in progress. Ideally, on maturity, this library should support developers interested in making applications that assist with tasks like determining the geometric design of streets, sizing culverts and basic economic analysis for construction projects. The ultimate goal is to have a simple API for common civil engineering design equations so that an application developer can focus more on application-specific implementation rather than on researching methods. The aim of Civil is to be comprehensive enough to be useful, but simple enough to be flexible and lightweight. Consequently, complex tasks, like reading in digital elevation models, delineating watersheds and transforming geometric objects to be written to some very vendor-specific file types (I'm looking at you .DWG) is out of scope for this project.

## Why?

> Why this library?

1. There are no similar FOSS libraries for Rust that fill this role.

> Why make a library at all?

2. The freely available software tools that are intended to support developers are multitude. The tools available to support other industries are much fewer. This makes sense because most of the software is written by programmers, and programmers deal with (and see solutions for) problems that impact developers rather than those problems that affect the Architecture-Engineering-Construction world. Logically, what you are left with are about a trillion repositories on GitHub for developers' problems (and novel forms of entertainment ie **games**), and approximately zero for engineer's problems. I hope this is a step in the direction of closing that gap.

> Why make a library at all, but this time without your snide annotations?

3. Engineers and designers solve important tangible problems. Modern infrastructure like water and sanitation enable a quality of life that was impossible just a few hundred years ago. Lowering the barriers to providing these benefits to more people makes the world a better place. Robust, freely available software lowers the barriers.

> Why Rust?

4. Rust provides a combination of safety and performance guarantees that are hard to find elsewhere (or at least, never achieved much popularity). I think the need for total reliability of engineering design software should be self-explanatory.

## The Road to 0.1.0
Checklist:
- ~~Scaffold for finished program is near complete. Modules have a sensible layout and major logical groupings have been made.~~
- ~~Module files properly reference other shell module files~~
- ~~At least one useful thing exposed~~
- Sensible TO-DOs where they belong for reference

## The Road to 1.0.0
Checklist:
- All essential modules and functions have been written
- 100% function documentation
- 90% code coverage or better
- Partial benchmarking of functions to support implementation style
- All unstable features are migrated to a development.

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
