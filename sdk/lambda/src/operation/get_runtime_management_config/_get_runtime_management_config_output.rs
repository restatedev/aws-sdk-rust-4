// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRuntimeManagementConfigOutput {
    /// <p>The current runtime update mode of the function.</p>
    pub update_runtime_on: ::std::option::Option<crate::types::UpdateRuntimeOn>,
    /// <p>The ARN of the runtime the function is configured to use. If the runtime update mode is <b>Manual</b>, the ARN is returned, otherwise <code>null</code> is returned.</p>
    pub runtime_version_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub function_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetRuntimeManagementConfigOutput {
    /// <p>The current runtime update mode of the function.</p>
    pub fn update_runtime_on(&self) -> ::std::option::Option<&crate::types::UpdateRuntimeOn> {
        self.update_runtime_on.as_ref()
    }
    /// <p>The ARN of the runtime the function is configured to use. If the runtime update mode is <b>Manual</b>, the ARN is returned, otherwise <code>null</code> is returned.</p>
    pub fn runtime_version_arn(&self) -> ::std::option::Option<&str> {
        self.runtime_version_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn function_arn(&self) -> ::std::option::Option<&str> {
        self.function_arn.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetRuntimeManagementConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetRuntimeManagementConfigOutput {
    /// Creates a new builder-style object to manufacture [`GetRuntimeManagementConfigOutput`](crate::operation::get_runtime_management_config::GetRuntimeManagementConfigOutput).
    pub fn builder() -> crate::operation::get_runtime_management_config::builders::GetRuntimeManagementConfigOutputBuilder {
        crate::operation::get_runtime_management_config::builders::GetRuntimeManagementConfigOutputBuilder::default()
    }
}

/// A builder for [`GetRuntimeManagementConfigOutput`](crate::operation::get_runtime_management_config::GetRuntimeManagementConfigOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetRuntimeManagementConfigOutputBuilder {
    pub(crate) update_runtime_on: ::std::option::Option<crate::types::UpdateRuntimeOn>,
    pub(crate) runtime_version_arn: ::std::option::Option<::std::string::String>,
    pub(crate) function_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetRuntimeManagementConfigOutputBuilder {
    /// <p>The current runtime update mode of the function.</p>
    pub fn update_runtime_on(mut self, input: crate::types::UpdateRuntimeOn) -> Self {
        self.update_runtime_on = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current runtime update mode of the function.</p>
    pub fn set_update_runtime_on(mut self, input: ::std::option::Option<crate::types::UpdateRuntimeOn>) -> Self {
        self.update_runtime_on = input;
        self
    }
    /// <p>The current runtime update mode of the function.</p>
    pub fn get_update_runtime_on(&self) -> &::std::option::Option<crate::types::UpdateRuntimeOn> {
        &self.update_runtime_on
    }
    /// <p>The ARN of the runtime the function is configured to use. If the runtime update mode is <b>Manual</b>, the ARN is returned, otherwise <code>null</code> is returned.</p>
    pub fn runtime_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.runtime_version_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the runtime the function is configured to use. If the runtime update mode is <b>Manual</b>, the ARN is returned, otherwise <code>null</code> is returned.</p>
    pub fn set_runtime_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.runtime_version_arn = input;
        self
    }
    /// <p>The ARN of the runtime the function is configured to use. If the runtime update mode is <b>Manual</b>, the ARN is returned, otherwise <code>null</code> is returned.</p>
    pub fn get_runtime_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.runtime_version_arn
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn set_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of your function.</p>
    pub fn get_function_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.function_arn
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetRuntimeManagementConfigOutput`](crate::operation::get_runtime_management_config::GetRuntimeManagementConfigOutput).
    pub fn build(self) -> crate::operation::get_runtime_management_config::GetRuntimeManagementConfigOutput {
        crate::operation::get_runtime_management_config::GetRuntimeManagementConfigOutput {
            update_runtime_on: self.update_runtime_on,
            runtime_version_arn: self.runtime_version_arn,
            function_arn: self.function_arn,
            _request_id: self._request_id,
        }
    }
}
