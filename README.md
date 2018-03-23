# Running-Go (Rust)

## Install Instruction

1.

```
rustup toolchain install nightly
```

2.

```
rustup override set nightly
```

3.

```
APIKEY x2

=>

./running-go/src/constant.rs
```

3.

```
running-go> cargo build --release
```

4.

```
running-go> cd target/release
```

## Usage Example

1.

```
running-go -h
```

2.

```
release> running-go generate user/HanMeiMei --username 139xxxxxx32 --password ILoveLeLei --lat 23.123456 --lon 125.123456
    Done Parse argument
    Done Generate user config
```

3.

```
release> running-go run user/HanMeiMei -d 5001 -t "2018/1/1 23:59:59"
    Done Parse argument
    Done Load user config
    Done Login
    Done Fetch point
    Done Plan route
    Done Get captcha
    Done Hack captcha
    Done Validate captcha
    Done Upload record
    Done Logout
```

## FAQ

1.

```
error[E0554]: #[feature] may not be used on the stable release channel
```

```
rustup toolchain install nightly
rustup override set nightly
```

2.

```
Error Fetch point - Error occured: Api("请求不合法")
```

```rust
src> cat constant.rs
6   ...
7   pub const SEL_DISTANCE: u64 = 2000;
8   ...
```

3.

```
Error Plan route - Error occured: Api("APP不存在，AK有误请检查再重试")
```

```rust
src> cat constant.rs
19   ...
20   pub const API_KEY_CAPTCHA: &'static str = "???";
21   pub const API_KEY_BAIDU: &'static str = "???";
22   ...
```