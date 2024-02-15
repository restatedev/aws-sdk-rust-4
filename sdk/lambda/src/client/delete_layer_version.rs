// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLayerVersion`](crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`layer_name(impl Into<String>)`](crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder::layer_name) / [`set_layer_name(Option<String>)`](crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder::set_layer_name):<br>required: **true**<br><p>The name or Amazon Resource Name (ARN) of the layer.</p><br>
    ///   - [`version_number(i64)`](crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder::version_number) / [`set_version_number(Option<i64>)`](crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder::set_version_number):<br>required: **true**<br><p>The version number.</p><br>
    /// - On success, responds with [`DeleteLayerVersionOutput`](crate::operation::delete_layer_version::DeleteLayerVersionOutput)
    /// - On failure, responds with [`SdkError<DeleteLayerVersionError>`](crate::operation::delete_layer_version::DeleteLayerVersionError)
    pub fn delete_layer_version(&self) -> crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder {
        crate::operation::delete_layer_version::builders::DeleteLayerVersionFluentBuilder::new(self.handle.clone())
    }
}
