```
-- comment
2 3 add!
```

every non-literal token means one of
(below "it" means the topmost value from the stack, now removed)
- (1.) `,` RECALLing it :: placing the value associated with a word on the stack. anonymous functions are no different from literal values
- (2.) `!` RECALLing and then DOing it :: this is usually how words will be called. there should also probably be an in-built keyword "do" or something that executes anonymous functions right on the stack. this suffix is `,` + that keyword.
- (3.) ` ` also POINTing to it?? :: we should likely provide a reference type
   * why though? just to allow mutability?
- (4.) ` ` POINTing and then DOing it?? why?


how do i define a word in postfix?
i want a value on the stack... we already got 5 from earlier.
then, do we need another suffix? if we have a "get" suffix (`,`), we should probably also have a "set" suffix. lets say ".".

```
five.
```

now, if i want to print out the value of the word `five`:
```
five, print!

five, 2 mul! ten.

ten, five, add! print! -- 15
```

now that i think about it there is no reason for particles to be strict suffixes of tokens. the language itself is postfix but the "metalanguage of token markings" doesnt have to be.
i can imagine the lexer making its way through sourcecode and encountering, lets say, a prefix instead of a suffix. it creates a new token entry and marks it as whatever it wouldve marked it as if it wouldve been a suffix.

basically it's a question of whether we want to separate the lex stage from the run stage. if we're going for strict prefixes we pretty much don't even need a lex stage, we can just execute everything as it goes. a dedicated lex stage would discern an atom word from a word with a particle and perform that two-to-one transformation.

---

when it comes to complex data structures, i also remember this cool idea from a couple of years ago. disregard the particle thing for now and look at this:
```
[      -- mark current stack position as array beginning
  1 2 3
]      -- pop everything until the marker and create a new array

print. -- [1, 2, 3]
```

that's cool and all, and maybe it could be a built-in kinda deal, but at the same time i want this to be as self-contained and predictable as possible. built-ins aren't that a whole lot.

this stack marker-based thing also could be the answer to anonymous functions:
```
{                -- "halt execution" and mark current position in code as start of new function object.
                 -- maybe metaprogramming comes in play here? code as data on the stack?
  2 mul!
}                -- pop everything from the stack until the marker into a new function object
double.          -- assign to new variable

3 double! print! -- 6
```


what about control flow? else branches/JIFs are not very "know-at-lex-time". i guess we can try to get around this with markers one more time:
```
2 2 eq?! <THEN> "two equals two" print! <ELSE> "something went wrong" print! <END> -- special keyword tokens TBD
```

all of this should be easily achievable with "state counters". i did a similar thing with `(dis)` in [awrwydr](https://github.com/ab9st8/awrwydr)

