Exposes a utility function to reduce boilerplate for reading from stdin.

# Yes

```rust
use std::io::Result;
use stdinix::stdinix;

fn main() -> Result<()> {
    stdinix(|buf| {
        println!("{}", buf.rev().collect());

        Ok(())
    })
}
```


# No

```rust
use std::io::Result;

fn main() -> Result<()> {
    let mut buf = String::new();
    while let Ok(true) = {
        buf.clear();
        std::io::stdin().read_line(&mut buf).map(|l| l > 0)
    } {
        println!("{}", buf.rev().collect());
    }

    Ok(())
}
```

# Async

```rust
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let context = vec![0, 3];

    astdinix(move |line| {
        let context = context.clone();

        async move {
            println!("{} {}", line.bytes().nth(context[0]).unwrap(), line.bytes().nth(context[1]).unwrap());
            Ok(())
        }
    })
    .await
}
```
