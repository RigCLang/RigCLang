# RigC Language

A simple and flexible native language that compiles to C.

## Example

```rigc
fn main {
	var world = "World";
	print("Hello, {}!", world);
}
```

<details>
<summary>More examples</summary>
<div>

### Ask for input

```rigc
fn main {
	var name: String;
	var age: Int;
	println("Please enter your name and age (example: John 20): ");
	scan_input("{} {}", name, age);

	if (age >= 18) {
		println("Hello, {}! You are an adult.", name);
	} else {
		println("Hello, {}! You are a minor.", name);
	}
}
```

### Fibonacci

```rigc
fn fib(n: Int) -> Int {
	if (n <= 1) {
		return n;
	} else {
		return fib(n - 1) + fib(n - 2);
	}
}

fn main {
	println("Please enter a number: ");
	let n = Int.try_parse( input("{}", n) )?;

	println("Fibonacci of {} is {}.", n, fib(n));
}
```

### Item list

```rigc
fn main {
	var items = Array(["Apple", "Banana", "Orange"]);
	for (item in items) {
		println("Item: {}", item);
	}
}
```

### Classes

```rigc
class Person {
	var name: String;
	var age: Int;

	construct(name: String, age: Int) {
		this.name = name;
		this.age = age;
	}

	greet {
		println("Hello, {}! You are {} years old.", this.name, this.age);
	}
}

fn main {
	var person = Person("John", 20);
	person.greet();
}
```

</div>
</details>