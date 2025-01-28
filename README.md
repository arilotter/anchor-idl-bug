# weird idl error with const generics

to repro:
`anchor build` throws 
```
error[E0573]: expected type, found constant `SOME_CONST`
  --> programs/idl-repro/src/lib.rs:23:31
   |
23 |     pub checkpointers: Foo<I, SOME_CONST>,
   |                               ^^^^^^^^^^ not a type
```

but if you replace `SOME_CONST` with the literal `16`, there's no error.