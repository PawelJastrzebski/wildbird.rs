<br />

<a href="#top"></a>

<h1 style="color: #376FFF; font-size: 2rem; font-weight: 500;">
    Rust framework 🐦 <sup style="color: black; font-size: 1.2rem; font-weight: 400;">'Experimental</sup>
</h1>

# Table of contents

- [Services](#services)
- [Globals](#globals)
- [Authors](#created-by)
- [License](#license)

# Services

Create service instance (Singleton) in one step

```rust
use wildbird::derive::*;

// Convert struct to Service + impl construct()

#[service(construct = "init")]
struct HelloService {
    component_name: String,
}

impl HelloService {
    fn init() -> HelloService {
        HelloService {
            component_name: "Hello penguins 🐧".to_string(),
        }
    }

    fn sey_hello(&self) {
        println!("Hello! 👋")
    }
}

fn main() {
    HelloService.sey_hello();
}
```

<br />

```rust
use wildbird::derive::*;

// Convert struct to Service
#[service]
struct HelloService {
    component_name: String,
}

// Impl Service trait construct() 
#[service(construct)]
fn hello_init() -> HelloService {
    HelloService {
        component_name: "Hello 🚀".to_string(),
    }
}
```

# Globals

Create global variable

```rust
use wildbird::derive::*;

#[var]
pub fn my_name() -> String {
    String::from("Hawk 🦅")
}

fn main() {
    println!("Hello from 🇵🇱, {}", MY_NAME);
}
```

<br />

```rust
use wildbird::derive::*;

#[var(name = "HOME")]
fn custom_name() -> String {
    std::env::var("HOME").expect("env:HOME not found")
}

fn main() {
    println!("Home: {}", HOME);
}
```

# Created By

<a target="_blank" href="http://wildbirds.studio" >
    <img src="https://wildbirds.studio/img/Logo_full.fe1f5caa.png"  width="30%" height="30%">
</a>

<br />

# License

MIT

<br />

[BACK TO TOP ⬆️](#top)