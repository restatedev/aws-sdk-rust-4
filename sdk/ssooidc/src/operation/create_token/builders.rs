// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_token::_create_token_output::CreateTokenOutputBuilder;

pub use crate::operation::create_token::_create_token_input::CreateTokenInputBuilder;

impl CreateTokenInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_token::CreateTokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_token::CreateTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_token();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateToken`.
///
/// <p>Creates and returns access and refresh tokens for clients that are authenticated using client secrets. The access token can be used to fetch short-term credentials for the assigned AWS accounts or to access application APIs using <code>bearer</code> authentication.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateTokenFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_token::builders::CreateTokenInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_token::CreateTokenOutput,
        crate::operation::create_token::CreateTokenError,
    > for CreateTokenFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_token::CreateTokenOutput,
            crate::operation::create_token::CreateTokenError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateTokenFluentBuilder {
    /// Creates a new `CreateToken`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateToken as a reference.
    pub fn as_input(&self) -> &crate::operation::create_token::builders::CreateTokenInputBuilder {
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
        crate::operation::create_token::CreateTokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_token::CreateTokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_token::CreateToken::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_token::CreateToken::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_token::CreateTokenOutput,
        crate::operation::create_token::CreateTokenError,
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
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_id(input.into());
        self
    }
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_id(input);
        self
    }
    /// <p>The unique identifier string for the client or application. This value comes from the result of the <code>RegisterClient</code> API.</p>
    pub fn get_client_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_id()
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn client_secret(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_secret(input.into());
        self
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn set_client_secret(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_secret(input);
        self
    }
    /// <p>A secret string generated for the client. This value should come from the persisted result of the <code>RegisterClient</code> API.</p>
    pub fn get_client_secret(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_secret()
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn grant_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_type(input.into());
        self
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn set_grant_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_type(input);
        self
    }
    /// <p>Supports the following OAuth grant types: Device Code and Refresh Token. Specify either of the following values, depending on the grant type that you want:</p>
    /// <p>* Device Code - <code>urn:ietf:params:oauth:grant-type:device_code</code></p>
    /// <p>* Refresh Token - <code>refresh_token</code></p>
    /// <p>For information about how to obtain the device code, see the <code>StartDeviceAuthorization</code> topic.</p>
    pub fn get_grant_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_type()
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn device_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_code(input.into());
        self
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn set_device_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_code(input);
        self
    }
    /// <p>Used only when calling this API for the Device Code grant type. This short-term code is used to identify this authorization request. This comes from the result of the <code>StartDeviceAuthorization</code> API.</p>
    pub fn get_device_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_code()
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.code(input.into());
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_code(input);
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. The short-term code is used to identify this authorization request. This grant type is currently unsupported for the <code>CreateToken</code> API.</p>
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_code()
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn refresh_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.refresh_token(input.into());
        self
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn set_refresh_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_refresh_token(input);
        self
    }
    /// <p>Used only when calling this API for the Refresh Token grant type. This token is used to refresh short-term tokens, such as the access token, that might expire.</p>
    /// <p>For more information about the features and limitations of the current IAM Identity Center OIDC implementation, see <i>Considerations for Using this Guide</i> in the <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/Welcome.html">IAM Identity Center OIDC API Reference</a>.</p>
    pub fn get_refresh_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_refresh_token()
    }
    /// Appends an item to `scope`.
    ///
    /// To override the contents of this collection use [`set_scope`](Self::set_scope).
    ///
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn scope(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scope(input.into());
        self
    }
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn set_scope(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_scope(input);
        self
    }
    /// <p>The list of scopes for which authorization is requested. The access token that is issued is limited to the scopes that are granted. If this value is not specified, IAM Identity Center authorizes all scopes that are configured for the client during the call to <code>RegisterClient</code>.</p>
    pub fn get_scope(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_scope()
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn redirect_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.redirect_uri(input.into());
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn set_redirect_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_redirect_uri(input);
        self
    }
    /// <p>Used only when calling this API for the Authorization Code grant type. This value specifies the location of the client or application that has registered to receive the authorization code.</p>
    pub fn get_redirect_uri(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_redirect_uri()
    }
}
