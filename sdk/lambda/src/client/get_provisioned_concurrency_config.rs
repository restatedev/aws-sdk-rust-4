// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetProvisionedConcurrencyConfig`](crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code>.</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder::set_qualifier):<br>required: **true**<br><p>The version number or alias name.</p><br>
    /// - On success, responds with [`GetProvisionedConcurrencyConfigOutput`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput) with field(s):
    ///   - [`requested_provisioned_concurrent_executions(Option<i32>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::requested_provisioned_concurrent_executions): <p>The amount of provisioned concurrency requested.</p>
    ///   - [`available_provisioned_concurrent_executions(Option<i32>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::available_provisioned_concurrent_executions): <p>The amount of provisioned concurrency available.</p>
    ///   - [`allocated_provisioned_concurrent_executions(Option<i32>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::allocated_provisioned_concurrent_executions): <p>The amount of provisioned concurrency allocated. When a weighted alias is used during linear and canary deployments, this value fluctuates depending on the amount of concurrency that is provisioned for the function versions.</p>
    ///   - [`status(Option<ProvisionedConcurrencyStatusEnum>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::status): <p>The status of the allocation process.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::status_reason): <p>For failed allocations, the reason that provisioned concurrency could not be allocated.</p>
    ///   - [`last_modified(Option<String>)`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigOutput::last_modified): <p>The date and time that a user last updated the configuration, in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601 format</a>.</p>
    /// - On failure, responds with [`SdkError<GetProvisionedConcurrencyConfigError>`](crate::operation::get_provisioned_concurrency_config::GetProvisionedConcurrencyConfigError)
    pub fn get_provisioned_concurrency_config(
        &self,
    ) -> crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder {
        crate::operation::get_provisioned_concurrency_config::builders::GetProvisionedConcurrencyConfigFluentBuilder::new(self.handle.clone())
    }
}
