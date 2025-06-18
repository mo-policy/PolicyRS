# Lookup Sort

Returns the value of a bound name.

## Example

~~~policy
x
~~~

## Properties

| Name          | Description |
|---------------|-------------|
| **name**      | The name to lookup. |


## Rewrite

1. Lookup name in current bindings

    1.a. If name is found and bound value is Fix, return rewrite of bound value.

    1.b. else, if name is found, return bound value

    1.c. else, if name not found, return blocked **lookup**

    | Property     | Value |
    |--------------|-------|
    |**name**      | copy of **name** |

## Conformance Tests

### Test List: Simple List
~~~policy
let x = 1 in x
~~~

#### Result
~~~policy
1
~~~
