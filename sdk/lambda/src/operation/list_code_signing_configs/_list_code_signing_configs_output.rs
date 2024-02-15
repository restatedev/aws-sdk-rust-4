// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCodeSigningConfigsOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>The code signing configurations</p>
    pub code_signing_configs: ::std::option::Option<::std::vec::Vec<crate::types::CodeSigningConfig>>,
    _request_id: Option<String>,
}
impl ListCodeSigningConfigsOutput {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>The code signing configurations</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.code_signing_configs.is_none()`.
    pub fn code_signing_configs(&self) -> &[crate::types::CodeSigningConfig] {
        self.code_signing_configs.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ListCodeSigningConfigsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCodeSigningConfigsOutput {
    /// Creates a new builder-style object to manufacture [`ListCodeSigningConfigsOutput`](crate::operation::list_code_signing_configs::ListCodeSigningConfigsOutput).
    pub fn builder() -> crate::operation::list_code_signing_configs::builders::ListCodeSigningConfigsOutputBuilder {
        crate::operation::list_code_signing_configs::builders::ListCodeSigningConfigsOutputBuilder::default()
    }
}

/// A builder for [`ListCodeSigningConfigsOutput`](crate::operation::list_code_signing_configs::ListCodeSigningConfigsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListCodeSigningConfigsOutputBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) code_signing_configs: ::std::option::Option<::std::vec::Vec<crate::types::CodeSigningConfig>>,
    _request_id: Option<String>,
}
impl ListCodeSigningConfigsOutputBuilder {
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_marker
    }
    /// Appends an item to `code_signing_configs`.
    ///
    /// To override the contents of this collection use [`set_code_signing_configs`](Self::set_code_signing_configs).
    ///
    /// <p>The code signing configurations</p>
    pub fn code_signing_configs(mut self, input: crate::types::CodeSigningConfig) -> Self {
        let mut v = self.code_signing_configs.unwrap_or_default();
        v.push(input);
        self.code_signing_configs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The code signing configurations</p>
    pub fn set_code_signing_configs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CodeSigningConfig>>) -> Self {
        self.code_signing_configs = input;
        self
    }
    /// <p>The code signing configurations</p>
    pub fn get_code_signing_configs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CodeSigningConfig>> {
        &self.code_signing_configs
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListCodeSigningConfigsOutput`](crate::operation::list_code_signing_configs::ListCodeSigningConfigsOutput).
    pub fn build(self) -> crate::operation::list_code_signing_configs::ListCodeSigningConfigsOutput {
        crate::operation::list_code_signing_configs::ListCodeSigningConfigsOutput {
            next_marker: self.next_marker,
            code_signing_configs: self.code_signing_configs,
            _request_id: self._request_id,
        }
    }
}
