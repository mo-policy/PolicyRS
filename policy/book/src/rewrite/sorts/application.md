# Application Sort

Application of a funtion with an argument.

### Example

~~~policy
(fun x -> x) 1
~~~

### Properties

| Name    | Description |
|---------|-------------|
| **fun** | The function to be applied. |
| **arg** | The argument used in the application. |

### Rewrite

1. Rewrite property **fun**
    
    1.a. If blocked: return blocked application
    
    | Property | Value |
    |----------|-------|
    |**fun**   | result of blocked **fun** |
    |**arg**   | copy of **app** |
    
    1.b. If result not Function: return exception "Not Function"

2. Rewrite property **arg**

    2.a. If blocked: return blocked application

    | Property | Value |
    |----------|-------|
    |**fun**   | result of rewrite **fun** |
    |**arg**   | result of blocked **app** |

3. Bind **arg** result to **fun**.**pattern** using **fun**.**bindings**

    3.a Return rewrite **fun**.**term** using binding result

### Conformance Tests

#### Test Application: Apply identity function
~~~policy
(fun x -> x) 1
~~~

##### Result
~~~policy
1
~~~

#### Test Application: Apply blocked funtion
~~~policy
blocked 1
~~~

##### Result
~~~policy
blocked 1
~~~

#### Test Application: Apply blocked argument
~~~policy
(fun x -> x) blocked
~~~

##### Result
~~~policy
(fun x -> x) blocked
~~~
