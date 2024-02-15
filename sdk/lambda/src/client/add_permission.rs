// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddPermission`](crate::operation::add_permission::builders::AddPermissionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function, version, or alias.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`statement_id(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::statement_id) / [`set_statement_id(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_statement_id):<br>required: **true**<br><p>A statement identifier that differentiates the statement from others in the same policy.</p><br>
    ///   - [`action(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::action) / [`set_action(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_action):<br>required: **true**<br><p>The action that the principal can use on the function. For example, <code>lambda:InvokeFunction</code> or <code>lambda:GetFunction</code>.</p><br>
    ///   - [`principal(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::principal) / [`set_principal(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_principal):<br>required: **true**<br><p>The Amazon Web Service or Amazon Web Services account that invokes the function. If you specify a service, use <code>SourceArn</code> or <code>SourceAccount</code> to limit who can invoke the function through that service.</p><br>
    ///   - [`source_arn(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::source_arn) / [`set_source_arn(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_source_arn):<br>required: **false**<br><p>For Amazon Web Services, the ARN of the Amazon Web Services resource that invokes the function. For example, an Amazon S3 bucket or Amazon SNS topic.</p> <p>Note that Lambda configures the comparison using the <code>StringLike</code> operator.</p><br>
    ///   - [`source_account(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::source_account) / [`set_source_account(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_source_account):<br>required: **false**<br><p>For Amazon Web Service, the ID of the Amazon Web Services account that owns the resource. Use this together with <code>SourceArn</code> to ensure that the specified account owns the resource. It is possible for an Amazon S3 bucket to be deleted by its owner and recreated by another account.</p><br>
    ///   - [`event_source_token(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::event_source_token) / [`set_event_source_token(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_event_source_token):<br>required: **false**<br><p>For Alexa Smart Home functions, a token that the invoker must supply.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version or alias to add permissions to a published version of the function.</p><br>
    ///   - [`revision_id(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_revision_id):<br>required: **false**<br><p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p><br>
    ///   - [`principal_org_id(impl Into<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::principal_org_id) / [`set_principal_org_id(Option<String>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_principal_org_id):<br>required: **false**<br><p>The identifier for your organization in Organizations. Use this to grant permissions to all the Amazon Web Services accounts under this organization.</p><br>
    ///   - [`function_url_auth_type(FunctionUrlAuthType)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::function_url_auth_type) / [`set_function_url_auth_type(Option<FunctionUrlAuthType>)`](crate::operation::add_permission::builders::AddPermissionFluentBuilder::set_function_url_auth_type):<br>required: **false**<br><p>The type of authentication that your function URL uses. Set to <code>AWS_IAM</code> if you want to restrict access to authenticated users only. Set to <code>NONE</code> if you want to bypass IAM authentication to create a public endpoint. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/urls-auth.html">Security and auth model for Lambda function URLs</a>.</p><br>
    /// - On success, responds with [`AddPermissionOutput`](crate::operation::add_permission::AddPermissionOutput) with field(s):
    ///   - [`statement(Option<String>)`](crate::operation::add_permission::AddPermissionOutput::statement): <p>The permission statement that's added to the function policy.</p>
    /// - On failure, responds with [`SdkError<AddPermissionError>`](crate::operation::add_permission::AddPermissionError)
    pub fn add_permission(&self) -> crate::operation::add_permission::builders::AddPermissionFluentBuilder {
        crate::operation::add_permission::builders::AddPermissionFluentBuilder::new(self.handle.clone())
    }
}
