<br />
<h1 style="color: #376FFF; font-size: 2rem; font-weight: 500;">
    Rust framework 🐦 <sup style="color: black; font-size: 1.2rem; font-weight: 400;">'Experimental</sup>
</h1>

# Table of contents

- [Services](#Services)
- [Authors](#Created-By)
- [License](#License)

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

# Created By

<a target="_blank" href="http://wildbirds.studio" >
    <img src="https://wildbirds.studio/img/Logo_full.fe1f5caa.png"  width="30%" height="30%">
</a>

<br />

# License

MIT