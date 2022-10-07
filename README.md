<h1 align="center"> Keyman </h1>
<h2 align="center">A fast password manager written in Rust</h2><br>

This is a simple password manager, which is to handle the school internship program. Because It is too simple, so I written it by Rust. But it's common that manary password manager written in Rust.

However, this is just the backend, if you want to use, you may need a application to use the api.
I recommend you to use the front [project](https://github.com/Jacen-cpu/keyman-front) by myself. 

## Main Construction

<p align="center">
<img src="./doc/construction.png" />
</p>

## Main Algorithm

<p align="center">
we use PBKDF2 to the key.<br/>
<img src="./doc/gen.jpg" width=70% height=70% />
</p>

<p align="center">
we use AES to enc user's password.<br>
<img src="./doc/enc.jpg" align="center" width=70% height=70% />
</p>

<p align="center">
This is how keyman dec the cipher.
<img src="./doc/dec.jpg" />
</p>

**by the way you can see more design detail from my [slide](https://www.waysoahc.xyz/demo_slides/keyman/index.html#/).**

## Usage

```rust
cargo run
```

## Basic api

```
(add_key)           POST    /api/ application/json
(get_all_key)       GET     /api/
(update_key)        PUT     /api/ application/json
(delete_key)        DELETE  /api/<id>
(login_master)      POST    /api/login application/json
(register_password) POST    /api/register application/json
(get_key)           GET     /api/<site_addr>
```

## License

GNU General Public License v3.0