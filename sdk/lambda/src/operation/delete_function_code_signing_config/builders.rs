// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_function_code_signing_config::_delete_function_code_signing_config_output::DeleteFunctionCodeSigningConfigOutputBuilder;

pub use crate::operation::delete_function_code_signing_config::_delete_function_code_signing_config_input::DeleteFunctionCodeSigningConfigInputBuilder;

impl DeleteFunctionCodeSigningConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_function_code_signing_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteFunctionCodeSigningConfig`.
///
/// <p>Removes the code signing configuration from the function.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteFunctionCodeSigningConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_function_code_signing_config::builders::DeleteFunctionCodeSigningConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
    > for DeleteFunctionCodeSigningConfigFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
            crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteFunctionCodeSigningConfigFluentBuilder {
    /// Creates a new `DeleteFunctionCodeSigningConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteFunctionCodeSigningConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_function_code_signing_config::builders::DeleteFunctionCodeSigningConfigInputBuilder {
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
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigOutput,
        crate::operation::delete_function_code_signing_config::DeleteFunctionCodeSigningConfigError,
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
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn function_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.function_name(input.into());
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn set_function_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_function_name(input);
        self
    }
    /// <p>The name of the Lambda function.</p>
    /// <p class="title"><b>Name formats</b></p>
    /// <ul>
    /// <li>
    /// <p><b>Function name</b> - <code>MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p></li>
    /// <li>
    /// <p><b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p></li>
    /// </ul>
    /// <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    pub fn get_function_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_function_name()
    }
}
