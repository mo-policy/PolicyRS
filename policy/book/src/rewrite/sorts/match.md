# Match Sort

Conditional branching based on pattern matching.

### Example

~~~policy
match 1 with 
| x -> x
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **term**      | The value to match against. |
| **rules**     | A list of rule items. |


## Rule Properties

| Name          | Description |
|---------------|-------------|
| **pattern**   | The binding pattern. |
| **guard**     | A Boolean gaurd, evaluated on successful pattern match. If not **true** will not branch. |
| **term**      | The result term if the pattern matched successfully and the guard is **true**. |

### Rewrite

1. Rewrite **term**

    1.a If blocked: return blocked **match**
    
    | Property     | Value |
    |--------------|-------|
    |**term**      | result of blocked **term** |
    |**rules**     | copy of **rules** |

2. For each rule in **rules**

    2.a. Attempt to bind **term** to **rule**.**pattern**

    2.a.a. If binding is successfull, rewrite rule.**guard** with rule bindings.

    2.a.a.a. If **guard** is blocked, return blocked **match** with blocked rule guard.

    2.a.a.b. else, if **guard** is **true**, return rewrite of rule.**term** with rule bindings.

3. If no rules matched, return exception "No rules matched".

### Conformance Tests

#### Test Function: Simple Function
~~~policy
match 1 with 
| x -> x
~~~

##### Result
~~~policy
1
~~~
