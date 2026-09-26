```
-- until-EOL / doc comment
2 3 (inline comment) +
```

a word token (in this case, `+`) causes the VM to shift execution to the codeblock associated with it. no such association existing causes an error

how do i define a word (or rather an association with a word) in postfix?
we need
- an word literal for that new association. for now let's assume we place word literals on the stack by enclosing words in single quotes
- a value to be associated with that word literal
- an inbuilt word that actually performs the binding between the word literal and the other value on the stack in the vm. for now let's assume that word is `;`

```
'three' 3 ;
'six' three 2 mul ;
```

now, if i want to print out the value of the word `five`:
```
five print
```

the language itself is postfix but the "metalanguage of token markings" (such as the single quote syntax) doesnt have to be.

basically it's a question of whether we want to separate the lex stage from the run stage. if we're going for strict postfixes we pretty much don't even need a lex stage, we can just execute everything as it goes. a dedicated lex stage would discern an atom word from a word with a particle and perform that two-to-one transformation.

---

when it comes to complex data structures, i also remember this cool idea from a couple of years ago. disregard the particle thing for now and look at this:
```
[      -- mark current stack position as array beginning
  1 2 3
]      -- pop everything until the marker and create a new array

print -- [1, 2, 3]
```


this stack marker-based thing also could be the answer to anonymous functions:
```
'double' {     -- "halt execution" and mark current position in code as start of new function object.
               -- maybe metaprogramming comes in play here? code as data on the stack?
  2 *
} ;            -- pop everything from the stack until the marker into a new function object and associate with word 'double'

3 double print -- 6
```


what about control flow? else branches/JIFs are not very "know-at-lex-time". i guess we can try to get around this with markers one more time:
```
2 2 eq?! <THEN> "two equals two" print! <ELSE> "something went wrong" print! <END> -- special keyword tokens TBD
```

all of this should be easily achievable with "state counters". i did a similar thing with `(dis)` in [awrwydr](https://github.com/ab9st8/awrwydr)

---

open question: how does the VM know to execute a codeblock associated with a word but doesn't try to "execute" a literal value assigned to it?
- maybe it does execute it? maybe "executing" a single value on the stack is equivalent to placing it there? i think that's how Joy works; quoting Wikipedia:
> For instance, the numeral '5' does not represent an integer constant, but instead a short program that pushes the number 5 onto the stack.
- maybe the VM is smart enough to simply know to execute codeblocks.
  - but what if i want to simply recall the codeblock associated with the word and place it on the stack? perhaps to concatenate with another?

