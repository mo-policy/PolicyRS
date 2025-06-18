# CellSet sort

Set the value of a cell.

### Example

~~~policy
x := 1
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **cell**      | The cell to be set. |
| **value**     | The value to set. |

### Rewrite

1. Rewrite property **cell**
    
    1.a. If blocked: return blocked cell_set
    
    | Property       | Value |
    |----------------|-------|
    |**cell**        | result of blocked **cell** |
    |**value**       | copy of **value** |
    
    1.b. If result not Cell: return exception "Not Cell"

2. Rewrite property **value**

    2.a. If blocked: return blocked cell_set

    | Property     | Value |
    |--------------|-------|
    |**cell**      | result of rewrite **cell** |
    |**value**     | result of blocked **value** |

3. Update cell with value

4. Return **null**

### Conformance Tests

#### Test CellSet: Assign Value to Cell
~~~policy
let x = cell 1 in x := 2
~~~

##### Result
~~~policy
null
~~~
