we are working on a postfix, stack-oriented language

we are striving for minimalism, human-readability, portability, and, in all of this, admittedly, novelty

### minimalism

one of the central ideas of stack-oriented languages is that their grammar is extremely minimal. in the case of our language we are opting for something as minimalist as

```
  (Word [Particle])*
```

where
- a Word is a raw literal value -- a number, string, boolean, or atom
- a Particle is one of a few inbuilt special one-character words that may endow their preceding words with special meaning. 

in practice we want to achieve no lookahead in the parser (or barely any parsing stage at all). at first glance this grammar seems to accomodate that because a Word without a Particle is not incorrect. perhaps some look*behind* is necessary to endow Words with Particles at parse-time? this is TBD

so minimalism also extends to implementation. much stuff that in other languages would be taken of humanely will here be deferred to a chaotic -- but *deterministic* -- runtime. 

### human-readability

the grammar of a stack oriented language, like the one described above, naturally enforces the use of postfix notation:
```
2 3 + # instead of `2 + 3`

"hello world" print # instead of `print("hello world")` or what have you
```

this may look alien at first but i theorize that in the context of a novel kind of natural language-like, object- (or "subject-") oriented programming paradigm, it can gain a peculiar melody. imagine
- `,` is a Particle that marks a Word the subject of a verb (marks a word the object of a method),
- `.` is a Particle that marks a Word a verb (marks a word a function/method), and
- `;` is short for `,.`,

and keep in mind verbs dont have to have subjects:
```
server, 8080 listen.

stdout, "hello world" print.

stdout,
  "hello %s", "me" format.
  print.

tcp_listener, new; "0.0.0.0:3000"
```

its just an idea, and a bit of a pretentious one at that, but i like it and it looks cool. its interesting to me how something resembling standard programming language "grammar" can emerge naturally from such a minimal base.

code begins to resemble natural language, more specifically SOV (subject-object-verb) word order languages like japanese. fun fact, [this seems to have been explored in the Japanese programming language Mind](https://ja.wikipedia.org/wiki/Mind_(%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0%E8%A8%80%E8%AA%9E))
