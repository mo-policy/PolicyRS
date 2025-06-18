# Throw Sort

Creates an exception.

## Example

~~~policy
throw "error"
~~~

## Properties

| Name             | Description |
|------------------|-------------|
| **description**  | The cause of the exception. |


## Rewrite

1. Rewrite **description**

    1.a If blocked, return blocked **throw**

    | Property        | Value |
    |-----------------|-------|
    |**description**  | result of blocked **description** |

2. Return exception with description.

## Conformance Tests

### Test List: Simple List
~~~policy
throw "error"
~~~

#### Result
~~~policy
Exception: "error"
~~~
