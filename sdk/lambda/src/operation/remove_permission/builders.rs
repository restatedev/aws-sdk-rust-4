// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_permission::_remove_permission_output::RemovePermissionOutputBuilder;

pub use crate::operation::remove_permission::_remove_permission_input::RemovePermissionInputBuilder;

impl RemovePermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_permission::RemovePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_permission::RemovePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemovePermission`.
///
/// <p>Revokes function-use permission from an Amazon Web Service or another Amazon Web Services account. You can get the ID of the statement from the output of <code>GetPolicy</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemovePermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_permission::builders::RemovePermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_permission::RemovePermissionOutput,
        crate::operation::remove_permission::RemovePermissionError,
    > for RemovePermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_permission::RemovePermissionOutput,
            crate::operation::remove_permission::RemovePermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemovePermissionFluentBuilder {
    /// Creates a new `RemovePermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemovePermission as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_permission::builders::RemovePermissionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::remove_permission::RemovePermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_permission::RemovePermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_permission::RemovePermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_permission::RemovePermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_permission::RemovePermissionOutput,
        crate::operation::remove_permission::RemovePermissionError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name of the Lambda function, version, or alias.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>
    /// <li>
    /// <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li>
    /// </ul>
    /// <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
    /// <p>Statement ID of the permission to remove.</p>
    pub fn statement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>Statement ID of the permission to remove.</p>
    pub fn set_statement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>Statement ID of the permission to remove.</p>
    pub fn get_statement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_id()
    }
    /// <p>Specify a version or alias to remove permissions from a published version of the function.</p>
    pub fn qualifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.qualifier(input.into());
        self
    }
    /// <p>Specify a version or alias to remove permissions from a published version of the function.</p>
    pub fn set_qualifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_qualifier(input);
        self
    }
    /// <p>Specify a version or alias to remove permissions from a published version of the function.</p>
    pub fn get_qualifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_qualifier()
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
    /// <p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn get_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_revision_id()
    }
}
