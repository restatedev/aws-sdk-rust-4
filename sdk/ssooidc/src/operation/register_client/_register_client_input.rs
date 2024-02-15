// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterClientInput {
    /// <p>The friendly name of the client.</p>
    pub client_name: ::std::option::Option<::std::string::String>,
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub client_type: ::std::option::Option<::std::string::String>,
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RegisterClientInput {
    /// <p>The friendly name of the client.</p>
    pub fn client_name(&self) -> ::std::option::Option<&str> {
        self.client_name.as_deref()
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn client_type(&self) -> ::std::option::Option<&str> {
        self.client_type.as_deref()
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.scopes.is_none()`.
    pub fn scopes(&self) -> &[::std::string::String] {
        self.scopes.as_deref().unwrap_or_default()
    }
}
impl RegisterClientInput {
    /// Creates a new builder-style object to manufacture [`RegisterClientInput`](crate::operation::register_client::RegisterClientInput).
    pub fn builder() -> crate::operation::register_client::builders::RegisterClientInputBuilder {
        crate::operation::register_client::builders::RegisterClientInputBuilder::default()
    }
}

/// A builder for [`RegisterClientInput`](crate::operation::register_client::RegisterClientInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RegisterClientInputBuilder {
    pub(crate) client_name: ::std::option::Option<::std::string::String>,
    pub(crate) client_type: ::std::option::Option<::std::string::String>,
    pub(crate) scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RegisterClientInputBuilder {
    /// <p>The friendly name of the client.</p>
    /// This field is required.
    pub fn client_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The friendly name of the client.</p>
    pub fn set_client_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_name = input;
        self
    }
    /// <p>The friendly name of the client.</p>
    pub fn get_client_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_name
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    /// This field is required.
    pub fn client_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn set_client_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_type = input;
        self
    }
    /// <p>The type of client. The service supports only <code>public</code> as a client type. Anything other than public will be rejected by the service.</p>
    pub fn get_client_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_type
    }
    /// Appends an item to `scopes`.
    ///
    /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
    ///
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn scopes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.scopes.unwrap_or_default();
        v.push(input.into());
        self.scopes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn set_scopes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.scopes = input;
        self
    }
    /// <p>The list of scopes that are defined by the client. Upon authorization, this list is used to restrict permissions when granting an access token.</p>
    pub fn get_scopes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.scopes
    }
    /// Consumes the builder and constructs a [`RegisterClientInput`](crate::operation::register_client::RegisterClientInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::register_client::RegisterClientInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::register_client::RegisterClientInput {
            client_name: self.client_name,
            client_type: self.client_type,
            scopes: self.scopes,
        })
    }
}
