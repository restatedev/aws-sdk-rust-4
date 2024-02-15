// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tags::_list_tags_output::ListTagsOutputBuilder;

pub use crate::operation::list_tags::_list_tags_input::ListTagsInputBuilder;

impl ListTagsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tags::ListTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags::ListTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_tags();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTags`.
///
/// <p>Returns a function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>. You can also view tags with <code>GetFunction</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTagsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tags::builders::ListTagsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError>
    for ListTagsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTagsFluentBuilder {
    /// Creates a new `ListTags`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTags as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tags::builders::ListTagsInputBuilder {
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
        crate::operation::list_tags::ListTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags::ListTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_tags::ListTags::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_tags::ListTags::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError, Self>
    {
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
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource(input.into());
        self
    }
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>The function's Amazon Resource Name (ARN). Note: Lambda does not support adding tags to aliases or versions.</p>
    pub fn get_resource(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource()
    }
}
