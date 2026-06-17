# Plugx Input (work-in-progress)
A simple and flexible data-structure for configuration and state manipulation of plugins.

[**Package**](https://crates.io/crates/plugx-input)   |   [**Documentation**](https://docs.rs/plugx-input)   |   [**Repository**](https://github.com/plugx-rs/plugx-input)

<br/>

## Features

| Feature | Enables |
|---------|---------|
| *(none)* | Core [`Input`] type, `From` conversions, [`Display`] |
| `serde` | [`Serialize`]/[`Deserialize`], infallible [`Input::serialize`] |
| `rkyv` | Binary archive via [`Input::to_rkyv_bytes`] / [`Input::from_rkyv_bytes`] |

`serde` or `rkyv` also exposes [`error`] and [`position`] for structured deserialize errors.

```toml
plugx-input = { version = "1.1", features = ["serde"] }
```

## Demo
```rust
use plugx_input::Input;

let mut map = Input::new_map();
map.map_mut().unwrap().insert("key".into(), Input::from([1, 2, 3]));
let inner_list = map
    .map_mut().unwrap()   // &mut Hashmap<String, Input>
    .get_mut("key")       // Option<&mut Input>
    .unwrap()             // &mut Input (which is a list)
    .list_mut().unwrap(); // &mut Vec<Input>
*inner_list.get_mut(0).unwrap() = 3.14.into();
*inner_list.get_mut(1).unwrap() = true.into();
*inner_list.get_mut(2).unwrap() = "hello world".into();
println!("{map}");
// prints:
// {"key": [3.14, true, "hello world"]}
```

# To contributors
I ❤️ PR from everyone, and I appreciate your help but before opening a PR, file an issue and describe your feature, fix, etc.
