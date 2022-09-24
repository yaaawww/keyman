<h1 align="center"> Keyman </h1>
<p align="center">
    <img src="./doc/rust_key.jpg" width=40% height=40%/>
</p>

A fast password manager written in Rust.

However, this is just the backend, if you want to use, you may need a application to use the api.

I recommend you to use the front [project](https://github.com/Jacen-cpu/keyman-front) by myself. 

## Main Algorithm

we use PBKDF2 to the key.<br/>

<img src="https://www.waysoahc.xyz/demo_slides/keyman/assets/gen.jpg" width=70% height=70% />

we use AES to enc user's password.

<img src="https://www.waysoahc.xyz/demo_slides/keyman/assets/enc.jpg" width=70% height=70% />

This is how we dec the cipher.

<img src="https://www.waysoahc.xyz/demo_slides/keyman/assets/dec.jpg" />

by the way

you can see more from my [slide](https://www.waysoahc.xyz/demo_slides/keyman/index.html#/).

## Usage
```rust
cargo run
```

## License

GNU General Public License v3.0