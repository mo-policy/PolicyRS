# Standard Library

Note: Consider making these the same as JavaScript

- Integer
    - std::integer::MIN  i64::MIN
    - std::integer::MAX  i64::MAX
    - std::integer::checked_abs(self, rhs: integer) -> integer | null
    - std::integer::checked_add(self, rhs: integer) -> integer | null
    - std::integer::checked_div(self, rhs: integer) -> integer | null
    - std::integer::checked_mul(self, rhs: integer) -> integer | null
    - std::integer::checked_neg(self, rhs: integer) -> integer | null
    - std::integer::checked_pow(self, rhs: integer) -> integer | null
    - std::integer::checked_rem(self, rhs: integer) -> integer | null
    - std::integer::checked_shl(self, rhs: integer) -> integer | null
    - std::integer::checked_shr(self, rhs: integer) -> integer | null
    - std::integer::checked_sub(self, rhs: integer) -> integer | null
    - std::integer::isqrt(self) -> integer

- Maps
    - std::map::contains_key(self, k: string) -> bool
    - std::map::get(self, k: string) -> value
    - std::map::insert(self, k: string, v: value) -> value
    - std::map::is_empty(self) -> bool
    - std::map::keys(self) -> list
    - std::map::len(self) -> integer
    - std::map::new() -> map
    - std::map::remove(self, k: string) -> map
    - std::map::values(self) -> list
    - iterators?
    - combinators?

- Lists
    - std::list::get(self, index: integer) -> value
    - std::list::insert(self, index: integer, element: value) -> list
    - std::list::is_empty(self) -> bool
    - std::list::len(self) -> integer
    - std::list::new() -> list
    - std::list::pop(self) -> list
    - std::list::push(self) -> list
    - std::list::remove(self, index: integer) -> list
    - iterators?
    - combinators?

- Tuples
    - std::list::get(self, index: integer) -> value
    - iterators?

- Comparison
    - std::cmp::eq(a: value, b: value) -> bool
    - std::cmp::ord(a: value, b: value) -> integer (-1: a < b) (0: a = b) (1: a > b)
    - std::cmp::max(a: value, b: value) -> value 
    - std::cmp::min(a: value, b: value) -> value 

