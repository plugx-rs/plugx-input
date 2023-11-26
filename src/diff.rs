use crate::{position::InputPosition, Input};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize)]
pub struct InputDiff {
    input: Input,
    position: InputPosition,
    maybe_old_value: Option<Input>,
    maybe_new_value: Option<Input>,
    action: InputDiffAction,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum InputDiffAction {
    Added,
    Removed,
    Updated(Option<String>),
}

impl InputDiff {
    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn position(&self) -> &InputPosition {
        &self.position
    }

    pub fn maybe_old_value(&self) -> Option<&Input> {
        self.maybe_old_value.as_ref()
    }

    pub fn maybe_new_value(&self) -> Option<&Input> {
        self.maybe_new_value.as_ref()
    }

    pub fn action(&self) -> &InputDiffAction {
        &self.action
    }
}

impl Display for InputDiff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let position = if self.position.is_empty() {
            "".to_string()
        } else {
            format!("{} ", self.position)
        };
        let description = &self.action;
        let text = match (self.maybe_old_value.as_ref(), self.maybe_new_value.as_ref()) {
            (Some(old_value), Some(new_value)) => {
                format!("{position}value `{old_value}` {description} to new value `{new_value}`")
            }
            (Some(old_value), _) => {
                format!("{position}value `{old_value}` {description}")
            }
            (_, Some(new_value)) => {
                format!("{position}value `{new_value}` {description}")
            }
            _ => format!("{position}has no update"),
        };
        f.write_str(text.as_str())
    }
}

impl Display for InputDiffAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Updated(None) => "updated",
            Self::Updated(Some(updated)) => updated,
        })
    }
}

pub fn diff<F>(input_1: &Input, input_2: &Input, for_each_function: &mut F)
where
    F: FnMut(InputDiff),
{
    diff_with_position(input_1, input_2, for_each_function, InputPosition::new());
}

pub fn diff_with_position<F>(
    input_1: &Input,
    input_2: &Input,
    for_each_function: &mut F,
    position: InputPosition,
) where
    F: FnMut(InputDiff),
{
    if input_1 == input_2 {
        return;
    }
    if input_1.is_map() && input_2.is_map() {
        let (old_map, new_map) = (input_1.as_map(), input_2.as_map());
        for (key, old_value) in old_map {
            let new_position = position.new_with_key(key);
            if let Some(new_value) = new_map.get(key) {
                diff_with_position(old_value, new_value, for_each_function, new_position);
            } else {
                let diff = InputDiff {
                    input: input_1.clone(),
                    position: new_position,
                    maybe_old_value: Some(old_value.clone()),
                    maybe_new_value: None,
                    action: InputDiffAction::Removed,
                };
                for_each_function(diff);
            }
        }
        for (key, new_value) in new_map {
            let new_position = position.new_with_key(key);
            if !old_map.contains_key(key) {
                let diff = InputDiff {
                    input: input_1.clone(),
                    position: new_position,
                    maybe_old_value: None,
                    maybe_new_value: Some(new_value.clone()),
                    action: InputDiffAction::Added,
                };
                for_each_function(diff);
            }
        }
    } else if input_1.is_list() && input_2.is_list() {
        let (old_list, new_list) = (input_1.as_list(), input_2.as_list());
        let (mut added_index_list, mut removed_index_list) = (Vec::new(), Vec::new());
        for (old_index, inner_input) in old_list.iter().enumerate() {
            if !new_list.contains(inner_input) {
                removed_index_list.push(old_index)
            }
        }
        for (new_index, inner_input) in new_list.iter().enumerate() {
            if !old_list.contains(inner_input) {
                added_index_list.push(new_index)
            }
        }
        for added_index in &added_index_list {
            let added_index = *added_index;
            let new_position = position.new_with_index(added_index);
            if removed_index_list.contains(&added_index) {
                diff_with_position(
                    old_list.get(added_index).unwrap(),
                    new_list.get(added_index).unwrap(),
                    for_each_function,
                    new_position,
                );
            } else {
                let diff = InputDiff {
                    input: input_1.clone(),
                    position: new_position,
                    maybe_old_value: None,
                    maybe_new_value: Some(new_list.get(added_index).unwrap().clone()),
                    action: InputDiffAction::Added,
                };
                for_each_function(diff);
            }
        }
        for removed_index in removed_index_list {
            let new_position = position.new_with_index(removed_index);
            if !added_index_list.contains(&removed_index) {
                let diff = InputDiff {
                    input: input_1.clone(),
                    position: new_position,
                    maybe_old_value: Some(old_list.get(removed_index).unwrap().clone()),
                    maybe_new_value: None,
                    action: InputDiffAction::Removed,
                };
                for_each_function(diff);
            }
        }
    } else if input_1.is_str() && input_2.is_str() {
        let diff = InputDiff {
            input: input_1.clone(),
            position,
            maybe_old_value: Some(input_1.clone()),
            maybe_new_value: Some(input_2.clone()),
            action: InputDiffAction::Updated(None),
        };
        for_each_function(diff);
    } else if input_1.is_int() && input_2.is_int() {
        let (old_int, new_int) = (*input_1.as_int(), *input_2.as_int());
        let description = if old_int < new_int {
            format!("increased by {}", new_int - old_int)
        } else {
            format!("decreased by {}", old_int - new_int)
        };
        let diff = InputDiff {
            input: input_1.clone(),
            position,
            maybe_old_value: Some(input_1.clone()),
            maybe_new_value: Some(input_2.clone()),
            action: InputDiffAction::Updated(Some(description)),
        };
        for_each_function(diff);
    } else if input_1.is_float() && input_2.is_float() {
        let (old_float, new_float) = (*input_1.as_float(), *input_2.as_float());
        let description = if old_float < new_float {
            format!("increased by {}", new_float - old_float)
        } else {
            format!("decreased by {}", old_float - new_float)
        };
        let diff = InputDiff {
            input: input_1.clone(),
            position,
            maybe_old_value: Some(input_1.clone()),
            maybe_new_value: Some(input_2.clone()),
            action: InputDiffAction::Updated(Some(description)),
        };
        for_each_function(diff);
    } else if input_1.is_bool() && input_2.is_bool() {
        let diff = InputDiff {
            input: input_1.clone(),
            position,
            maybe_old_value: Some(input_1.clone()),
            maybe_new_value: Some(input_2.clone()),
            action: InputDiffAction::Updated(None),
        };
        for_each_function(diff);
    } else {
        // Changed to different type:
        let diff = InputDiff {
            input: input_1.clone(),
            position,
            maybe_old_value: Some(input_1.clone()),
            maybe_new_value: Some(input_2.clone()),
            action: InputDiffAction::Updated(Some(format!(
                "changed from {} type to {} type",
                input_1.type_name(),
                input_2.type_name()
            ))),
        };
        for_each_function(diff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{enable_logging, info};
    use crate::position::InputPositionType;
    use std::collections::HashMap;

    fn diff_to_list(old_input: &Input, new_input: &Input, print: bool) -> Vec<InputDiff> {
        enable_logging();

        let mut diff_list = Vec::new();
        diff(old_input, new_input, &mut |diff| {
            if print {
                info(format!("\t{diff}"));
            }
            diff_list.push(diff)
        });
        diff_list
    }

    #[test]
    fn functionality() {
        enable_logging();

        let old_input = Input::from(true);
        let mut new_input = Input::from(true);
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.bool_mut() = false;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(!diff_list.is_empty());

        let old_input = Input::from(100);
        let mut new_input = Input::from(100);
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.int_mut() = -100;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(!diff_list.is_empty());
        assert_eq!(
            diff_list[0].action.to_string(),
            "decreased by 200".to_string()
        );

        let old_input = Input::from(-1.5);
        let mut new_input = Input::from(-1.5);
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.float_mut() = 3.0;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(!diff_list.is_empty());
        assert_eq!(
            diff_list[0].action.to_string(),
            "increased by 4.5".to_string()
        );

        let old_input = Input::from("foo");
        let mut new_input = Input::from("foo");
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.str_mut() = "bar".to_string();
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(!diff_list.is_empty());
        assert_eq!(diff_list[0].action, InputDiffAction::Updated(None));

        let old_input = Input::from(Vec::from([
            Input::from(true),
            Input::from(10),
            Input::from(0.5),
            Input::from("foo"),
        ]));
        let mut new_input = Input::from(Vec::from([
            Input::from(true),
            Input::from(10),
            Input::from(0.5),
            Input::from("foo"),
        ]));
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.list_mut()[0].bool_mut() = false;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert_eq!(diff_list.len(), 1);
        assert_eq!(
            diff_list[0].position.clone()[0],
            InputPositionType::Index(0)
        );
        *new_input.list_mut()[1].int_mut() = 20;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert_eq!(diff_list.len(), 2);
        assert_eq!(
            diff_list[1].position.clone()[0],
            InputPositionType::Index(1)
        );
        *new_input.list_mut()[2].float_mut() = -0.5;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert_eq!(diff_list.len(), 3);
        assert_eq!(
            diff_list[2].position.clone()[0],
            InputPositionType::Index(2)
        );
        *new_input.list_mut()[3].str_mut() = "bar".to_string();
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert_eq!(diff_list.len(), 4);
        assert_eq!(
            diff_list[3].position.clone()[0],
            InputPositionType::Index(3)
        );

        let old_input = Input::from(HashMap::from([(
            "foo".to_string(),
            Input::from(Vec::from([
                Input::from(true),
                Input::from(10),
                Input::from(0.5),
                Input::from("foo"),
            ])),
        )]));
        let mut new_input = Input::from(HashMap::from([(
            "foo".to_string(),
            Input::from(Vec::from([
                Input::from(true),
                Input::from(10),
                Input::from(0.5),
                Input::from("foo"),
            ])),
        )]));
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(diff_list.is_empty());
        *new_input.map_mut().get_mut("foo").unwrap().list_mut()[1].int_mut() = 100;
        let diff_list = diff_to_list(&old_input, &new_input, false);
        assert!(!diff_list.is_empty());
        assert_eq!(
            diff_list[0].position.clone()[0],
            InputPositionType::Key("foo".to_string())
        );
        assert_eq!(
            diff_list[0].position.clone()[1],
            InputPositionType::Index(1)
        );
    }

    #[test]
    fn print() {
        enable_logging();

        let json = serde_json::json!({"foo": {"bar": {"baz": [-10, 1.5, false, "bar"]}}});
        let input: Input = serde_json::from_value(json.clone()).unwrap();
        let mut new_input = input.clone();
        let list = new_input
            .map_mut()
            .get_mut("foo")
            .unwrap()
            .map_mut()
            .get_mut("bar")
            .unwrap()
            .map_mut()
            .get_mut("baz")
            .unwrap()
            .list_mut();
        *list[0].int_mut() = 10;
        *list[1].float_mut() = -1.5;
        list[2] = Input::from("new string");
        list.remove(3);
        info(format!("Original data: {}", json.to_string()));
        info(format!(
            "Updated data:  {}",
            serde_json::to_string(&new_input).unwrap()
        ));
        info(format!("Updates:"));
        diff_to_list(&input, &new_input, true);
    }
}
