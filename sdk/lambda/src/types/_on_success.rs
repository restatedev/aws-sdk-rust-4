// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A destination for events that were processed successfully.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OnSuccess {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub destination: ::std::option::Option<::std::string::String>,
}
impl OnSuccess {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
}
impl OnSuccess {
    /// Creates a new builder-style object to manufacture [`OnSuccess`](crate::types::OnSuccess).
    pub fn builder() -> crate::types::builders::OnSuccessBuilder {
        crate::types::builders::OnSuccessBuilder::default()
    }
}

/// A builder for [`OnSuccess`](crate::types::OnSuccess).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct OnSuccessBuilder {
    pub(crate) destination: ::std::option::Option<::std::string::String>,
}
impl OnSuccessBuilder {
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
    /// Consumes the builder and constructs a [`OnSuccess`](crate::types::OnSuccess).
    pub fn build(self) -> crate::types::OnSuccess {
        crate::types::OnSuccess {
            destination: self.destination,
        }
    }
}
