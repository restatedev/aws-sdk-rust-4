// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The function's X-Ray tracing configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TracingConfigResponse {
    /// <p>The tracing mode.</p>
    pub mode: ::std::option::Option<crate::types::TracingMode>,
}
impl TracingConfigResponse {
    /// <p>The tracing mode.</p>
    pub fn mode(&self) -> ::std::option::Option<&crate::types::TracingMode> {
        self.mode.as_ref()
    }
}
impl TracingConfigResponse {
    /// Creates a new builder-style object to manufacture [`TracingConfigResponse`](crate::types::TracingConfigResponse).
    pub fn builder() -> crate::types::builders::TracingConfigResponseBuilder {
        crate::types::builders::TracingConfigResponseBuilder::default()
    }
}

/// A builder for [`TracingConfigResponse`](crate::types::TracingConfigResponse).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TracingConfigResponseBuilder {
    pub(crate) mode: ::std::option::Option<crate::types::TracingMode>,
}
impl TracingConfigResponseBuilder {
    /// <p>The tracing mode.</p>
    pub fn mode(mut self, input: crate::types::TracingMode) -> Self {
        self.mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tracing mode.</p>
    pub fn set_mode(mut self, input: ::std::option::Option<crate::types::TracingMode>) -> Self {
        self.mode = input;
        self
    }
    /// <p>The tracing mode.</p>
    pub fn get_mode(&self) -> &::std::option::Option<crate::types::TracingMode> {
        &self.mode
    }
    /// Consumes the builder and constructs a [`TracingConfigResponse`](crate::types::TracingConfigResponse).
    pub fn build(self) -> crate::types::TracingConfigResponse {
        crate::types::TracingConfigResponse { mode: self.mode }
    }
}
