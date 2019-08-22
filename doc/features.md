OmgLang features
================

### Event Driven
OmgLang is at its core a multi treaded event loop. The result of the event may
not run on the same tread as where it was created.

### None blocking
It will be impossible to write blocking code in OmgLang. Yes you can write code
that looks like is blocking like a tight loop of cpu bound work. But the runtime
will pause your execution if feels that time better spent elsewhere. So you as
developer is not responsible to make sure that all your code returns quickly.

### Synchronous and asynchronous
If everything is asynchronous then noting are. In OmgLang you can mix
synchronous code with asynchronous code at will. You as the caller decide if you
want to wait for the result or if want to get a future and continue. Synchronous
coding is the default where the asynchronous have special syntax. OmgLang do not
have the async poisoned effect you have other languages. Everything have to use
a async style or none of it.

### Immutability

### Values passed by deep copy

### Statically typed

### Algebraic data type

### No Exceptions

