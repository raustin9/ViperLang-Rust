# Viper Programming Language

Viper is meant to be a nice middle ground between the type system and syntax of rust while keeping the direct memory control of C. 

Dissimilar from Rust, the purpose of Viper is not memory safety, as the goal is to essentially be C with Rust-like syntax. 

Example hello world:
```rust
define main(argc: i32, argv[[u8]]): i32 {
    std::io::print("Hello, World\n");
    return 0;
}
```

Structured Data Types:
```rust
/// Data type that represents a "User"
/// These fields are private by default and can only be accessed by 
/// methods defined for the User type
struct User {
    name: std::string,
    age: u32,
}

impl User {
    /// Static method defined for the TYPE User to generate and return a new user from the defined values
    public define new(name: std::string, age: u43): User {
        return User {
            name: name,
            age: age,
        };
    }
    
    /// Public method defined for the 
    public define print(&self): void {
        std::io::print("Name: ${self.name}. Age: ${self.age}");
        return 0;
    }

    /// Accessor for the private 'User.name' field 
    public name(&self): std::string {
        return self.name;
    }
}

define main(argc: i32, argv[[u8]]): i32 {
    let Alex: User = User::new("Alex", 22);
    Alex.print();

    let name: std::string = Alex.name();
    
    return 0;
}
```

Rust-like Sum Types
```rust
enum Weapon {
    LaserGun {
        ammo: u32,
        bulletspeed: u32,
    },
    Sword {
        range: u32,
        recharge: u32,
    },
}

define main(void) {
    let gun: Weapon = Weapon::LaserGun {
        ammo: 100,
        bulletspeed: 10,
    };

    return 0;
}
```

Loops
```rust
// Standard for loop
for (let i: i32 = 0; i < 10; i += 1) {
    std::io::print("i: ${i}");
}

// While loop
while token.kind != TokenKind::EOF {
    token = lexer.next_token();
}
```

Functions
```rust
// Functions are defined using the 'define' keyword
define test(parameter1: u32, parameter2: std::string): return_type {
    // Like most other languages, you return a value from a function using the 'return' keyword
    return value;
}
```

Expressions and the 'yield' keyword:
```rust
// Like Rust, most pieces of code are expressions
// This means that they can be evaluated and used within other expressions
let i: i32 = if User.name == "Alex" {
    yield 10;
} else {
    yield 0;
}; // note the trailing ';'

// Rust has similar functionality with their tails in code-block expressions.
// However, I prefer to be more expressive, so the yield keyword does this for Viper.
// Code blocks do not have to be attached to an if-else clause to evaluate
let i: std::string = {
    let a:i32 = 10;
    let b:i32 = 90;

    yield a + b;
};
std::io::print("i: ${i}"); // prints "i: 100"
```
