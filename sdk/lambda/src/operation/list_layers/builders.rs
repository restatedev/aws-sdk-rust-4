// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_layers::_list_layers_output::ListLayersOutputBuilder;

pub use crate::operation::list_layers::_list_layers_input::ListLayersInputBuilder;

impl ListLayersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_layers::ListLayersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_layers::ListLayersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_layers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListLayers`.
///
/// <p>Lists <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-layers.html">Lambda layers</a> and shows information about the latest version of each. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only layers that indicate that they're compatible with that runtime. Specify a compatible architecture to include only layers that are compatible with that <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListLayersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_layers::builders::ListLayersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_layers::ListLayersOutput,
        crate::operation::list_layers::ListLayersError,
    > for ListLayersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_layers::ListLayersOutput,
            crate::operation::list_layers::ListLayersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListLayersFluentBuilder {
    /// Creates a new `ListLayers`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListLayers as a reference.
    pub fn as_input(&self) -> &crate::operation::list_layers::builders::ListLayersInputBuilder {
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
        crate::operation::list_layers::ListLayersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_layers::ListLayersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_layers::ListLayers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_layers::ListLayers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_layers::ListLayersOutput,
        crate::operation::list_layers::ListLayersError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_layers::paginator::ListLayersPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_layers::paginator::ListLayersPaginator {
        crate::operation::list_layers::paginator::ListLayersPaginator::new(self.handle, self.inner)
    }
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn compatible_runtime(mut self, input: crate::types::Runtime) -> Self {
        self.inner = self.inner.compatible_runtime(input);
        self
    }
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn set_compatible_runtime(mut self, input: ::std::option::Option<crate::types::Runtime>) -> Self {
        self.inner = self.inner.set_compatible_runtime(input);
        self
    }
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn get_compatible_runtime(&self) -> &::std::option::Option<crate::types::Runtime> {
        self.inner.get_compatible_runtime()
    }
    /// <p>A pagination token returned by a previous call.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>A pagination token returned by a previous call.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>A pagination token returned by a previous call.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of layers to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of layers to return.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of layers to return.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
    /// <p>The compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
    pub fn compatible_architecture(mut self, input: crate::types::Architecture) -> Self {
        self.inner = self.inner.compatible_architecture(input);
        self
    }
    /// <p>The compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
    pub fn set_compatible_architecture(mut self, input: ::std::option::Option<crate::types::Architecture>) -> Self {
        self.inner = self.inner.set_compatible_architecture(input);
        self
    }
    /// <p>The compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architecture</a>.</p>
    pub fn get_compatible_architecture(&self) -> &::std::option::Option<crate::types::Architecture> {
        self.inner.get_compatible_architecture()
    }
}
