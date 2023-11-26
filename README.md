# Plugx Input (work-in-progress)
A simple and flexible data-structure for configuration and state manipulation of plugins.

[**Package**](https://crates.io/crates/plugx-input)   |   [**Documentation**](https://docs.rs/plugx-input)   |   [**Repository**](https://github.com/plugx-rs/plugx-input)

<br/>

## Demo
```rust
use plugx_input::Input;

let mut map = Input::new_map();
map.map_mut().insert("key".into(), Input::from([1, 2, 3]));
let inner_list = map
    .map_mut()       // &mut Hashmap<String, Input>
    .get_mut("key")  // Option<&mut Input>
    .unwrap()        // &mut Input (which is a list)
    .list_mut();     // &mut Vec<Input>
*inner_list.get_mut(0).unwrap() = 3.14.into();
*inner_list.get_mut(1).unwrap() = true.into();
*inner_list.get_mut(2).unwrap() = "hello world".into();
println!("{map}");
// prints:
// {"key": [3.14, true, "hello world"]}
```

## Features

### Diff
```rust
use plugx_input::Input;
use plugx_input::diff::{diff, InputDiff};

let mut map = Input::new_map();                       // {}
map.map_mut().insert("foo".into(), Input::new_map()); // {"foo": {}}
map                                                   // {"foo": {"bar": [50, 60, 70]}}
    .map_mut()
    .get_mut("foo").unwrap()
    .map_mut()
    .insert("bar".into(), Input::from([50, 60, 70]));

let mut map2 = map.clone();                           // {"foo": {"bar": [50,  60, 70]}}
*map2                                                 // {"foo": {"bar": [100, 60, 70]}}
    .map_mut()
    .get_mut("foo").unwrap()
    .map_mut()
    .get_mut("bar").unwrap()
    .list_mut()
    .get_mut(0).unwrap()
    .int_mut() = 100;

diff(
    &map,
    &map2,
    &mut |diff: InputDiff| {
        println!("value {} {}", diff.position(), diff.action())
    }
);
// prints:
//     value [foo][bar][0] increased by 50
```


### Merge
```rust
use plugx_input::Input;
use plugx_input::merge::merge;

let mut map = Input::new_map();                       // {}
map.map_mut().insert("foo".into(), Input::new_map()); // {"foo": {}}
map                                                   // {"foo": {"bar": false}}
    .map_mut()
    .get_mut("foo").unwrap()
    .map_mut()
    .insert("bar".into(), false.into());

let mut map2 = Input::new_map();                       // {}
map2.map_mut().insert("foo".into(), Input::new_map()); // {"foo": {}}
map2                                                   // {"foo": {"baz": true}}
    .map_mut()
    .get_mut("foo").unwrap()
    .map_mut()
    .insert("baz".into(), true.into());

merge(&mut map, &map2);
println!("{map}");
// prints:
//     {"foo": {"bar": false, "baz": true}}
```

## Cargo features
* **default**: Nothing!  
* **schema**: Enables schema and validation `Input`.  
* **logging**: Enables logging via [log](https://docs.rs/log/latest/log/) crate.  
* **tracing**: Enables logging via [tracing](https://docs.rs/tracing/latest/tracing/) crate.

# To contributors
I ❤️ PR from everyone, and I appreciate your help but before opening a PR, file an issue and describe your feature, fix, etc.
