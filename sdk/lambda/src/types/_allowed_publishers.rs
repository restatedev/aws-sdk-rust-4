// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List of signing profiles that can sign a code package.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AllowedPublishers {
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.</p>
    pub signing_profile_version_arns: ::std::vec::Vec<::std::string::String>,
}
impl AllowedPublishers {
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.</p>
    pub fn signing_profile_version_arns(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.signing_profile_version_arns.deref()
    }
}
impl AllowedPublishers {
    /// Creates a new builder-style object to manufacture [`AllowedPublishers`](crate::types::AllowedPublishers).
    pub fn builder() -> crate::types::builders::AllowedPublishersBuilder {
        crate::types::builders::AllowedPublishersBuilder::default()
    }
}

/// A builder for [`AllowedPublishers`](crate::types::AllowedPublishers).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AllowedPublishersBuilder {
    pub(crate) signing_profile_version_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AllowedPublishersBuilder {
    /// Appends an item to `signing_profile_version_arns`.
    ///
    /// To override the contents of this collection use [`set_signing_profile_version_arns`](Self::set_signing_profile_version_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.</p>
    pub fn signing_profile_version_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.signing_profile_version_arns.unwrap_or_default();
        v.push(input.into());
        self.signing_profile_version_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.</p>
    pub fn set_signing_profile_version_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.signing_profile_version_arns = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package.</p>
    pub fn get_signing_profile_version_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.signing_profile_version_arns
    }
    /// Consumes the builder and constructs a [`AllowedPublishers`](crate::types::AllowedPublishers).
    /// This method will fail if any of the following fields are not set:
    /// - [`signing_profile_version_arns`](crate::types::builders::AllowedPublishersBuilder::signing_profile_version_arns)
    pub fn build(self) -> ::std::result::Result<crate::types::AllowedPublishers, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::AllowedPublishers {
            signing_profile_version_arns: self.signing_profile_version_arns.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "signing_profile_version_arns",
                    "signing_profile_version_arns was not specified but it is required when building AllowedPublishers",
                )
            })?,
        })
    }
}
