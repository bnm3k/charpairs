# CharPairs Language

Simple Language to explore recursive descent parsing.

The grammar is as follows:

```
Node -> Char | Pair
Char -> Any character except '(' ')' or ' '
Pair -> '(' Node ' ' Node ')'
```

It's simple with tokens consisting of single characters, there is no
left-recursive production rules and at most we need to look-ahead 1 token. The
output is a pretty-printed parse tree.

## Usage

At the project's root, run `cargo run`. It will launch a REPL. To exit from the
REPL, use ctrl-c

```
> a
Char('a')
> (a b)
Pair(
  Char('a')
  Char('b')
)
> (a (b c))
Pair(
  Char('a')
  Pair(
    Char('b')
    Char('c')
  )
)
> (a (b c)
Error: Incomplete input
>
```
