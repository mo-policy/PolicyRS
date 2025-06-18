# Fix Sort

Fixed point for recursive function.

### Example

~~~policy
fix (fun x -> x)
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **term** | The function to make recursive. |

### Rewrite

1. Rewrite property **term**
    
    1.a. If blocked: return blocked fix
    
    | Property       | Value |
    |----------------|-------|
    |**term**   | result of blocked **term** |
    
    1.b. If result not Function: return exception "Not Function"

2. Bind **term**.**pattern** to fix term.

3. Return rewrite of 

### Conformance Tests

#### Test Fix: Apply identity function
~~~policy
let rec f = fun x -> x in f 1
~~~

##### Result
~~~policy
1
~~~
