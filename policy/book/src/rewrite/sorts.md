# Sorts

| Sort Name     | Description |
|---------------|-------------|
| [Application](./sorts/application.md) | Application of a funtion with an argument. |
| [Cell](./sorts/cell.md) | A mutable value. |
| [CellGet](./sorts/cell_set.md) | Get the current value of a cell. |
| [CellSet](./sorts/cell_set.md) | Assign a value to a cell. |
| [Constant](./sorts/constant.md) | A constant value. |
| [Dereference](./sorts/dereference.md) | Dereference a DID URL. |
| [Eval](./sorts/eval.md) | Rewrite value as code. |
| [Fix](./sorts/fix.md) | Fixed point for recursive function. |
| [Function](./sorts/function.md) | A function definition. |
| [If](./sorts/if.md) | Conditional branching, if ... then ... else ... . |
| [Let](./sorts/let_in.md) | Binds values to names. |
| [List](./sorts/list.md) | A list of values. |
| [Lookup](./sorts/lookup.md) | Returns the value of a bound name. |
| [Loop](./sorts/loop.md) | Repeated execution. |
| [Map](./sorts/map.md) | A mapping of strings to values. |
| [Match](./sorts/match.md) | Conditional branching based on pattern matching. |
| [Policy](./sorts/policy.md) | Conditional branching based on pattern matching of the current term. |
| [Receive](./sorts/receive.md) | Conditional branching based on pattern matching of value received over channel. |
| [Send](./sorts/send.md) | Sends a message on a channel. |
| [Sequence](./sorts/sequence.md) | A sequence of terms, resulting in the value of the last item. |
| [Throw](./sorts/throw.md) | Creates an exception. |
| [Try](./sorts/try.md) | Conditional branching based on pattern matching of exceptions. |
| [Tuple](./sorts/tuple.md) | A fixed length list of values. |



Note: The lookup with name 'blocked' is used to produce a blocked result. The name 'blocked' is assumed to be unbound and unbound names block.