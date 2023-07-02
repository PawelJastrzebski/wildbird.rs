<br />

<a href="#top"></a>

<h1 style="color: #376FFF; font-size: 2rem; font-weight: 500;">
    Rust framework 🐦 <sup style="color: black; font-size: 1.2rem; font-weight: 400;">'Experimental</sup>
</h1>

# Table of contents
- [Get Started](#get-started)
- [Services](#services)
- [Globals](#globals)
- [Authors](#created-by)
- [License](#license)

# Services

Create service instance (Singleton) in one step

```rust
use wildbird::prelude::*;

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

    fn say_hello(&self) {
        println!("Hello! 👋")
    }
}

fn main() {
    HelloService.say_hello();
}
```
- Async init
```rust
use wildbird::derive::*;

#[service(construct = "async init")]
struct AsyncService {}

impl AsyncService {
    async fn init() -> AsyncService {
        AsyncService {}
    }

    fn greeting(&self) {
        println!("Hello 🗼")
    }
}

fn main() {
    AsyncService.greeting();
}
```
- Async init functional

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
<br />

# Globals
Create global 
```rust
use wildbird::derive::*;

#[var]
pub fn my_name() -> String {
    String::from("Hawk 🦅")
}

fn main() {
    println!("Hello from 🇵🇱, {}", &*MY_NAME);
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
    println!("Home: {}", &*HOME);
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
```

- callback init
```rust
use std::time::Duration;
use wildbird::derive::*;
use std::thread::sleep;

#[var(name = "PORT")]
async fn init_http_service(callback: wildbird::Callback<String>) {
    sleep(Duration::from_millis(200));
    println!("Server started");
    callback.call("8080".to_string());
}
```
<br />

# Get started

##### Add dependency
Cargo.toml
```toml
[dependencies]
wildbird = "^0.0.8"
```

##### Feature flags
Optional features

- *tokio* - Use to support tokio async environment
```toml
[dependencies]
tokio = "1.28.2"
wildbird = {version = "^0.0.8", features = ["tokio"]}
```
<br />

# Created By

<a target="_blank" href="http://wildbirds.studio" >
    <img src="https://wildbirds.studio/img/Logo_full.fe1f5caa.png"  width="30%" height="30%">
</a>

<br />

# License

MIT

<br />

[BACK TO TOP ⬆️](#top)