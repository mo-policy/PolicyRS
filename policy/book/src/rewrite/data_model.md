# Data Model

The data model of policy is the DID data model.

## DID Data Model

| Data Type | Description |
|-----------|-------------|
| map       | A finite ordered sequence of key/value pairs, with no key appearing twice. |
| list      | A finite ordered sequence of items. |
| set       | A finite ordered sequence of items that does not contain the same item twice. |
| datetime  | A date and time value. |
| string    | A sequence of code units often used to represent human readable language. |
| integer   | A real number without a fractional component. |
| double    | A value that is often used to approximate arbitrary real numbers. |
| boolean   | A value that is either true or false. |
| null      | A value that is used to indicate the lack of a value. |

See: [DID Data Model](https://www.w3.org/TR/did-core/#data-model)

### Sorts and Hashes

Each policy value has two meta-properties: sort and hash.

| Meta-Property | Description | 
|---------------|-------------|
| sort          | The 'sort' of the value determines the rewrite rules applied. It is comprised of two values, a sort name and data type name. Example: (send, map) for a send operation represented as a map. See: [Sorts](./sorts.md) |
| hash          | A SHA-256 hash of the value. See: [Hashing](./hashing.md) |


## Representations

Any valid DID representation is a valid Policy representation. See: [DID Representations](https://www.w3.org/TR/did-core/#representations)

When reading or writing policy values from /to any representation, the assignment/serialization of the value's sort is performed.

When reading or writing policy values from/to any representation, the hash value is computed on read and not written on write. Any hash value provided in the representation is representation must be ignored.

### Policy ML Representation

Policy ML is a text based representation with syntax similar to ML (OCaml, F#), JavaScript, and Rust. See: [Policy ML](../policy/policy-ml.md)

### JSON Representation

When Policy values are presented in JSON, the policy sort information is modeled in a map property with a key of "sort". If the value is a string and is one of the valid sort names, then the data value is assigned the sort with that name and a data type of "map". If the value is not a string or is not one of the valid sort names, the sort assigned is "constant" and data type "map". Any value type other than map is assigned the sort "constant" and the data type of the value. For example, a Boolean true would be assiged the sort "constant" and data type "boolean".

#### JSON Schema

Representations which use JSON Schema must use a canonical schema URI of https://mobileownership.org/policy-schema.json when representing sorts. All other values of any other schema will be assigned the sort "constant".

### JSON-LD Representation

JSON-LD representations use a @context with a @name of https://mobileownership.org/policy.

