# Cell Sort

A mutable cell.

### Example

~~~policy
let x = cell 1
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **value**     | The initial value of the cell. |

### Rewrite

1. Rewrite property **value**
    
    1.a. If blocked: return blocked **cell**
    
    | Property       | Value |
    |----------------|-------|
    |**value**       | result of blocked **value** |
    
2. Return cell with value

### Conformance Tests

#### Test Assignment: Create Cell
~~~policy
let x = cell 1 in !x
~~~

##### Result
~~~policy
1
~~~
