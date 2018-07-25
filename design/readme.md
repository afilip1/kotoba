# Kotoba
Kotoba is a simple programming language created with the purpose of learning the
intrinsics of language design and implementation. It is a C-like language supporting
the basic facilities expected from a programming language, such as variables,
functions, conditional flow, etc. It is dynamically typed and garbage collected.

"Kotoba" (言葉) is Japanese for "word".

# Syntax considerations

## Requirements
- interpreted
- dynamically typed
- REPL support
    - concise syntax (possible to write most expressions on one line)
- does not reinvent the wheel unless necessary
- minimal, but expressive
- one, obvious way to do a thing
- allows for growth (e.g. addition of static analysis at a later point)

## Language syntax comparison

### C

```c
// line comment
/* block comment */

// Literals
0
1
4567.234
'a'
"hello world\0"
[1, 2, 3]

// Imports
#include <windows.h>

// Macros
#define TRUE 1
#define FALSE 0
#define max(a, b) (((a) > (b)) ? (a) : (b))

// Expressions
0 == 1
3.4665 + 23456 * 23346 >= 2346761 && !('a' == 65)
(TRUE) ? 1 : 2

// Statements
int32_t x = 254;

if (1 == 2)
    x = 123;
else
    x = 321;

if (3 >= 5) {
    int x = 1;
    int y = 2;
    int z = y + x;    
}

while (1) {
    x++;
    x--;
}

// Declarations
struct Foo {
    int x,
    int y
}

int foo(int x, int y) {
    if (x > y)
        return x;
    else
        return y;
}
```

### Rust

```rust
// line comment
/* block comment */

// Literals
0
123.456
true
'c'
"blah"
r#""blah""#
[1, 2, 3]

// Imports
extern crate foo; // Rust 2015
use std::io::prelude::*;

// Macros
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp = std::collections::HashMap::new();
        $(tmp.insert($key, $value);)*
        tmp
    });
}

// Expressions
0 == 1
3.4665 + 23456.0 * 23346.0 >= 2346761.0 && !('a' == 65)
if true {1} else {2}

// Statements
let foo = 12;

let mut bar = vec![];

if true {
    foo();
} else {
    bar();
}

while a > b {
    b += 1;
}

for i in 0..5 {
    i + 1;
}

loop {
    poll_events();
}

match x {
    Some(t) => t,
    _ => panic!("Fuck me"),
}

// Declarations
struct Foo<'a> {
    bar: &'a [u8],
    baz: Vec<String>,
}

fn foo(x: u32, y: u32) -> bool {
    x + y > 10
}
```

```python
# line comment - no block comment

# Literals
0
1
4567.234
"this is a string"
'this is also a string'
True
[1, 2, 3, "lmao a string"]
None

# Imports
import random

# Macros - none

# Expressions
0 == 1
3.4665 + 23456 * 23346 >= 2346761 and not 24 == 65
1 if True else 2

# Statements
x = 254

if 1 == 2:
    x = 123
else:
    x = 321

if 3 >= 5:
    x = 1
    y = 2
    z = y + x

while 1:
    x += 1
    x -= 1

for i in range(0, 5):
    i + 5

# Declarations
class Foo:
    x = 1,
    y = "heh"

    def method(x, y):
        pass

def foo(x, y):
    if x > y:
        return x
    else:
        return y
```

# Syntax proposal

```rust
// line comment - no block comment

// Literals
0
1.2345
true
false
"stringalingding"
nil

// Imports - none for now

// Expressions
1 + 3
-45.67 / (3 - 345677)
true && !false || 23 >= 45

// Statements
x = 123, // , continues current scope to the next stmt (enforced for readability)
if a > b: foo(), bar() else baz(); // : starts a new local scope
while x < 5: foo(), bar(), x = x + 1; // ; ends the current local scope (and so does `else`)

// Declarations
fn foo(x, y): bar(x), baz(y), ret 1; // nil if no explicit ret
```

## Syntax example: Fizzbuzz
```rust
fn div(q, n): ret n % q == 0; x = 1, while x <= 100: if div(3, x): print("Fizz"); if div(5, x): print("Buzz"); if !(div3(x) || div5(x)): print(x); print("\n"), x = x + 1;
```

## Syntax example with annotated scopes
```rust
// ---------------------------------- global
fn div(q, n): ret n % q == 0; // ----- extend: global <- fn, yield
x = 1, // ---------------------------- global
while x <= 100: // ------------------ extend: global <- while
    if div(3, x): // -------------------- extend: global <- while <- if
        print("Fizz"); // -------------------- yield to parent: global <- while
    if div(5, x): print("Buzz"); // ------ extend: global <- while <- if, yield
    if !(div3(x) || div5(x)): print(x); // extend: global <- while <- if, yield
    print("\n"), // --------------------- global <- while, continue
    x = x + 1; // ------------------------ global <- while, yield
// ---------------------------------- global
```