# the virtual machine and its various viscera

## overview

as has been noted several times we are dealing with a stack. the VM has a stack which is an array of static size.

it is static because reallocations at large stack sizes could be costly. in any case this should be generic (static and dynamic arrays have the same interface) and not difficult to change.

the array stores values of "literal" type. the literal type is a "tagged union" type and can be one of
- (1) a number (for simplicity we do not (yet) discern between floats and integers)
- (2) a string
- (3) a word (i.e. an atom)
- (4) a function (i.e. a codeblock)

all but (1) and (2) are complex types and as such they cannot be stored in a literal type. what is stored in the literal type then is the pointer to their actual value which is dynamically allocated and garbage-collected.

note we do not provide a boolean type because `true` and `false` are simply ("special") words.

uniquely, i think atoms should have a static lifetime (i.e. should not be garbage-collected and live as long as the VM does). but we'll see.

---

## associations

the VM is capable of storing associations between words and values. a word may be associated with any literal. the visibility and lifetime of these associations is dependent on their context

### name resolution and scoping

note that codeblocks dont provide any "special" syntax for named arguments, but you can make do if you need to:
```
{ a. b. c.
  a, b, add! c, add!
} sum-three.

1 2 3 sum-three! -- binds `a` to 3, `b` to 2, `c` to 1 inside the codeblock, computes sum
print!
```

this however can (and should) be more idiomatically written as
```
{
  add! add!
} sum-three.
```


the big question: lexical or dynamic scope?



