// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFunctionInput {
    /// <p>The name of the Lambda function or version.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub function_name: ::std::option::Option<::std::string::String>,
    /// <p>Specify a version to delete. You can't delete a version that an alias references.</p>
    pub qualifier: ::std::option::Option<::std::string::String>,
}
impl DeleteFunctionInput {
    /// <p>The name of the Lambda function or version.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(&self) -> ::std::option::Option<&str> {
        self.function_name.as_deref()
    }
    /// <p>Specify a version to delete. You can't delete a version that an alias references.</p>
    pub fn qualifier(&self) -> ::std::option::Option<&str> {
        self.qualifier.as_deref()
    }
}
impl DeleteFunctionInput {
    /// Creates a new builder-style object to manufacture [`DeleteFunctionInput`](crate::operation::delete_function::DeleteFunctionInput).
    pub fn builder() -> crate::operation::delete_function::builders::DeleteFunctionInputBuilder {
        crate::operation::delete_function::builders::DeleteFunctionInputBuilder::default()
    }
}

/// A builder for [`DeleteFunctionInput`](crate::operation::delete_function::DeleteFunctionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteFunctionInputBuilder {
    pub(crate) function_name: ::std::option::Option<::std::string::String>,
    pub(crate) qualifier: ::std::option::Option<::std::string::String>,
}
impl DeleteFunctionInputBuilder {
    /// <p>The name of the Lambda function or version.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    /// This field is required.
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Lambda function or version.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_name = input;
        self
    }
    /// <p>The name of the Lambda function or version.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.function_name
    }
    /// <p>Specify a version to delete. You can't delete a version that an alias references.</p>
    pub fn qualifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.qualifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify a version to delete. You can't delete a version that an alias references.</p>
    pub fn set_qualifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.qualifier = input;
        self
    }
    /// <p>Specify a version to delete. You can't delete a version that an alias references.</p>
    pub fn get_qualifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.qualifier
    }
    /// Consumes the builder and constructs a [`DeleteFunctionInput`](crate::operation::delete_function::DeleteFunctionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_function::DeleteFunctionInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_function::DeleteFunctionInput {
            function_name: self.function_name,
            qualifier: self.qualifier,
        })
    }
}
