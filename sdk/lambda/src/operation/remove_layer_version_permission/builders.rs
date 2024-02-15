// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_layer_version_permission::_remove_layer_version_permission_output::RemoveLayerVersionPermissionOutputBuilder;

pub use crate::operation::remove_layer_version_permission::_remove_layer_version_permission_input::RemoveLayerVersionPermissionInputBuilder;

impl RemoveLayerVersionPermissionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_layer_version_permission();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveLayerVersionPermission`.
///
/// <p>Removes a statement from the permissions policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>. For more information, see <code>AddLayerVersionPermission</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveLayerVersionPermissionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_layer_version_permission::builders::RemoveLayerVersionPermissionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionOutput,
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
    > for RemoveLayerVersionPermissionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionOutput,
            crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveLayerVersionPermissionFluentBuilder {
    /// Creates a new `RemoveLayerVersionPermission`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveLayerVersionPermission as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_layer_version_permission::builders::RemoveLayerVersionPermissionInputBuilder {
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
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_layer_version_permission::RemoveLayerVersionPermission::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermission::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionOutput,
        crate::operation::remove_layer_version_permission::RemoveLayerVersionPermissionError,
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
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.layer_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_layer_name(input);
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn get_layer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_layer_name()
    }
    /// <p>The version number.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.inner = self.inner.version_number(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version_number(input);
        self
    }
    /// <p>The version number.</p>
    pub fn get_version_number(&self) -> &::std::option::Option<i64> {
        self.inner.get_version_number()
    }
    /// <p>The identifier that was specified when the statement was added.</p>
    pub fn statement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>The identifier that was specified when the statement was added.</p>
    pub fn set_statement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>The identifier that was specified when the statement was added.</p>
    pub fn get_statement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_id()
    }
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.revision_id(input.into());
        self
    }
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_revision_id(input);
        self
    }
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    pub fn get_revision_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_revision_id()
    }
}
