# Tuple Sort

A fixed length list of values.

## Example

~~~policy
( 1 , 2 )
~~~

## Properties

| Name          | Description |
|---------------|-------------|
| **items**     | The list items. |


## Rewrite

1. For each **item** in **items**

    A. Rewrite **item**

    A.a If blocked item: store blocked **List Item**, **any_blocked = true**
    
    | Property     | Value |
    |--------------|-------|
    |**term**      | result of blocked **term** |
    |**parallel**  | copy of **parallel** |

2. If any items blocked: return blocked **Tuple Term**
    
    | Property     | Value |
    |--------------|-------|
    |**items**     | result of blocked **items** |

3. If no items blocked: return **List Value**
    
## Conformance Tests

### Test List: Simple List
~~~policy
( 1; 2 )
~~~

#### Result
~~~policy
( 1; 2 )
~~~
