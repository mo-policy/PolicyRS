# Policy Sort

Conditional branching based on pattern matching of the current term.

### Example

~~~policy
policy 
    send "hello" on "world"
with 
| send _ on "world" -> 
    throw "Send not permitted on 'world' channel."
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **term**      | The program. |
| **rules**     | A list of rule items. |


### Rewrite

1. Add **policy** to current list of active policies.

2. Rewrite **term**

    2.a If blocked: return blocked **policy**
    
    | Property     | Value |
    |--------------|-------|
    |**term**      | result of blocked **term** |
    |**rules**     | copy of **rules** |


### Policy Rule Matching

1. At the start of each term rewrite, for each active policy, from inner to outer **policy**

    1.a. Attempt rule match against current term using binding context of **policy**

    1.a.a. If rule match, return rewrite of **term** of matching rule using rule bindings.

    1.a.b. else, if no rule match, continue to next outer **policy**.

2. If no **policy** rules match, return rewrite of current term.    

### Conformance Tests

#### Test Policy: Simple Policy
~~~policy
policy 
    1 + 1
with 
| x -> x + 1
~~~

##### Result
~~~policy
4
~~~
