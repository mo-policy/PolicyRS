# Try Sort

Conditional branching based on pattern matching of exceptions.

### Example

~~~policy
try 
    throw "error"
with 
| "error" -> "An error occured"
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **term**      | The term within which the try catch is in scope. |
| **rules**     | A list of rule items. |


### Rewrite

1. Rewrite **term**

    1.a. If blocked: return blocked **try catch**
    
    | Property     | Value |
    |--------------|-------|
    |**term**      | result of blocked **term** |
    |**rules**     | copy of **rules** |

2. If **term** results in an exception, 

    2.a. Attempt to bind the exception to **rule**.**pattern**

    2.a.a. If binding is successfull, rewrite rule.**guard** with rule bindings.

    2.a.a.a. If **guard** is blocked, return blocked **try catch** with blocked rule guard.

    2.a.a.b. else, if **guard** is **true**, return rewrite of rule.**term** with rule bindings.

3. If no rules matched, return the exception

4. If no exception occured during the rewrite of **term** return the result.

### Conformance Tests

#### Test Function: Simple Function
~~~policy
try 1 with 
| 1 -> 2
~~~

##### Result
~~~policy
1
~~~
