# Function Sort

A function definition.

### Example

~~~policy
fun x -> x
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **pattern**   | The binding pattern for the argument to the function. |
| **term**      | The body of the funtion. |
| **bindings**  | A closure of name bindings used in the function body. |

### Rewrite

1. Calculate closure from names used in **term**
    
2. Return new function with **bindings** set to closure bindings.

### Conformance Tests

#### Test Function: Simple Function
~~~policy
fun x -> x
~~~

##### Result
~~~policy
fun x -> x
~~~
