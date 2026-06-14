use std::fmt;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InputPath {
    maybe_filename: Option<String>,
    maybe_line_number: Option<usize>,
    segment_list: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    Key(String),
    Index(usize),
}

impl InputPath {
    pub fn root() -> Self {
        Self {
            maybe_filename: None,
            maybe_line_number: None,
            segment_list: Vec::with_capacity(4),
        }
    }

    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.maybe_filename = Some(filename.into());
        self
    }

    pub fn with_line_number(mut self, line_number: usize) -> Self {
        self.maybe_line_number = Some(line_number);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.segment_list.push(PathSegment::Key(key.into()));
        self
    }

    pub fn with_index(mut self, index: usize) -> Self {
        self.segment_list.push(PathSegment::Index(index));
        self
    }

    pub fn is_empty(&self) -> bool {
        self.maybe_filename.is_none()
            && self.maybe_line_number.is_none()
            && self.segment_list.is_empty()
    }
}

impl fmt::Display for InputPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut has_path = false;
        if let Some(filename) = &self.maybe_filename {
            write!(f, "{filename}")?;
            has_path = true;
        }
        let mut has_line_number = false;
        if let Some(line_number) = &self.maybe_line_number {
            if has_path {
                write!(f, ":{line_number}")?;
            } else {
                write!(f, "{line_number}")?;
            }
            has_line_number = true;
        }
        if has_line_number && !self.segment_list.is_empty() {
            write!(f, ":")?;
        }
        for segment in &self.segment_list {
            match segment {
                PathSegment::Key(key) => write!(f, "[{key}]")?,
                PathSegment::Index(index) => write!(f, "[{index}]")?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::InputPath;

    #[test]
    fn root_is_empty() {
        assert!(InputPath::root().is_empty());
        assert_eq!(InputPath::root().to_string(), "");
    }

    #[test]
    fn segment_path_display() {
        let path = InputPath::root()
            .with_key("foo")
            .with_index(0)
            .with_key("bar");
        assert!(!path.is_empty());
        assert_eq!(path.to_string(), "[foo][0][bar]");
    }

    #[test]
    fn filename_display() {
        let path = InputPath::root().with_filename("config.toml");
        assert!(!path.is_empty());
        assert_eq!(path.to_string(), "config.toml");
    }

    #[test]
    fn line_number_without_filename() {
        let path = InputPath::root().with_line_number(12);
        assert!(!path.is_empty());
        assert_eq!(path.to_string(), "12");
    }

    #[test]
    fn filename_and_line_number() {
        let path = InputPath::root()
            .with_filename("config.toml")
            .with_line_number(42);
        assert_eq!(path.to_string(), "config.toml:42");
    }

    #[test]
    fn line_number_and_segments_use_colon_separator() {
        let path = InputPath::root()
            .with_line_number(3)
            .with_key("items")
            .with_index(1);
        assert_eq!(path.to_string(), "3:[items][1]");
    }

    #[test]
    fn filename_line_number_and_segments() {
        let path = InputPath::root()
            .with_filename("app.json")
            .with_line_number(7)
            .with_key("server")
            .with_key("host");
        assert_eq!(path.to_string(), "app.json:7:[server][host]");
    }

    #[test]
    fn builders_do_not_mutate_cloned_path() {
        let base = InputPath::root().with_key("root");
        let child = base.clone().with_key("child");
        assert_eq!(base.to_string(), "[root]");
        assert_eq!(child.to_string(), "[root][child]");
    }
}
