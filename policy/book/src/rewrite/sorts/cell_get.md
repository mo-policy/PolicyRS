# CellGet Sort

Return the current value of a cell.

### Example

~~~policy
!x
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **cell**      | The cell. |

### Rewrite

1. Rewrite property **cell**
    
    1.a. If blocked: return blocked cell_get
    
    | Property       | Value |
    |----------------|-------|
    |**cell**        | result of blocked **cell** |
    
    1.b. If result not Cell: return exception "Not Cell"

2. Return result of **cell_get** rewrite.

### Conformance Tests

#### Test CellGet: Apply identity function
~~~policy
let x = cell 1 in !x
~~~

##### Result
~~~policy
1
~~~
