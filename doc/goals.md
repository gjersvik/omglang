OmgLang Goals
=====

Main Goals
----------
OmgLang have 3 main goal in order. These are the hills the language will die on.

### Tread safety
The first and most important goal of OmgLang is to be tread safe. It should be
impossible to write code that is not safe to run on multiple cores. OmgLang
should strive to minimize or eliminate deadlock's and other concurrency hazards.

### Easy to use
There are ways to solve the treading issue that require the developer to learn a
new paradigm like Actors from erlang. Or reactive programming. OmgLang goal is
that coding should not be harder than it need to be. You should not be a
concurrence expert to get started with OmgLang. And complexity should only be
added as you as a developer adds more parallelism into your own code and not
what libraries have decided to use. 

### Multi treaded performance
OmgLang can't just add synchronization everywhere and call it a day. We need to
strive to use the future hyper core processors as efficiently as possible. Even
if that results in some single core inefficiency. 

Preferences
-----------
These are tings we would like to have if we can have them but is willing to
trade away for main goals.

### Single core performance
Even if our main goal is to be multi treaded we should not just waste core
cycles just because we can. Every operation have a power cost. So making sure
OmgLang operations runs fast and efficient is important.

### Play well with others
OmgLang should not live in a vacuum and should have great interoperability story
with other programming languages and remote connection standards. You should not
have to re-implement everything in OmgLang. This very important when it comes to
tooling around OmgLang

None Goals
----------
Stuff that we do not care about.

### Support embedded
OmgLang will not try to run on everything from micro controllers to
supercomputers. OmgLang will need tings like Atomic operation from cpu's so will
only be able to run quite beefy chips. OmgLang is designed for many cores and
will have a none trivial runtime. If you need great treading on embedded look at
Rust.
