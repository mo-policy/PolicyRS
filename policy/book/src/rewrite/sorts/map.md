# Map Sort

A mapping of strings to values.

## Example

~~~policy
{ "a": 1 }
~~~

## Properties

| Name          | Description |
|---------------|-------------|
| **items**     | A list of map items. |

## Map Item Properties

| Name          | Description |
|---------------|-------------|
| **key**       | The key of the mapping. |
| **value**     | The value of the mapping. |
| **parallel**  | Executed sequentially (**false**) or parallel (**true**). |

## Rewrite

1. For each **item** in **items**

    A. Rewrite **key**

    A.a If blocked item: store blocked **Map Item**, **any_blocked = true**
    
    | Property     | Value |
    |--------------|-------|
    |**key**       | result of blocked **key** |
    |**value**     | copy of **value** |
    |**parallel**  | copy of **parallel** |

    A.b If result not String, Exception "Map key not String"

    B. Rewrite **value**

    B.a If blocked item: store blocked **Map Item**, **any_blocked = true**
    
    | Property     | Value |
    |--------------|-------|
    |**key**       | result of **key** |
    |**value**     | result of blocked **value** |
    |**parallel**  | copy of **parallel** |

2. If any items blocked: return blocked **Map Term**
    
    | Property     | Value |
    |--------------|-------|
    |**items**     | result of blocked **items** |

3. If no items blocked: return **Map Value**

## Conformance Tests

### Test List: Simple List
~~~policy
{ "a": 1 }
~~~

#### Result
~~~policy
{ a: 1 }
~~~
