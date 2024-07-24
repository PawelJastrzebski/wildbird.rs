<br />

<a href="#top"></a>

<h1 align="center" style="color: #376FFF; font-size: 2.4rem; font-weight: 500; border: none;">
Rust Framework  🐦 
<sup style="color: #777; font-size: 1rem; font-weight: 400;">Wildbird</sup>
</h1>
<br />

## Introduction 👋
Welcome to the Wildbird 🐦, designed to streamline your Rust development. Equipped with tools for creating and managing services 🧩 and globals with dependency injection 📌 functionality.

<br />

## Table of contents
- [Get Started](#get-started)
- [Services](#services)
- [Globals](#globals)
- [Dependency Injection](#dependency-injection)
- [Authors](#created-by)

<br />

## Services

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
<br />

## Globals
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
<br />

## Dependency Injection
Injection support is currently limited to the `#[service(construct)]` initialization method.

For example:
```rust
use wildbird::prelude::*;

#[service]
struct B {
    name: String,
}
#[service(construct)]
async fn b_init() -> B {
    B {
        name: "Baby 🐤".to_string(),
    }
}

#[service]
struct A {
    name: String,
    b_service: Arc<B>
}
#[service(construct)]
async fn a_init(b: Arc<B>) -> A {
    A {
        name: "Automobile 🚗".to_string(),
        b_service: b
    }
}
```

<br />
<br />

## Get started

##### Add dependency
Cargo.toml
```toml
[dependencies]
wildbird = "^0.0.11"
```

##### Feature flags
Optional features

- *tokio* - Use to support tokio async environment
```toml
[dependencies]
tokio = "1.28"
wildbird = {version = "^0.0.11", features = ["tokio"]}
```
<br />

##### Project status

Project is in early state of development. Each release is prior tested but api changes will most likely to happen in the future as the project progress.

<br />
<br />

## Created By

<a target="_blank" href="http://wildbirds.studio" >
    <img src="https://wildbirds.studio/img/Logo_full.fe1f5caa.png"  width="30%" height="30%">
</a>

<br />

This project is licensed under the MIT License.

<br />

[BACK TO TOP ⬆️](#top)

<br />
<br />