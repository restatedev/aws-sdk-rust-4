// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetFunction`](crate::operation::get_function::builders::GetFunctionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::get_function::builders::GetFunctionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::get_function::builders::GetFunctionFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function, version, or alias.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::get_function::builders::GetFunctionFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::get_function::builders::GetFunctionFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version or alias to get details about a published version of the function.</p><br>
    /// - On success, responds with [`GetFunctionOutput`](crate::operation::get_function::GetFunctionOutput) with field(s):
    ///   - [`configuration(Option<FunctionConfiguration>)`](crate::operation::get_function::GetFunctionOutput::configuration): <p>The configuration of the function or version.</p>
    ///   - [`code(Option<FunctionCodeLocation>)`](crate::operation::get_function::GetFunctionOutput::code): <p>The deployment package of the function or version.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_function::GetFunctionOutput::tags): <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    ///   - [`concurrency(Option<Concurrency>)`](crate::operation::get_function::GetFunctionOutput::concurrency): <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    /// - On failure, responds with [`SdkError<GetFunctionError>`](crate::operation::get_function::GetFunctionError)
    pub fn get_function(&self) -> crate::operation::get_function::builders::GetFunctionFluentBuilder {
        crate::operation::get_function::builders::GetFunctionFluentBuilder::new(self.handle.clone())
    }
}
