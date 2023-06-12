<br />
<h1 style="color: #376FFF; font-size: 2rem">
    Rust framework <sup style="color: black; font-size: 1.2rem">'Experimental</sup>
</h1>

 # Table of contents

 - [Services](#Services)
 - [Authors](#Created-By)
 - [License](#License)


# Services

```rust
use wildbird::derive::*;

// Convert struct to Service
#[derive(Service)]
struct HelloService {
    component_name: String,
}

// Impl Service trait construct() 
#[ServiceConstruct]
fn hello_init() -> HelloService {
    HelloService {
        component_name: "Hello World".to_string(),
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