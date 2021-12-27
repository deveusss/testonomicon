pub mod assertions;
use colors::*;
use std::borrow::Borrow;
use std::fmt::Debug;

#[cfg(not(test))]
mod colors {
    pub const RED: &'static str = "\x1B[31m";
    pub const BOLD: &'static str = "\x1B[1m";
    pub const RESET: &'static str = "\x1B[0m";
}

#[cfg(test)]
mod colors {
    pub const RED: &'static str = "";
    pub const BOLD: &'static str = "";
    pub const RESET: &'static str = "";
}

#[derive(Debug)]
pub struct Assert<'a, C> {
    pub component: &'a C,
    pub component_name: Option<&'a str>,
    pub location: Option<String>,
    pub description: Option<&'a str>,
}

#[derive(Debug)]
pub struct AssertionFailure<'a, C: 'a> {
    component: &'a C,
    expected: Option<String>,
    actual: Option<String>,
}
pub struct Assertion{
    
}
pub struct AssertDescription<'a> {
    value: &'a str,
    location: Option<String>,
}
pub trait AssertDescriptor<'a> {
    fn component_name(&self) -> Option<&'a str>;
    fn location(&self) -> Option<String>;
    fn description(&self) -> Option<&'a str>;
}

pub fn assertion(description: Option<&str>) -> AssertDescription {
    AssertDescription {
        value: description.expect("Invalid assert description"),
        location: None,
    }
}
pub fn assert() -> Assertion {
    Assertion {
        
    }
}
impl<'a> Assertion{
     pub fn that<C>(self, component: &'a C) -> Assert<'a, C> {
        Assert {
            component: component,
            component_name: None,
            description: None,
            location:None,
        }
    }
}

impl<'a> AssertDescription<'a> {
    pub fn with_location(self, location: String) -> Self {
        let mut description = self;

        description.location = Some(location);
        description
    }

    pub fn that<C>(self, component: &'a C) -> Assert<'a, C> {
        Assert {
            component: component,
            component_name: None,
            location: self.location,
            description: Some(self.value),
        }
    }
}
impl<'a, C> Assert<'a, C> {
    pub fn that(&self, component: &'a C) -> Assert<'a, C> {
        Assert {
            component: component,
            component_name: None,
            location: None,
            description: None,
        }
    }
    pub fn with_location(self, location: String) -> Self {
        let mut assert = self;
        assert.location = Some(location);
        assert
    }
    pub fn named(self, component_name: &'a str) -> Self {
        let mut assert = self;
        assert.component_name = Some(component_name);
        assert
    }
}
impl<'a, T> AssertDescriptor<'a> for Assert<'a, T> {
    fn component_name(&self) -> Option<&'a str> {
        self.component_name
    }

    fn location(&self) -> Option<String> {
        self.location.clone()
    }

    fn description(&self) -> Option<&'a str> {
        self.description
    }
}

impl<'a, A: AssertDescriptor<'a>> AssertionFailure<'a, A> {
    pub fn from(component: &'a A) -> AssertionFailure<'a, A> {
        AssertionFailure {
            component: component,
            expected: None,
            actual: None,
        }
    }

    pub fn with_expected(&mut self, expected: String) -> &mut Self {
        let mut assertion = self;
        assertion.expected = Some(expected);

        assertion
    }

    pub fn with_actual(&mut self, actual: String) -> &mut Self {
        let mut assertion = self;
        assertion.actual = Some(actual);

        assertion
    }

    pub fn fail(&mut self) {
        if !self.expected.is_some() || !self.actual.is_some() {
            panic!("Invalid assertion info");
        }

        let location = self.location_formatted();
        let component_name = self.component_name_formatted();
        let description = self.description_formatted();

        panic!(
            "{}{}\n\t{}expected: {}\n\t but was: {}{}\n{}",
            description,
            component_name,
            RED,
            self.expected.clone().unwrap(),
            self.actual.clone().unwrap(),
            RESET,
            location
        )
    }

    fn fail_with_message(&mut self, message: String) {
        let location = self.location_formatted();
        let component_name = self.component_name_formatted();
        let description = self.description_formatted();

        panic!(
            "{}{}\n\t{}{}{}\n{}",
            description, component_name, RED, message, RESET, location
        )
    }

    fn location_formatted(&self) -> String {
        match self.component.location() {
            Some(value) => format!("\n\t{}at location: {}{}\n", BOLD, value, RESET),
            None => "".to_string(),
        }
    }

    fn description_formatted(&self) -> String {
        match self.component.description() {
            Some(value) => format!("\n\t{}:{}{}", BOLD, value, RESET),
            None => "".to_string(),
        }
    }

    fn component_name_formatted(&self) -> String {
        match self.component.component_name() {
            Some(value) => format!("\n\t{}for component [{}]{}", BOLD, value, RESET),
            None => "".to_string(),
        }
    }
}

impl<'a, C> Assert<'a, C>
where
    C: Debug + PartialEq,
{
    pub fn is_equal_to<E: Borrow<C>>(&mut self, expected: E) {
        let subject = self.component;
        let borrowed_expected = expected.borrow();

        if !subject.eq(borrowed_expected) {
            AssertionFailure::from(self)
                .with_expected(format!("<{:?}>", borrowed_expected))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    pub fn is_not_equal_to<E: Borrow<C>>(&mut self, expected: E) {
        let subject = self.component;
        let borrowed_expected = expected.borrow();

        if subject.eq(borrowed_expected) {
            AssertionFailure::from(self)
                .with_expected(format!(
                    "<{:?}> to not equal <{:?}>",
                    subject, borrowed_expected
                ))
                .with_actual(format!("equal"))
                .fail();
        }
    }
}
