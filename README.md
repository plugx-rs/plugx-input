# Plugx Input (work-in-progress)
A simple and flexible data-structure for configuration and state manipulation of plugins.

[**Package**](https://crates.io/crates/plugx-input)   |   [**Documentation**](https://docs.rs/plugx-input)   |   [**Repository**](https://github.com/plugx-rs/plugx-input)

<br/>

## Demo
```rust
use plugx_input::Input;

let mut map = Input::new_map();
map.map_mut().unwrap().insert("key".into(), Input::from([1, 2, 3]));
let inner_list = map
    .map_mut()       // Option<&mut Hashmap<String, Input>>
    .unwrap()        // &mut Hashmap<String, Input>
    .get_mut("key")  // Option<&mut Input>
    .unwrap()        // &mut Input (which is a list)
    .list_mut()      // Option<&mut Vec<Input>>
    .unwrap();       // &mut Vec<Input>
*inner_list.get_mut(0).unwrap() = 3.14.into();
*inner_list.get_mut(1).unwrap() = true.into();
*inner_list.get_mut(2).unwrap() = "hello world".into();
assert_eq!(format!("{map}"), "{\"key\":[3.14, true, \"hello world\"]}".to_string());
```

## Features

### Diff
```rust
use plugx_input::Input;
use plugx_input::diff::{diff, InputDiff};

let mut map = Input::new_map();                                // {}
map.map_mut().unwrap().insert("foo".into(), Input::new_map()); // {"foo": {}}
map                                                            // {"foo": {"bar": [50, 60, 70]}}
    .map_mut().unwrap()
    .get_mut("foo").unwrap()
    .map_mut().unwrap()
    .insert("bar".into(), Input::from([50, 60, 70]));

let mut map2 = map.clone();                                    // {"foo": {"bar": [50,  60, 70]}}
*map2                                                          // {"foo": {"bar": [100, 60, 70]}}
    .map_mut().unwrap()
    .get_mut("foo").unwrap()
    .map_mut().unwrap()
    .get_mut("bar").unwrap()
    .list_mut().unwrap()
    .get_mut(0).unwrap()
    .int_mut().unwrap() = 100;

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

let mut map = Input::new_map();                                // {}
map.map_mut().unwrap().insert("foo".into(), Input::new_map()); // {"foo": {}}
map                                                            // {"foo": {"bar": false}}
    .map_mut().unwrap()
    .get_mut("foo").unwrap()
    .map_mut().unwrap()
    .insert("bar".into(), false.into());

let mut map2 = Input::new_map();                                // {}
map2.map_mut().unwrap().insert("foo".into(), Input::new_map()); // {"foo": {}}
map2                                                            // {"foo": {"baz": true}}
    .map_mut().unwrap()
    .get_mut("foo").unwrap()
    .map_mut().unwrap()
    .insert("baz".into(), true.into());

merge(&mut map, &map2);
println!("{map}");
// prints:
//     {"foo": {"bar": false, "baz": true}}
```

### Validation with human-readable errors
```rust
    use plugx_input::Input;
    use plugx_input::validation::validate;
    use plugx_input::definition::{InputDefinition, InputDefinitionType};

    let rules_json = r#"
        {
            "type": "static_map",
            "definitions": {
                "foo": {"definition": {"type": "boolean"}},
                "bar": {"definition": {"type": "string"}, "default": "hello world"},
                "baz": {"definition": {"type": "enum", "items": ["x", "y", "z"]}, "default": "y"},
                "qux": {
                    "definition": {
                        "type": "either",
                        "definitions": [
                            {"type": "enum", "items": ["yes", "y", "no", "n"]},
                            {"type": "boolean"}
                        ]
                    }
                }
            }
        }
    "#;
    // Also we could programmatically build `rules`:
    let rules: InputDefinitionType = serde_json::from_str(&rules_json).unwrap();

    let mut map = Input::new_map();
    let error = validate(&mut map, &rules.clone().into(), None).err().unwrap();
    println!("{error}");
    // prints:
    //    qux is not set (expected a value that must be enum with possible values ["yes", "y", "no", "n"] or boolean)

    map.map_mut().unwrap().insert("qux".into(), "yes".into());
    let error = validate(&mut map, &rules.clone().into(), None).err().unwrap();
    println!("{error}");
    // prints:
    //    foo is not set (expected boolean)

    map.map_mut().unwrap().insert("foo".into(), false.into());
    assert!(validate(&mut map, &rules.clone().into(), None).is_ok());

    // Default values:
    assert_eq!(
        map.map_ref().unwrap().get("bar").unwrap().str_ref().unwrap(),
        "hello world"
    );
    assert_eq!(
        map.map_ref().unwrap().get("baz").unwrap().str_ref().unwrap(),
        "y"
    );
```

## Cargo features
* **default**: Nothing!  
* **logging**: Enables logging via [log](https://docs.rs/log/latest/log/) crate.  
* **tracing**: Enables logging via [tracing](https://docs.rs/tracing/latest/tracing/) crate.

# To contributors
I ❤️ PR from everyone, and I appreciate your help but before opening a PR, file an issue and describe your feature, fix, etc.
