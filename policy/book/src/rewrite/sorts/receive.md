# Receive Sort

Conditional branching based on pattern matching of value received over channel.

### Example

~~~policy
receive on "world" with 
| (id, "hello") -> 
    send 42 on id
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **channel**   | The channel over which to receive values to match. |
| **last_id**   | The id of the last message tested on the channel. |
| **rules**     | A list of rule items. |

### Rewrite

1. Rewrite **channel**

    1.a If blocked: return blocked **receive**
    
    | Property     | Value |
    |--------------|-------|
    |**channel**   | result of blocked **channel** |
    |**last_id**   | copy of **last_id** |
    |**rules**     | copy of **rules** |

2. For each message available on the **channel** with a message id greater than **last_id**

    2.a. Attempt to match message value against **rules**

    2.b. If a rule matches, return rewrite of **term** of matching rule

3. If no rule matches, return blocked **receive**

    | Property     | Value |
    |--------------|-------|
    |**channel**   | result of **channel** |
    |**last_id**   | value of last message id to be tested |
    |**rules**     | copy of **rules** |

### Channel Communication

Messages are reserved before an attept to match the message against the rules.  If a rule matches, the message is removed from the channel. If no rule is matched, the reservation is released.

The **receive** term stores the last id of the message so subsequent executions of the **receive** will only check newer messages on the channel, if any. 

### Conformance Tests

#### Test Function: Simple Function
~~~policy
send 1 on "a"; 
receive on "a" with 
| x -> x
~~~

##### Result
~~~policy
1
~~~
