# nem2-rs
[NEM Catapult (NEM2)](https://nem.io/catapult/) Client for [Rust](https://www.rust-lang.org/)

*This crate is highly under development.*

## Usage
```
nem2 = { git = "https://github.com/hhatto/nem2-rs" }
```

```
use nem2::block;

fn main() {
    let client = block::BlockHttp::new("http://localhost:3000");

    let resp = client.get_block_by_height(1);
    match resp {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("error: {:?}", e),
    }
}
```

## License
Apache-2.0 or MIT