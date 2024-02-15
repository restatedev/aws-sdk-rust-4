// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lambda couldn't set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubnetIpAddressLimitReachedException {
    #[allow(missing_docs)] // documentation missing in model
    pub r#type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl SubnetIpAddressLimitReachedException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
}
impl SubnetIpAddressLimitReachedException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for SubnetIpAddressLimitReachedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "SubnetIpAddressLimitReachedException [SubnetIPAddressLimitReachedException]")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for SubnetIpAddressLimitReachedException {}
impl ::aws_types::request_id::RequestId for crate::types::error::SubnetIpAddressLimitReachedException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for SubnetIpAddressLimitReachedException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl SubnetIpAddressLimitReachedException {
    /// Creates a new builder-style object to manufacture [`SubnetIpAddressLimitReachedException`](crate::types::error::SubnetIpAddressLimitReachedException).
    pub fn builder() -> crate::types::error::builders::SubnetIpAddressLimitReachedExceptionBuilder {
        crate::types::error::builders::SubnetIpAddressLimitReachedExceptionBuilder::default()
    }
}

/// A builder for [`SubnetIpAddressLimitReachedException`](crate::types::error::SubnetIpAddressLimitReachedException).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SubnetIpAddressLimitReachedExceptionBuilder {
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl SubnetIpAddressLimitReachedExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.r#type
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`SubnetIpAddressLimitReachedException`](crate::types::error::SubnetIpAddressLimitReachedException).
    pub fn build(self) -> crate::types::error::SubnetIpAddressLimitReachedException {
        crate::types::error::SubnetIpAddressLimitReachedException {
            r#type: self.r#type,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
