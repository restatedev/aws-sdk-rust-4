// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The function's Amazon CloudWatch Logs configuration settings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LoggingConfig {
    /// <p>The format in which Lambda sends your function's application and system logs to CloudWatch. Select between plain text and structured JSON.</p>
    pub log_format: ::std::option::Option<crate::types::LogFormat>,
    /// <p>Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the selected level and lower.</p>
    pub application_log_level: ::std::option::Option<crate::types::ApplicationLogLevel>,
    /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level and lower.</p>
    pub system_log_level: ::std::option::Option<crate::types::SystemLogLevel>,
    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/<function name></function></code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
    pub log_group: ::std::option::Option<::std::string::String>,
}
impl LoggingConfig {
    /// <p>The format in which Lambda sends your function's application and system logs to CloudWatch. Select between plain text and structured JSON.</p>
    pub fn log_format(&self) -> ::std::option::Option<&crate::types::LogFormat> {
        self.log_format.as_ref()
    }
    /// <p>Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the selected level and lower.</p>
    pub fn application_log_level(&self) -> ::std::option::Option<&crate::types::ApplicationLogLevel> {
        self.application_log_level.as_ref()
    }
    /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level and lower.</p>
    pub fn system_log_level(&self) -> ::std::option::Option<&crate::types::SystemLogLevel> {
        self.system_log_level.as_ref()
    }
    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/<function name></function></code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
    pub fn log_group(&self) -> ::std::option::Option<&str> {
        self.log_group.as_deref()
    }
}
impl LoggingConfig {
    /// Creates a new builder-style object to manufacture [`LoggingConfig`](crate::types::LoggingConfig).
    pub fn builder() -> crate::types::builders::LoggingConfigBuilder {
        crate::types::builders::LoggingConfigBuilder::default()
    }
}

/// A builder for [`LoggingConfig`](crate::types::LoggingConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LoggingConfigBuilder {
    pub(crate) log_format: ::std::option::Option<crate::types::LogFormat>,
    pub(crate) application_log_level: ::std::option::Option<crate::types::ApplicationLogLevel>,
    pub(crate) system_log_level: ::std::option::Option<crate::types::SystemLogLevel>,
    pub(crate) log_group: ::std::option::Option<::std::string::String>,
}
impl LoggingConfigBuilder {
    /// <p>The format in which Lambda sends your function's application and system logs to CloudWatch. Select between plain text and structured JSON.</p>
    pub fn log_format(mut self, input: crate::types::LogFormat) -> Self {
        self.log_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>The format in which Lambda sends your function's application and system logs to CloudWatch. Select between plain text and structured JSON.</p>
    pub fn set_log_format(mut self, input: ::std::option::Option<crate::types::LogFormat>) -> Self {
        self.log_format = input;
        self
    }
    /// <p>The format in which Lambda sends your function's application and system logs to CloudWatch. Select between plain text and structured JSON.</p>
    pub fn get_log_format(&self) -> &::std::option::Option<crate::types::LogFormat> {
        &self.log_format
    }
    /// <p>Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the selected level and lower.</p>
    pub fn application_log_level(mut self, input: crate::types::ApplicationLogLevel) -> Self {
        self.application_log_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the selected level and lower.</p>
    pub fn set_application_log_level(mut self, input: ::std::option::Option<crate::types::ApplicationLogLevel>) -> Self {
        self.application_log_level = input;
        self
    }
    /// <p>Set this property to filter the application logs for your function that Lambda sends to CloudWatch. Lambda only sends application logs at the selected level and lower.</p>
    pub fn get_application_log_level(&self) -> &::std::option::Option<crate::types::ApplicationLogLevel> {
        &self.application_log_level
    }
    /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level and lower.</p>
    pub fn system_log_level(mut self, input: crate::types::SystemLogLevel) -> Self {
        self.system_log_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level and lower.</p>
    pub fn set_system_log_level(mut self, input: ::std::option::Option<crate::types::SystemLogLevel>) -> Self {
        self.system_log_level = input;
        self
    }
    /// <p>Set this property to filter the system logs for your function that Lambda sends to CloudWatch. Lambda only sends system logs at the selected level and lower.</p>
    pub fn get_system_log_level(&self) -> &::std::option::Option<crate::types::SystemLogLevel> {
        &self.system_log_level
    }
    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/<function name></function></code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
    pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/<function name></function></code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
    pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group = input;
        self
    }
    /// <p>The name of the Amazon CloudWatch log group the function sends logs to. By default, Lambda functions send logs to a default log group named <code>/aws/lambda/<function name></function></code>. To use a different log group, enter an existing log group or enter a new log group name.</p>
    pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group
    }
    /// Consumes the builder and constructs a [`LoggingConfig`](crate::types::LoggingConfig).
    pub fn build(self) -> crate::types::LoggingConfig {
        crate::types::LoggingConfig {
            log_format: self.log_format,
            application_log_level: self.application_log_level,
            system_log_level: self.system_log_level,
            log_group: self.log_group,
        }
    }
}
