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
Async init
```rust
use wildbird::derive::*;

#[service(construct = "async init")]
struct AsyncService {
    component_name: String,
}

impl AsyncService {
    async fn init() -> AsyncService {
        AsyncService {
            component_name: "Tokyo 🗼".to_string(),
        }
    }

    fn sey_hello(&self) {
        println!("Hello! 👋")
    }
}

fn main() {
    AsyncService.sey_hello();
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
async fn hello_init() -> HelloService {
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

- Custom name
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

- Async init
```rust
use std::time::Duration;
use wildbird::derive::*;
use std::thread::sleep;

#[var(name = "USERS")]
async fn http_fetch_users() -> String {
    sleep(Duration::from_millis(200));
    String::from("⏱️")
}

fn main() {
    println!("Users: {}", USERS);
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