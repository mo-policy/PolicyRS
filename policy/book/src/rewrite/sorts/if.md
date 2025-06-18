# If Sort

Conditional branching, if ... then ... else ... .

### Example

~~~policy
if x then 1 else 2
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **condition** | The test condition. |
| **then**      | The result when the condition is **true**. |
| **else**  | The result when the condition is **false**. |

### Rewrite

1. Rewrite **condition**

    1.a If blocked: return blocked **if**
    
    | Property     | Value |
    |--------------|-------|
    |**condition** | result of blocked **condition** |
    |**then**      | copy of **then** |
    |**else**      | copy of **else** |

    1.b. If result not Boolean: return exception: "Not Boolean".

    1.c. If result is **true**, return rewrite of 
    **then**.

    1.d. If result is **false**, return rewrite of **else**.
    
### Conformance Tests

#### Test Function: Simple Function
~~~policy
if true then 1
~~~

##### Result
~~~policy
1
~~~
