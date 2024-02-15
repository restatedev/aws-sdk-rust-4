// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`Invoke`](crate::operation::invoke::builders::InvokeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function, version, or alias.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`invocation_type(InvocationType)`](crate::operation::invoke::builders::InvokeFluentBuilder::invocation_type) / [`set_invocation_type(Option<InvocationType>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_invocation_type):<br>required: **false**<br><p>Choose from the following options.</p> <ul>  <li>   <p><code>RequestResponse</code> (default) – Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API response includes the function response and additional data.</p></li>  <li>   <p><code>Event</code> – Invoke the function asynchronously. Send events that fail multiple times to the function's dead-letter queue (if one is configured). The API response only includes a status code.</p></li>  <li>   <p><code>DryRun</code> – Validate parameter values and verify that the user or role has permission to invoke the function.</p></li> </ul><br>
    ///   - [`log_type(LogType)`](crate::operation::invoke::builders::InvokeFluentBuilder::log_type) / [`set_log_type(Option<LogType>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_log_type):<br>required: **false**<br><p>Set to <code>Tail</code> to include the execution log in the response. Applies to synchronously invoked functions only.</p><br>
    ///   - [`client_context(impl Into<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::client_context) / [`set_client_context(Option<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_client_context):<br>required: **false**<br><p>Up to 3,583 bytes of base64-encoded data about the invoking client to pass to the function in the context object.</p><br>
    ///   - [`payload(Blob)`](crate::operation::invoke::builders::InvokeFluentBuilder::payload) / [`set_payload(Option<Blob>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_payload):<br>required: **false**<br><p>The JSON that you want to provide to your Lambda function as input.</p> <p>You can enter the JSON directly. For example, <code>--payload '{ "key": "value" }'</code>. You can also specify a file path. For example, <code>--payload file://payload.json</code>.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::invoke::builders::InvokeFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version or alias to invoke a published version of the function.</p><br>
    /// - On success, responds with [`InvokeOutput`](crate::operation::invoke::InvokeOutput) with field(s):
    ///   - [`status_code(i32)`](crate::operation::invoke::InvokeOutput::status_code): <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    ///   - [`function_error(Option<String>)`](crate::operation::invoke::InvokeOutput::function_error): <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    ///   - [`log_result(Option<String>)`](crate::operation::invoke::InvokeOutput::log_result): <p>The last 4 KB of the execution log, which is base64-encoded.</p>
    ///   - [`payload(Option<Blob>)`](crate::operation::invoke::InvokeOutput::payload): <p>The response from the function, or an error object.</p>
    ///   - [`executed_version(Option<String>)`](crate::operation::invoke::InvokeOutput::executed_version): <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    /// - On failure, responds with [`SdkError<InvokeError>`](crate::operation::invoke::InvokeError)
    pub fn invoke(&self) -> crate::operation::invoke::builders::InvokeFluentBuilder {
        crate::operation::invoke::builders::InvokeFluentBuilder::new(self.handle.clone())
    }
}
