// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLayerVersionByArn`](crate::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnFluentBuilder::set_arn):<br>required: **true**<br><p>The ARN of the layer version.</p><br>
    /// - On success, responds with [`GetLayerVersionByArnOutput`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput) with field(s):
    ///   - [`content(Option<LayerVersionContentOutput>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::content): <p>Details about the layer version.</p>
    ///   - [`layer_arn(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::layer_arn): <p>The ARN of the layer.</p>
    ///   - [`layer_version_arn(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::layer_version_arn): <p>The ARN of the layer version.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::description): <p>The description of the version.</p>
    ///   - [`created_date(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::created_date): <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    ///   - [`version(i64)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::version): <p>The version number.</p>
    ///   - [`compatible_runtimes(Option<Vec::<Runtime>>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::compatible_runtimes): <p>The layer's compatible runtimes.</p> <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    ///   - [`license_info(Option<String>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::license_info): <p>The layer's software license.</p>
    ///   - [`compatible_architectures(Option<Vec::<Architecture>>)`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnOutput::compatible_architectures): <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    /// - On failure, responds with [`SdkError<GetLayerVersionByArnError>`](crate::operation::get_layer_version_by_arn::GetLayerVersionByArnError)
    pub fn get_layer_version_by_arn(&self) -> crate::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnFluentBuilder {
        crate::operation::get_layer_version_by_arn::builders::GetLayerVersionByArnFluentBuilder::new(self.handle.clone())
    }
}
