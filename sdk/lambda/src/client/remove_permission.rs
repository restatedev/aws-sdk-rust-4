// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemovePermission`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function, version, or alias.</p> <p class="title"><b>Name formats</b></p> <ul>  <li>   <p><b>Function name</b> – <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p></li>  <li>   <p><b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p></li>  <li>   <p><b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p></li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`statement_id(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::statement_id) / [`set_statement_id(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_statement_id):<br>required: **true**<br><p>Statement ID of the permission to remove.</p><br>
    ///   - [`qualifier(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::qualifier) / [`set_qualifier(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_qualifier):<br>required: **false**<br><p>Specify a version or alias to remove permissions from a published version of the function.</p><br>
    ///   - [`revision_id(impl Into<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::revision_id) / [`set_revision_id(Option<String>)`](crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::set_revision_id):<br>required: **false**<br><p>Update the policy only if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p><br>
    /// - On success, responds with [`RemovePermissionOutput`](crate::operation::remove_permission::RemovePermissionOutput)
    /// - On failure, responds with [`SdkError<RemovePermissionError>`](crate::operation::remove_permission::RemovePermissionError)
    pub fn remove_permission(&self) -> crate::operation::remove_permission::builders::RemovePermissionFluentBuilder {
        crate::operation::remove_permission::builders::RemovePermissionFluentBuilder::new(self.handle.clone())
    }
}
