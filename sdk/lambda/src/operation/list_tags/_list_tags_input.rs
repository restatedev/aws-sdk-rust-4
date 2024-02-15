// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListTagsInput {
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub resource: ::std::option::Option<::std::string::String>,
}
impl ListTagsInput {
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
}
impl ListTagsInput {
    /// Creates a new builder-style object to manufacture [`ListTagsInput`](crate::operation::list_tags::ListTagsInput).
    pub fn builder() -> crate::operation::list_tags::builders::ListTagsInputBuilder {
        crate::operation::list_tags::builders::ListTagsInputBuilder::default()
    }
}

/// A builder for [`ListTagsInput`](crate::operation::list_tags::ListTagsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListTagsInputBuilder {
    pub(crate) resource: ::std::option::Option<::std::string::String>,
}
impl ListTagsInputBuilder {
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    /// This field is required.
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn get_resource(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource
    }
    /// Consumes the builder and constructs a [`ListTagsInput`](crate::operation::list_tags::ListTagsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_tags::ListTagsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_tags::ListTagsInput { resource: self.resource })
    }
}
