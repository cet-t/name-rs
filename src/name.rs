use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum NameFmt<'a> {
    Raw,
    Template(std::borrow::Cow<'a, str>),
}

impl<'a> NameFmt<'a> {
    pub fn template(fmt: impl Into<std::borrow::Cow<'a, str>>) -> Self {
        Self::Template(fmt.into())
    }
}

impl<'a> AsRef<NameFmt<'a>> for NameFmt<'a> {
    fn as_ref(&self) -> &NameFmt<'a> {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Name {
    value: String,
}

impl Name {
    #[must_use]
    pub fn show<'a, 'b, F: AsRef<NameFmt<'b>>>(&'a self, fmt: F) -> std::borrow::Cow<'a, str> {
        let fmt = fmt.as_ref();
        match fmt {
            NameFmt::Raw => Cow::Borrowed(&self.value),
            NameFmt::Template(fmt) => {
                let vars = [("name".to_owned(), self.value.as_str())];
                ::strfmt::strfmt(fmt, &vars.into())
                    .map(Cow::Owned)
                    .unwrap_or(Cow::Borrowed(&self.value))
            }
        }
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}
