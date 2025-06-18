# Send Sort

Sends a message on a channel.

### Example

~~~policy
send "hello" on "world"
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **channel**   | The channel over which to send the message. |
| **message**   | The message to send. |

### Rewrite

1. Rewrite property **channel**
    
    1.a. If blocked: return blocked **send**
    
    | Property       | Value |
    |----------------|-------|
    |**channel**     | result of blocked **channel** |
    |**message**     | copy of **message** |
    
2. Rewrite property **message**
    
    2.a. If blocked: return blocked **send**
    
    | Property       | Value |
    |----------------|-------|
    |**channel**     | result of **channel** |
    |**message**     | result of blocked **message** |

3. Send the message on the channel

4. Return **null**

### Conformance Tests

#### Test Assignment: Send on channel.
~~~policy
send 1 on "a"
~~~

##### Result
~~~policy
null
~~~
