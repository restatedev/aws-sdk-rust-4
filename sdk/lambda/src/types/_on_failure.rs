// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A destination for events that failed processing.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OnFailure {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub destination: ::std::option::Option<::std::string::String>,
}
impl OnFailure {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
}
impl OnFailure {
    /// Creates a new builder-style object to manufacture [`OnFailure`](crate::types::OnFailure).
    pub fn builder() -> crate::types::builders::OnFailureBuilder {
        crate::types::builders::OnFailureBuilder::default()
    }
}

/// A builder for [`OnFailure`](crate::types::OnFailure).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct OnFailureBuilder {
    pub(crate) destination: ::std::option::Option<::std::string::String>,
}
impl OnFailureBuilder {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub fn get_destination(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination
    }
    /// Consumes the builder and constructs a [`OnFailure`](crate::types::OnFailure).
    pub fn build(self) -> crate::types::OnFailure {
        crate::types::OnFailure {
            destination: self.destination,
        }
    }
}
