# Econ 

> **E**xpression **C**onfig **N**otation

**Econ** is a superset of **Json** that adds the ability to compute and compose data using expressions.

## Json
**Econ**  can parse standard **Json** 
```js
{
	a: 10.50,
	b: "Hello World!",
	c: true,
	d: nil,
	e: {
		a: "Hello World...Again!",
		b: [
			{
				aa: 20,
				bb: false
			},
			{
				aa: 30,
				bb: true
			}
		]
	},
	f: [
		1,
		2,
		3,
		4,
		5
	],
	"Key with spaces": true
}
```
>all keys are strings

## Literals
### Number
>numbers are represented by 64 bit floats
```js
5
25.75
```
### String
>strings not enclosed with ``"``'s must start with a letter but may contain special characters and digits after as long as they are not operators or keywords.
```js
"I'm a string with spaces"
Im_a_string_without_spaces
```
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
	a: 1,
	b: 2,
	c: {
		d: {
			e: "nested"
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
		name: "Suzie"
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
	a: 10,
	b: true,
	c: "Hello World!",
	d: "Hi_Im_also_a_string"
}
```

### Operators
**Econ** supports
 - Arithmetic ``+``, ``-``, ``*``, ``/``, ``%``
 - Logic ``or``/``||``, ``and``/``&&``, ``not``/``~``
 - Comparison ``>``, ``>=``, ``<``, ``<=``, ``==``, ``~=``
 - Ternary ``condition ? expr if true : expr if false``
 - Access ``[index/key]``, ``.index/key``
 - Length ``#``
 #### Arithmetic
 **Econ** will do its best to perform arithmetic on types but will not make large leaps. For example: ``"Hello" + " " + "World"`` will yield ``"Hello World"`` or ``"The Number Two ->" + 2`` will yield ``"The Number Two -> 2"`` but ``true + 2`` will throw and error ``
Error Parsing -> "Invalid addition of types."
``

**Econ** can concatenate Objects and Arrays as well. 

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
	a: {
		a: 1,
		b: 2
	},
	b: [
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
#### Logic
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
	a: false,
	b: true,
	c: false,
	d: true,
	e: false,
	f: true
}
```
#### Comparison
>Input
```js
{
	a: 20 < 20,
	b: 100 > 60,
	c: 25 == 25,
	d: 10 ~= 5,
	e: "Hello" == "Not Hello",
	f: 20 <= 20,
	g: 30 >= 30,
}
```
>Output
```js
{
	a: false
	b: true,
	c: true,
	d: true,
	e: false,
	f: true,
	g: true,
}
```
#### Ternary
>Input
```js
{
	a: "a" == "b" ? 1 : 2
}
```
>Output
```js
{
	a: 2
}
```
#### Access 
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
	a: [1,2,3,4,5],
	a_1: 2,
	a_0: 1,
	a_4: 5
	b: { 
		name: "Dill", 
		age: 20 
	},
	c: "Dill is 20 years old"
}
```
>Note: ``.(key/index)`` and ``[key/index]`` function the same essentially. 

## References
In **Econ** you can reference keys using the ``$`` or  ``!`` operators.
### ``$ ``operator 
Referenced keys must not contain whitespace or any other reserved operators. References are searched for in the current object depth but you may search up in depth by chaining together ``$``'s. If a key is not found it will return ``Nil``

>Input
```js
{
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
	a: 10,
	b: 20,
	c: {
		ca: 30,
		cb: nil
	}
}
```
### ``!`` operator 
Similar to ``$`` but it will walk up the object depth until it finds the key.  

>Input
```js
{
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
	a: 10,
	b: 20,
	c: {
		ca: 30,
		cb: 10
	}
}
```
