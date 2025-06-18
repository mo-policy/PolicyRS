# Dereference Sort

Dereference a DID URL.

### Example

~~~policy
`did:policy:123/path?query#fragment`
~~~

### Properties

| Name          | Description |
|---------------|-------------|
| **url**       | The url to dereference. |

### Rewrite

1. Rewrite property **url**
    
    1.a. If blocked: return blocked dereference
    
    | Property       | Value |
    |----------------|-------|
    |**url**   | result of blocked **dereference** |
    
    1.b. If result not Url: return exception "Not Url"

2. Return result of **dereference** rewrite.

### Conformance Tests

#### Test Dereference: --- todo
~~~policy
todo
~~~

##### Result
~~~policy
1
~~~
