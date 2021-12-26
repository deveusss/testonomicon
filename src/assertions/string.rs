use crate::*;
pub trait StrAssertions {
    fn starts_with<'a, E: Borrow<&'a str>>(&mut self, expected: E);
    fn ends_with<'a, E: Borrow<&'a str>>(&mut self, expected: E);
    fn contains<'a, E: Borrow<&'a str>>(&mut self, expected: E);
    fn is_empty(&mut self);
}


impl<'s> StrAssertions for Assert<'s, &'s str> {
    fn starts_with<'a,E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = self.component;
        starts_with(self, component, expected);
    }

    fn ends_with<'a, E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = self.component;
        ends_with(self, component, expected);
    }

    fn contains<'a, E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = self.component;
        contains(self, component, expected);
    }

    fn is_empty(&mut self) {
        let component = self.component;
        is_empty(self, component);
    }
}

impl<'s> StrAssertions for Assert<'s, String> {
    fn starts_with<'a, E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = &self.component;
        starts_with(self, component, expected);
    }

    fn ends_with<'a, E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = &self.component;
        ends_with(self, component, expected);
    }

    fn contains<'a, E: Borrow<&'a str>>(&mut self, expected: E) {
        let component = &self.component;
        contains(self, component, expected);
    }

    fn is_empty(&mut self) {
        let component = &self.component;
        is_empty(self, component);
    }
}

fn starts_with<'a, 's, DE: AssertDescriptor<'s>, E: Borrow<&'a str>>(
    descriptor: &'s DE,
    component: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !component.starts_with(borrowed_expected) {
        AssertionFailure::from(descriptor)
            .with_expected(format!("string starting with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", component))
            .fail();
    }
}

fn ends_with<'a, 's, AD: AssertDescriptor<'s>, E: Borrow<&'a str>>(
    descriptor: &'s AD,
    component: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !component.ends_with(borrowed_expected) {
        AssertionFailure::from(descriptor)
            .with_expected(format!("string ending with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", component))
            .fail();
    }
}

fn contains<'a, 's, AD: AssertDescriptor<'s>, E: Borrow<&'a str>>(
    descriptor: &'s AD,
    component: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !component.contains(borrowed_expected) {
        AssertionFailure::from(descriptor)
            .with_expected(format!("string containing <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", component))
            .fail();
    }
}

fn is_empty<'a, Descriptor: AssertDescriptor<'a>>(descriptor: &'a Descriptor, component: &str) {
    if !component.is_empty() {
        AssertionFailure::from(descriptor)
            .with_expected(format!("an empty string"))
            .with_actual(format!("<{:?}>", component))
            .fail();
    }
}

