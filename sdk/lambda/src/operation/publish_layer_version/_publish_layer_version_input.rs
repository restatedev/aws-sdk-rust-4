// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PublishLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub layer_name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the version.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The function layer archive.</p>
    pub content: ::std::option::Option<crate::types::LayerVersionContentInput>,
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>
    /// <li>
    /// <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>
    /// <li>
    /// <p>The full text of the license.</p></li>
    /// </ul>
    pub license_info: ::std::option::Option<::std::string::String>,
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
}
impl PublishLayerVersionInput {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(&self) -> ::std::option::Option<&str> {
        self.layer_name.as_deref()
    }
    /// <p>The description of the version.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The function layer archive.</p>
    pub fn content(&self) -> ::std::option::Option<&crate::types::LayerVersionContentInput> {
        self.content.as_ref()
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.compatible_runtimes.is_none()`.
    pub fn compatible_runtimes(&self) -> &[crate::types::Runtime] {
        self.compatible_runtimes.as_deref().unwrap_or_default()
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>
    /// <li>
    /// <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>
    /// <li>
    /// <p>The full text of the license.</p></li>
    /// </ul>
    pub fn license_info(&self) -> ::std::option::Option<&str> {
        self.license_info.as_deref()
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.compatible_architectures.is_none()`.
    pub fn compatible_architectures(&self) -> &[crate::types::Architecture] {
        self.compatible_architectures.as_deref().unwrap_or_default()
    }
}
impl PublishLayerVersionInput {
    /// Creates a new builder-style object to manufacture [`PublishLayerVersionInput`](crate::operation::publish_layer_version::PublishLayerVersionInput).
    pub fn builder() -> crate::operation::publish_layer_version::builders::PublishLayerVersionInputBuilder {
        crate::operation::publish_layer_version::builders::PublishLayerVersionInputBuilder::default()
    }
}

/// A builder for [`PublishLayerVersionInput`](crate::operation::publish_layer_version::PublishLayerVersionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PublishLayerVersionInputBuilder {
    pub(crate) layer_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) content: ::std::option::Option<crate::types::LayerVersionContentInput>,
    pub(crate) compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
    pub(crate) license_info: ::std::option::Option<::std::string::String>,
    pub(crate) compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
}
impl PublishLayerVersionInputBuilder {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    /// This field is required.
    pub fn layer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.layer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.layer_name = input;
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn get_layer_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.layer_name
    }
    /// <p>The description of the version.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the version.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the version.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The function layer archive.</p>
    /// This field is required.
    pub fn content(mut self, input: crate::types::LayerVersionContentInput) -> Self {
        self.content = ::std::option::Option::Some(input);
        self
    }
    /// <p>The function layer archive.</p>
    pub fn set_content(mut self, input: ::std::option::Option<crate::types::LayerVersionContentInput>) -> Self {
        self.content = input;
        self
    }
    /// <p>The function layer archive.</p>
    pub fn get_content(&self) -> &::std::option::Option<crate::types::LayerVersionContentInput> {
        &self.content
    }
    /// Appends an item to `compatible_runtimes`.
    ///
    /// To override the contents of this collection use [`set_compatible_runtimes`](Self::set_compatible_runtimes).
    ///
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn compatible_runtimes(mut self, input: crate::types::Runtime) -> Self {
        let mut v = self.compatible_runtimes.unwrap_or_default();
        v.push(input);
        self.compatible_runtimes = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn set_compatible_runtimes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>) -> Self {
        self.compatible_runtimes = input;
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn get_compatible_runtimes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Runtime>> {
        &self.compatible_runtimes
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>
    /// <li>
    /// <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>
    /// <li>
    /// <p>The full text of the license.</p></li>
    /// </ul>
    pub fn license_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.license_info = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>
    /// <li>
    /// <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>
    /// <li>
    /// <p>The full text of the license.</p></li>
    /// </ul>
    pub fn set_license_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.license_info = input;
        self
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li>
    /// <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p></li>
    /// <li>
    /// <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p></li>
    /// <li>
    /// <p>The full text of the license.</p></li>
    /// </ul>
    pub fn get_license_info(&self) -> &::std::option::Option<::std::string::String> {
        &self.license_info
    }
    /// Appends an item to `compatible_architectures`.
    ///
    /// To override the contents of this collection use [`set_compatible_architectures`](Self::set_compatible_architectures).
    ///
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub fn compatible_architectures(mut self, input: crate::types::Architecture) -> Self {
        let mut v = self.compatible_architectures.unwrap_or_default();
        v.push(input);
        self.compatible_architectures = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub fn set_compatible_architectures(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>) -> Self {
        self.compatible_architectures = input;
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub fn get_compatible_architectures(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Architecture>> {
        &self.compatible_architectures
    }
    /// Consumes the builder and constructs a [`PublishLayerVersionInput`](crate::operation::publish_layer_version::PublishLayerVersionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::publish_layer_version::PublishLayerVersionInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::publish_layer_version::PublishLayerVersionInput {
            layer_name: self.layer_name,
            description: self.description,
            content: self.content,
            compatible_runtimes: self.compatible_runtimes,
            license_info: self.license_info,
            compatible_architectures: self.compatible_architectures,
        })
    }
}
