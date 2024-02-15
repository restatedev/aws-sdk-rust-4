// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Amazon Web Services credentials for API authentication.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Credentials {
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub access_key_id: ::std::string::String,
    /// <p>The secret access key that can be used to sign requests.</p>
    pub secret_access_key: ::std::string::String,
    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    pub session_token: ::std::string::String,
    /// <p>The date on which the current credentials expire.</p>
    pub expiration: ::aws_smithy_types::DateTime,
}
impl Credentials {
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub fn access_key_id(&self) -> &str {
        use std::ops::Deref;
        self.access_key_id.deref()
    }
    /// <p>The secret access key that can be used to sign requests.</p>
    pub fn secret_access_key(&self) -> &str {
        use std::ops::Deref;
        self.secret_access_key.deref()
    }
    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    pub fn session_token(&self) -> &str {
        use std::ops::Deref;
        self.session_token.deref()
    }
    /// <p>The date on which the current credentials expire.</p>
    pub fn expiration(&self) -> &::aws_smithy_types::DateTime {
        &self.expiration
    }
}
impl ::std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Credentials");
        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
        formatter.field("expiration", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl Credentials {
    /// Creates a new builder-style object to manufacture [`Credentials`](crate::types::Credentials).
    pub fn builder() -> crate::types::builders::CredentialsBuilder {
        crate::types::builders::CredentialsBuilder::default()
    }
}

/// A builder for [`Credentials`](crate::types::Credentials).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CredentialsBuilder {
    pub(crate) access_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) secret_access_key: ::std::option::Option<::std::string::String>,
    pub(crate) session_token: ::std::option::Option<::std::string::String>,
    pub(crate) expiration: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl CredentialsBuilder {
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    /// This field is required.
    pub fn access_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.access_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub fn set_access_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.access_key_id = input;
        self
    }
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub fn get_access_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.access_key_id
    }
    /// <p>The secret access key that can be used to sign requests.</p>
    /// This field is required.
    pub fn secret_access_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_access_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The secret access key that can be used to sign requests.</p>
    pub fn set_secret_access_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_access_key = input;
        self
    }
    /// <p>The secret access key that can be used to sign requests.</p>
    pub fn get_secret_access_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.secret_access_key
    }
    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    /// This field is required.
    pub fn session_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.session_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    pub fn set_session_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.session_token = input;
        self
    }
    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    pub fn get_session_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.session_token
    }
    /// <p>The date on which the current credentials expire.</p>
    /// This field is required.
    pub fn expiration(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.expiration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date on which the current credentials expire.</p>
    pub fn set_expiration(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.expiration = input;
        self
    }
    /// <p>The date on which the current credentials expire.</p>
    pub fn get_expiration(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.expiration
    }
    /// Consumes the builder and constructs a [`Credentials`](crate::types::Credentials).
    /// This method will fail if any of the following fields are not set:
    /// - [`access_key_id`](crate::types::builders::CredentialsBuilder::access_key_id)
    /// - [`secret_access_key`](crate::types::builders::CredentialsBuilder::secret_access_key)
    /// - [`session_token`](crate::types::builders::CredentialsBuilder::session_token)
    /// - [`expiration`](crate::types::builders::CredentialsBuilder::expiration)
    pub fn build(self) -> ::std::result::Result<crate::types::Credentials, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Credentials {
            access_key_id: self.access_key_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "access_key_id",
                    "access_key_id was not specified but it is required when building Credentials",
                )
            })?,
            secret_access_key: self.secret_access_key.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "secret_access_key",
                    "secret_access_key was not specified but it is required when building Credentials",
                )
            })?,
            session_token: self.session_token.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "session_token",
                    "session_token was not specified but it is required when building Credentials",
                )
            })?,
            expiration: self.expiration.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "expiration",
                    "expiration was not specified but it is required when building Credentials",
                )
            })?,
        })
    }
}
impl ::std::fmt::Debug for CredentialsBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CredentialsBuilder");
        formatter.field("access_key_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_access_key", &"*** Sensitive Data Redacted ***");
        formatter.field("session_token", &"*** Sensitive Data Redacted ***");
        formatter.field("expiration", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
