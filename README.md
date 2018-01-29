Mail-rs
===========

This repository provides a library to interact with some popular e-mail services in Rust.

Currently it can send e-mails through the Mailgun interface.
Those e-mails can be static text or generated from a handlebars template.

## Example usage

```rust
let mailgun = Mailgun::new(domain, api_key);
let message = Message::new(from, to, subject, body)
message.send_using(mailgun);
```

The `send_using` function accepts any type that implements the `Sender` trait. This way new sending methods can be added easily in the future.


## License 
LGPL
