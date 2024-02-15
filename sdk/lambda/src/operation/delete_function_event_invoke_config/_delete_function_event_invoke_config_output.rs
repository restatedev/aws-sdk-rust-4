// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFunctionEventInvokeConfigOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteFunctionEventInvokeConfigOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteFunctionEventInvokeConfigOutput {
    /// Creates a new builder-style object to manufacture [`DeleteFunctionEventInvokeConfigOutput`](crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput).
    pub fn builder() -> crate::operation::delete_function_event_invoke_config::builders::DeleteFunctionEventInvokeConfigOutputBuilder {
        crate::operation::delete_function_event_invoke_config::builders::DeleteFunctionEventInvokeConfigOutputBuilder::default()
    }
}

/// A builder for [`DeleteFunctionEventInvokeConfigOutput`](crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteFunctionEventInvokeConfigOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteFunctionEventInvokeConfigOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteFunctionEventInvokeConfigOutput`](crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput).
    pub fn build(self) -> crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput {
        crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput {
            _request_id: self._request_id,
        }
    }
}
