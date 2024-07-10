# Econ Spec

> **E**xpression **C**onfig **N**otation

**Econ** is a superset of **Json** that allows the computation and composition of data using expressions and macros. Rust Api info [here](#Econ-Rust-Api)

## Json
**Econ**  can parse standard **Json** 
```js
{
	"a": 10.50,
	"b": "Hello World!",
	"c": true,
	"d": nil,
	"e": {
		"a": "Hello World...Again!",
		"b": [
			{
				"aa": 20,
				"bb": false
			},
			{
				"aa": 30,
				"bb": true
			}
		]
	},
	"f": [
		1,
		2,
		3,
		4,
		5
	],
}
```

## Literals
### Number
numbers are represented by 64 bit floats
```js
5
25.75
```
### String
strings not enclosed with ``"``'s must start with a letter but may contain special characters and digits after as long as they are not operators or keywords. 
```js
"I'm a string with spaces"
Im_a_string_without_spaces
```
You can also make multiline strings that will be formatted in a nice way using the ``\`` operator.
```js
"i": I'm a multiline \
     "string with \"* + - / filter\"" \
     "keywords and operators",
```

>Object keys are parsed as strings so the same rules apply to them as with string values.
>```js
>key_with_no_spaces: 1 //valid
>"key with spaces": 2 //valid
> another key containing spaces: 3 //not valid
>```
### Boolean
```js
true
false
```
### Nil
```js
nil
```
### Object
```js
{
	"a": 1,
	"b": 2,
	"c": {
		"d": {
			"e": "nested"
		}
	}
}
```
### Array
```js
[
	1,
	true,
	"Hello",
	{
		"name": "Suzie"
	},
	nil
]
```

## Expressions
**Econ** can parse expressions into values and will always output valid Json.

>Input
```js
{
	a: ((3+2)*10)/5,
	b: true or false,
	c: "Hello" + " " + "World!",
	d: Hi_Im_also_a_string
}
```
>Output
```js
{
	"a": 10,
	"b": true,
	"c": "Hello World!",
	"d": "Hi_Im_also_a_string"
}
```
## Keywords
 - ``or``
 - ``and``
 - ``not``
 - ``true``
 - ``false``
 - ``nil``
 - ``inf`` - a constant for infinity. Divide by zero's will evaluate as infinity.
 - [Functions](#Functions)
## Operators
**Econ** supports
 - Arithmetic ``+``, ``-``, ``*``, ``/``, ``%``, ``\``
 - Logic ``or``/``||``, ``and``/``&&``, ``not``/``~``
 - Comparison ``>``, ``>=``, ``<``, ``<=``, ``==``, ``~=``
 - Grouping ``()``
 - Ternary ``?:`` - ``condition ? expr if true : expr if false``
 - Access ``[index/key]``, ``.index/key``
 - Length ``#``
 - Comment ``//``
 - Reference ``$``/``!``
 - Macro ``@``
 #### Arithmetic
 **Econ** will do its best to perform arithmetic on types but will not make large leaps. For example: ``"Hello" + " " + "World"`` will yield ``"Hello World"`` or ``"The Number Two ->" + 2`` will yield ``"The Number Two -> 2"`` but ``true + 2`` will throw an error ``
Error Parsing -> "Invalid addition of types."
``
#### Example
>Input 
```js
{
	a: ((1+3)/2) * (5/3),
	//The \ operator wil concat strings or strings and bools/numbers with a newline inebetween.
	d: I'm a multiline \
		"string with \"* + - / filter\"" \
		"keywords and operators",
	e: [
		"multi_line" \
		"string" \
		"in" \
		"array",
		"single line",
		"another multi_line" \
		"string" \
		"in" \
		"array"
	],
	f: "another multi_line" \
		"string" \
		"in" \
		"object",
}
```
>Output
```js
{
	"a": 3.3333333333333335,
	"b": [
		1,
		2,
		3,
		4,
		5,
		6
	],
	"c": {
		"a": 1,
		"b": 2
	},
	"d": "I'm a multiline 
		  string with "* + - / filter"
		  keywords and operators",
	"e": [
			"multi_line
			 string
			 in
			 array",
			"single line",
			"another multi_line
			 string
			 in
			 array"
	],
	"f": "another multi_line
		  string
		  in
		  object"
}
```
**Econ** can concatenate Objects and Arrays.

>Input
```js
{
	a: { a: 1 } + { b: 2 },
	b: [1,2] + [3,4] + [5,6]
}
```
>Output
```js
{
	"a": {
		a: 1,
		b: 2
	},
	"b": [
		1,
		2,
		3,
		4,
		5,
		6
	]
}
```
>Note: ``nil + {} -> {}`` and ``nil + [] -> []`` this is important for function logic specifically ``fold()``
### Logic
Logical operators are similar to other programming languages and you can use the keywords or symbols. We followed in Lua's footsteps opting to use ``~`` over ``!`` for the ``not`` operator 
>Input
```js
{
	a: false,
	b: true,
	c: $a && $b,
	d: $a or $b,
	e: not $d,
	f: ~($c and $a) or $e
}
```
>Output
```js
{
	"a": false,
	"b": true,
	"c": false,
	"d": true,
	"e": false,
	"f": true
}
```
### Comparison
>Input
```js
{
	a: 20 < 20,
	b: 100 > 60,
	c: 25 == 25,
	d: 10 ~= 5,
	e: "Hello" == "Not Hello",
	f: 20 <= 20,
	g: 30 >= 30
}
```
>Output
```js
{
	"a": false,
	"b": true,
	"c": true,
	"d": true,
	"e": false,
	"f": true,
	"g": true
}
```
### Ternary
>Input
```js
{
	a: "a" == "b" ? 1 : 2
}
```
>Output
```js
{
	"a": 2
}
```
### ``.``/``[]`` Access operator 
Access operators are used to get elements from arrays and values from objects. Arrays are 0 base indexed. when using ``[]``  you can use expressions as long as they evaluate into strings for objects and numbers for arrays. Additionally you can group expressions for the ``.`` operator like this
```js
arr: [1,2,3],
a: $arr.(1+1) //outputs -> 3
```
>Input
```js
{
	a: [1,2,3,4,5],
	a_1: $a[1],
	a_0: $a[0],
	a_4: $a.4,
	b: { name: "Dill", age: 20 },
	c: $b.name + " is " + $b[age] + " years old"
}
```
>Output
```js
{
	"a": [1,2,3,4,5],
	"a_1": 2,
	"a_0": 1,
	"a_4": 5,
	"b": { 
		"name": "Dill", 
		"age": 20 
	},
	"c": "Dill is 20 years old"
}
```
>Note: ``.(key/index)`` and ``[key/index]`` are equivalent
### ``#`` Length operator
Gets the length of an Object or Array.
```js
#[0,1,2,3,4] //outputs -> 5
#{ a: 1, b: 2, c: 3} // outputs -> 3
```

## References
In **Econ** you can reference keys using the ``$`` or  ``!`` operators. You cannot reference a key before it is declared.
### ``$`` operator 
Referenced keys must not contain whitespace or any other reserved operators. References are searched for in the current object depth but you may search up in depth by chaining together ``$``'s. If a key is not found it will return ``Nil``

>Input
```js
{
	a: 10,
	b: $a*2,
	c: {
		ca: $$a + $$b,
		cb: $a
	}
}
```
>Output
```js
{
	"a": 10,
	"b": 20,
	"c": {
		"ca": 30,
		"cb": nil
	}
}
```
### ``!`` operator 
Similar to ``$`` but it will walk up the object depth until it finds the key.  

>Input
```js
{
	a: 10,
	b: $a*2,
	c: {
		ca: !a + !b,
		cb: !a
	}
}
```
>Output
```js
{
	"a": 10,
	"b": 20,
	"c": {
		"ca": 30,
		"cb": 10
	}
}
```
## Functions
**Econ** supports a set amount of predefined functions; they include:

 - [Filter](#Filter) ``filter(obj/array, iter => condition) -> obj/array``
 - [Map](#Map) ``map(obj/array, iter => expr) -> obj/array``
 - [Chars](#Chars) ``chars(string) -> array``
 - [To String](#To-String) ``to_string(any) -> string``
 - [Keys](#Keys) ``keys(obj) -> array``
 - [Values](#Values) ``values(obj) -> array``
 - [Fold](#Fold) ``fold(obj/array, |iter, acc| => expr) -> literal``
 - [Sort](#Sort) ``sort(array, |x, y| => cond) -> array``
 - [Zip](#Zip) ``zip(array, array) -> array``
 ### Filter
 Takes an Object or Array iterates through and returns a new Object or Array with only elements matching the condition.
  ##### Example Object
 >Input
```js
{
	a: filter({a: 1, b: 2, c: 3, d: 4}, i => ($i.val%2 == 0) || ($i.key == "a"))
}
```
>Output
```js
{
	"a": {
		"a": 1
		"b": 2,
		"d": 4,
	}
}
```
 ##### Example Array
 >Input
```js
{
	a: filter([0,1,2,3,4,5], i => $i%2 == 0)
}
```
>Output
```js
{
	"a": [0,2,4]
}
```
 ### Map
 Takes an Object or Array iterates through and returns a new Object or Array with elements modified by the expression
  ##### Example Object
 >Input
```js
{
	a: map({a: 1, b: 2, c: 3, d: 4}, i => $i + 1)
}
```
>Output
```js
{
	"a": {
		"a": 2
		"b": 3,
		"c": 4
		"d": 5,
	}
}
```
##### Example Array
>Input
```js
{
	a: map([0,1,2,3,4,5], i => $i+1)
}
```
>Output
```js
{
	"a": [
		1,
		2,
		3,
		4,
		5
	]
}
```
### Chars
Takes a string and returns an Array of chars.
##### Example
 >Input
```js
{
	a: chars("Hello")
}
```
>Output
```js
{
	"a": [
		"H",
		"e",
		"l",
		"l",
		"o"
	]
}
```
### To String
Takes any type and returns a string. Most of the types are straight forward but Objects will take all values and concatentate them in order.
##### Example
 >Input
```js
{
	a: to_string(["H", "e", "l", "l", "o"]),
        b: to_string({a: "hello", b: "world"}),
        c: to_string(nil),
        d: to_string(true),
        e: to_string([1,2,3,[7,4],{a:nil}])
}
```
>Output
```js
{
	"a": "Hello",
        "b": "helloworld",
        "c": "nil",
        "d": "true",
        "e": "12374nil"
}
```
### Keys
Takes an Object and returns and Array of keys.
##### Example
 >Input
```js
{
	a: keys({a: 1, b: 2, c: 3}}
}
```
>Output
```js
{
	"a": [
		"a",
		"b",
		"c"
	]
}
```
 ### Values
 Takes an Object and returns and Array of keys.
  ##### Example
 >Input
```js
{
	a: values({a: 1, b: 2, c: 3}}
}
```
>Output
```js
{
	"a": [
		1,
		2,
		3
	]
}
```
 ### Fold
 Takes an Object or Array and iterates through it while giving you access to an accumulator reference that it returns. The accumulator is initialized as ``Nil``.
  ##### Example Object
 >Input
```js
{
	a: fold({a: 1, b: 2, c: 3}, |i, acc| => $acc + $i.key + "=" + $i.val + " ")
}
```
>Output
```js
{
	"a": "a=1 b=2 c=3"
}
```
 ##### Example Array
 >Input
```js
{
	a: fold([1,2,3,4,5], |i, acc| => $acc + $i)
}
```
>Output
```js
{
	"a": 15
}
```
 ### Sort
 Takes an Array and returns an Array sorted. If you try to sort Arrays with differing types then you will most likely get an error ``
Error Parsing -> "Invalid comparison of types."
``
##### Example Array with Numbers
 >Input
```js
{
	a: sort([200, 30, 500, 5, 60], |x, y| => $x < $y)
}
```
>Output
```js
{
	"a": [
		5,
		30,
		60,
		200,
		500
	]
}
```
##### Example Array with Strings
 >Input
```js
{
	a: sort(["Cucumber", "Broccoli", "Apple", "Banana", "Peach"], |x, y| => $x < $y)
}
```
>Output
```js
{
	"a": [
		"Apple",
		"Banana",
		"Broccoli",
		"Cucumber",
		"Peach"
	]
}
```
 ### Zip
 Takes two Arrays and returns a new Array with elements of same index in sub-arrays.
``
##### Example
 >Input
```js
{
	a: zip([1,2,3], [4,5,6])
}
```
>Output
```js
{
	"a": [
		[
			1,
			4
		],
		[
			2,
			5
		],
		[
			3,
			6
		]
	]
}
```
##### Example
 >Input
```js
{
	a: {
		aa: 1,
		bb: 2,
		cc: 3
	},
	b: zip(keys($a), values($a))
}
```
>Output
```js
{
	"a": {
		"aa": 1,
		"bb": 2,
		"cc": 3
	},
	"b": [
		[
			"aa",
			1
		],
		[
			"bb",
			2
		],
		[
			"cc",
			3
		]
	]
}
```
## Macros
Macros are C-styled and like References must be declared before calling.
### Syntax
```js
identifier(args, ...) token_stream \
token_stream_on_newline
```
##### Example 1
>Input
```js
{
	@person(name, age) name: age
	a: {
		@person("Dave", 20),
		@person("Mickey", 25),
		@person("Suzie", 23),
		@person("Keli", 28)
	}
}
```
>Output
```js
{
	"a": {
		"Dave": 20,
		"Mickey": 25,
		"Suzie": 23,
		"Keli": 28
	}
}
```
##### Example 2
>Input
```js
{
   	@person(id, name, age) \
   	id: {\
       	name: age\
   	}
   	a: {
   		@person("1", "Dave", 20),
   		@person("2", "Mickey", 25),
   		@person("3", "Suzie", 23),
   		@person("4", "Keli", 28)
   	}
}
```
>Output
```js
{
	"a": {
		"1": {
			"Dave": 20
		},
		"2": {
			"Mickey": 25
		},
		"3": {
			"Suzie": 23
		},
		"4": {
			"Keli": 28
		}
	}
}
```
##### Example 3
>Input
```js
{
   	@sort_descending(obj) sort(obj, |x, y| => $x > $y)
   	a: [1,3,0,5],
   	b: @sort_descending($a)
}
```
>Output
```js
{
	"a": [
		1,
		3,
		0,
		5
	],
	"b": [
		5,
		3,
		1,
		0
	]
}
```
##### Example 4
>Input
```js
{
   	@is_even(x) (x%2 == 0) 
   	a: @is_even(2),
   	b: @is_even(3),
   	c: @is_even(7)
}
```
>Output
```js
{
	"a": true,
	"b": false,
	"c": false
}
```
# Pre-Processor Macros
**Econ** allows for two other types of macros which works on types rather than in function style.
## Constraint Macro
```js
@{type, ref => condition, new_value}
```

A constraint macro will work on all values of a certain type that are in scope. Only atomic types are allowed ``number``, ``bool``, ``string``, ``nil``
### Example 1
> Input 
```js
{
	@{number, x => $x%2 == 0, "Even"}
	a: 2,
	b: 3,
	c: 4
}
```
> Ouput
```js
{
	"a": "Even",
	"b": 3,
	"c": "Even"
}
```
### Example 2
> Input 
```js
{
	@{number, x => $x < 5, 5}
	a: [
		2,
		4,
		7,
		5,
		10
	],
	b: {
		@{number, x => $x > 0, $x * -1}
		a: [
			2,
			4,
			7,
			5,
			10
		]
	}
}
```
> Ouput
```js
{
	"a": [
		5,
		5,
		7,
		5,
		10
	],
	"b": {
		"a": [
			5,
			5,
			5,
			5,
			5
		]
	}
}
```
### Example 3
> Input 
```js
{
	@{string, x => $x == $x, to_string(filter(chars($x), x => $x ~= "_"))}
	a: "_Mike",
	b: "Kat_e",
	c: "_Lis_a",
	d: "Bill"
}
```
> Ouput
```js
{
	"a": "Mike",
	"b": "Kate",
	"c": "Lisa",
	"d": "Bill"
}
```
### Example 4
> Input 
```js
{
	@{nil, x => $x, false}
	a: nil
}
```
> Ouput
```js
{
	"a": false
}
```
## Error Macro
An error macro is similar to a constraint macro but it throws a parser error rather than changing the value. The source will not be parsed if the error is thrown and it will not panic the main thread unless you handle it that way.
```js
@!{type, ref => condition, error_msg}
```
### Example 1
> Input
```js
{
	@!{string, x => $x == "Hello World!", "No Hello Worlds!"}
	a: "Hello World!"
}
```
> Output
```rust
Line [0003] Error Parsing -> "No Hello Worlds!"

Line [0002]			@!{string, x => $x == "Hello World!", "No Hello Worlds!"}
Line [0003]	->		a: "Hello World!"
Line [0004] 	}
```
### Example 2
> Input
```js
{
	@!{bool, x => $x == $x, "Use a string Yes/No rather than booleans."}
	a: true
}
```
> Output
```rust
Line [0003] Error Parsing -> "Use a string Yes/No rather than booleans."

Line [0002]			@!{bool, x => $x == $x, "Use a string Yes/No rather than booleans."}
Line [0003]	->		a: true
Line [0004] 	}
```
### Example 3
> Input
```js
{
	@!{
		number, 
		x => ($x == inf) || ($x == -inf), 
		"Divide by zero"
	}
	a: 1/0,
	b: -1/0
}
```
> Output
```rust
Line [0007] Error Parsing -> "Divide by zero"
```

# Econ Rust Api
The proof-of-concept Api for **Econ** is written in *Rust*
## Create
You can use the create function to parse either a file or string. Create can output debug info and will return ``Result<EconObj, String>`` where as ``Econ::from()`` will return an empty ``EconObj`` if it fails and will not output debug info other than errors. 
>Source
```rust
let obj = Econ::create(
r#"
{
    a: 1,
    b: 2,
    c: 3
}
"#, true);
```
## from string
>Source
```rust
let obj = Econ::from(
r#"
{
    a: {
        b: [
            {
                c: [1,2,3]
            },
            {
                c: [4,5,6]
            }
        ]
    }
}
"#);
```
## from file path
>Source
```rust
let obj = Econ::from("file.econ");
```
## Accessing Values
>Source
```rust
let obj = Econ::from(
r#"
{
    a: {
        b: {
            c: [
                1,
                2,
                3,
                4
            ]
        }
    }
}
"#);
assert_eq!(3f64, obj["a"]["b"]["c"][2].value::<f64>());
```
