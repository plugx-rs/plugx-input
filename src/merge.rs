use crate::{is_trace_level_enabled, position::InputPosition, Input};
use cfg_if::cfg_if;

macro_rules! trace_merge {
    ($to_be_merged_input_position:expr, $to_be_merged_input:expr, $input_position:expr, $input:expr, $action:expr) => {
        cfg_if! {
            if #[cfg(feature = "tracing")] {
                tracing::trace!(
                    position = %$input_position,
                    old_value = %$input,
                    from = %$to_be_merged_input_position,
                    new_value = %$to_be_merged_input,
                    $action
                );
            } else if #[cfg(feature = "logging")] {
                log::trace!(
                    "position={:?} old_value={:?} from={:?} new_value={:?} message={:?}",
                    $input_position.to_string(),
                    $input.to_string(),
                    $to_be_merged_input_position.to_string(),
                    $to_be_merged_input.to_string(),
                    $action,
                );
            }
        }
    };
}

pub fn merge(input: &mut Input, to_be_merged_input: &Input) {
    merge_with_positions(
        input,
        InputPosition::new(),
        to_be_merged_input,
        InputPosition::new(),
    )
}

pub fn merge_with_positions(
    input: &mut Input,
    input_position: InputPosition,
    to_be_merged_input: &Input,
    to_be_merged_input_position: InputPosition,
) {
    if input.is_map() {
        merge_map(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else if input.is_list() {
        merge_list(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else if input.is_str() {
        merge_str(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else if input.is_float() {
        merge_float(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else if input.is_int() {
        merge_int(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else if input.is_bool() {
        merge_bool(
            input,
            input_position,
            to_be_merged_input,
            to_be_merged_input_position,
        )
    } else {
        unreachable!("{input:?}!!!")
    }
}

fn merge_map(
    input: &mut Input,
    input_position: InputPosition,
    to_be_merged_input: &Input,
    to_be_merged_input_position: InputPosition,
) {
    if !(to_be_merged_input.is_map() && input.is_map()) {
        trace_merge!(
            to_be_merged_input_position,
            to_be_merged_input,
            input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
        return;
    }
    let map = input.map_mut();
    let to_be_merged_map = to_be_merged_input.as_map();
    for (key, inner_to_be_merged_input) in to_be_merged_map {
        if let Some(inner_input) = map.get_mut(key) {
            merge_with_positions(
                inner_input,
                input_position.new_with_key(key),
                inner_to_be_merged_input,
                to_be_merged_input_position.new_with_key(key),
            );
        } else {
            map.insert(key.clone(), inner_to_be_merged_input.clone());
        }
    }
}

fn merge_list(
    input: &mut Input,
    _input_position: InputPosition,
    to_be_merged_input: &Input,
    _to_be_merged_input_position: InputPosition,
) {
    if !(to_be_merged_input.is_list() && input.is_list()) {
        trace_merge!(
            _to_be_merged_input_position,
            to_be_merged_input,
            _input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
        return;
    }
    let mut _input_clone = input.clone();
    let list = input.list_mut();
    let to_be_merged_list = to_be_merged_input.as_list();
    for (_index, inner_to_be_merged_input) in to_be_merged_list.iter().enumerate() {
        if !list.contains(inner_to_be_merged_input) {
            if is_trace_level_enabled!() {
                _input_clone
                    .list_mut()
                    .push(inner_to_be_merged_input.clone());
                trace_merge!(
                    _to_be_merged_input_position.new_with_index(_index),
                    inner_to_be_merged_input,
                    _input_position,
                    _input_clone,
                    "appended"
                );
            }
            list.push(inner_to_be_merged_input.clone());
        }
    }
}

fn merge_str(
    input: &mut Input,
    _input_position: InputPosition,
    to_be_merged_input: &Input,
    _to_be_merged_input_position: InputPosition,
) {
    if input != to_be_merged_input {
        trace_merge!(
            _to_be_merged_input_position,
            to_be_merged_input,
            _input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
    }
}

fn merge_float(
    input: &mut Input,
    _input_position: InputPosition,
    to_be_merged_input: &Input,
    _to_be_merged_input_position: InputPosition,
) {
    if input != to_be_merged_input {
        trace_merge!(
            _to_be_merged_input_position,
            to_be_merged_input,
            _input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
    }
}

fn merge_int(
    input: &mut Input,
    _input_position: InputPosition,
    to_be_merged_input: &Input,
    _to_be_merged_input_position: InputPosition,
) {
    if input != to_be_merged_input {
        trace_merge!(
            _to_be_merged_input_position,
            to_be_merged_input,
            _input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
    }
}

fn merge_bool(
    input: &mut Input,
    _input_position: InputPosition,
    to_be_merged_input: &Input,
    _to_be_merged_input_position: InputPosition,
) {
    if input != to_be_merged_input {
        trace_merge!(
            _to_be_merged_input_position,
            to_be_merged_input,
            _input_position,
            input,
            "replaced"
        );
        *input = to_be_merged_input.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::enable_logging;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        enable_logging();

        let mut map = HashMap::new();
        map.insert(
            "foo",
            Input::from(HashMap::from([
                ("key", Input::from("value")),
                (
                    "list",
                    Input::from([Input::from(1), Input::from(2), Input::from(3)]),
                ),
                ("number", Input::from(3.14)),
            ])),
        );
        let mut input = Input::from(map);
        let position = InputPosition::new().new_with_key("first");

        let mut to_be_merged_map = HashMap::new();
        to_be_merged_map.insert(
            "foo",
            Input::from(HashMap::from([
                ("key", Input::from("new value")),
                ("new key", Input::from("value")),
                (
                    "list",
                    Input::from([Input::from(1), Input::from(10), Input::from(3)]),
                ),
                ("number", Input::from(0.0)),
            ])),
        );
        let to_be_merged_input = Input::from(to_be_merged_map);
        let to_be_merged_position = InputPosition::new().new_with_key("second");
        merge_with_positions(
            &mut input,
            position,
            &to_be_merged_input,
            to_be_merged_position,
        );
    }
}
