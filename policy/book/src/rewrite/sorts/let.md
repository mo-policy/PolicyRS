# Let Sort

Binds values to names.

### Example

~~~policy
let x = 1 in x
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **pattern**   | The binding pattern. |
| **term**      | The value to bind. |
| **in**        | The term the binding is in scope. |

### Rewrite

1. Rewrite **term**

    1.a If blocked: return blocked **let**
    
    | Property     | Value |
    |--------------|-------|
    |**pattern**   | copy of **pattern** |
    |**term**      | result of blocked **term** |
    |**in**        | copy of **in** |

2. Bind result of **term** to **pattern**

3. Return rewrite of **in** using new bindings

    3.a If blocked: return blocked **let in**
    
    | Property     | Value |
    |--------------|-------|
    |**pattern**   | copy of **pattern** |
    |**term**      | result of **term** |
    |**in**        | result of blocked **in** |
    
### Conformance Tests

#### Test Function: Simple Function
~~~policy
let x = 1 in x
~~~

##### Result
~~~policy
1
~~~
