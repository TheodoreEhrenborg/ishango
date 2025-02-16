# Ishango

A lightweight command-line tool that tracks numerical values in different buckets.

## Installation

```bash
cargo install ishango
```

(And you can get cargo from https://www.rust-lang.org/tools/install)

## Usage

Say you want to track overtime. Create an overtime bucket:

```bash
ishango init overtime
```

Then you're 10 minutes late to work, but you work 20.5 minutes extra at lunch:

```bash
ishango add overtime -10
ishango add overtime 20.5
```

Then you can check the balance:

```bash
ishango balance overtime
```

If you're not sure if you remembered to record being late to work,
you can look back to see if you did:

```bash
ishango transactions overtime
```

### Less useful commands

`ishango list` lists all existing buckets.

`ishango where` shows where the bucket data is stored.
It's in [JSONL](https://jsonlines.org/) format,
so you can edit it by hand if you need.

## Alternatives

For use-cases like money,
it's important to have good records and
be able to query them in various ways ("How much did I spend on food last January?").
I recommend [beancount](https://github.com/beancount/beancount)
(and its GUI [fava](https://beancount.github.io/fava/)).
This gives you the extra benefit of double-entry accounting,
at the cost of taking a tad more time
to record a transaction.

If you need to clock in and out of a project,
I recommend Emacs's
[task clocking](https://orgmode.org/manual/Clocking-commands.html).
