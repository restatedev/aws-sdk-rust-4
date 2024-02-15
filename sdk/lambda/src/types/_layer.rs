// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Layer {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The size of the layer archive in bytes.</p>
    pub code_size: i64,
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub signing_profile_version_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub signing_job_arn: ::std::option::Option<::std::string::String>,
}
impl Layer {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn code_size(&self) -> i64 {
        self.code_size
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn signing_profile_version_arn(&self) -> ::std::option::Option<&str> {
        self.signing_profile_version_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn signing_job_arn(&self) -> ::std::option::Option<&str> {
        self.signing_job_arn.as_deref()
    }
}
impl Layer {
    /// Creates a new builder-style object to manufacture [`Layer`](crate::types::Layer).
    pub fn builder() -> crate::types::builders::LayerBuilder {
        crate::types::builders::LayerBuilder::default()
    }
}

/// A builder for [`Layer`](crate::types::Layer).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LayerBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) code_size: ::std::option::Option<i64>,
    pub(crate) signing_profile_version_arn: ::std::option::Option<::std::string::String>,
    pub(crate) signing_job_arn: ::std::option::Option<::std::string::String>,
}
impl LayerBuilder {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn code_size(mut self, input: i64) -> Self {
        self.code_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn set_code_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.code_size = input;
        self
    }
    /// <p>The size of the layer archive in bytes.</p>
    pub fn get_code_size(&self) -> &::std::option::Option<i64> {
        &self.code_size
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn signing_profile_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signing_profile_version_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn set_signing_profile_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signing_profile_version_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    pub fn get_signing_profile_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.signing_profile_version_arn
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn signing_job_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signing_job_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn set_signing_job_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signing_job_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    pub fn get_signing_job_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.signing_job_arn
    }
    /// Consumes the builder and constructs a [`Layer`](crate::types::Layer).
    pub fn build(self) -> crate::types::Layer {
        crate::types::Layer {
            arn: self.arn,
            code_size: self.code_size.unwrap_or_default(),
            signing_profile_version_arn: self.signing_profile_version_arn,
            signing_job_arn: self.signing_job_arn,
        }
    }
}
