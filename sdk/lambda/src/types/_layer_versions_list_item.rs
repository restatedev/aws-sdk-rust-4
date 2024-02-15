// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LayerVersionsListItem {
    /// <p>The ARN of the layer version.</p>
    pub layer_version_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version number.</p>
    pub version: i64,
    /// <p>The description of the version.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    pub created_date: ::std::option::Option<::std::string::String>,
    /// <p>The layer's compatible runtimes.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
    /// <p>The layer's open-source license.</p>
    pub license_info: ::std::option::Option<::std::string::String>,
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
}
impl LayerVersionsListItem {
    /// <p>The ARN of the layer version.</p>
    pub fn layer_version_arn(&self) -> ::std::option::Option<&str> {
        self.layer_version_arn.as_deref()
    }
    /// <p>The version number.</p>
    pub fn version(&self) -> i64 {
        self.version
    }
    /// <p>The description of the version.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    pub fn created_date(&self) -> ::std::option::Option<&str> {
        self.created_date.as_deref()
    }
    /// <p>The layer's compatible runtimes.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.compatible_runtimes.is_none()`.
    pub fn compatible_runtimes(&self) -> &[crate::types::Runtime] {
        self.compatible_runtimes.as_deref().unwrap_or_default()
    }
    /// <p>The layer's open-source license.</p>
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
impl LayerVersionsListItem {
    /// Creates a new builder-style object to manufacture [`LayerVersionsListItem`](crate::types::LayerVersionsListItem).
    pub fn builder() -> crate::types::builders::LayerVersionsListItemBuilder {
        crate::types::builders::LayerVersionsListItemBuilder::default()
    }
}

/// A builder for [`LayerVersionsListItem`](crate::types::LayerVersionsListItem).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LayerVersionsListItemBuilder {
    pub(crate) layer_version_arn: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<i64>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created_date: ::std::option::Option<::std::string::String>,
    pub(crate) compatible_runtimes: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>,
    pub(crate) license_info: ::std::option::Option<::std::string::String>,
    pub(crate) compatible_architectures: ::std::option::Option<::std::vec::Vec<crate::types::Architecture>>,
}
impl LayerVersionsListItemBuilder {
    /// <p>The ARN of the layer version.</p>
    pub fn layer_version_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.layer_version_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the layer version.</p>
    pub fn set_layer_version_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.layer_version_arn = input;
        self
    }
    /// <p>The ARN of the layer version.</p>
    pub fn get_layer_version_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.layer_version_arn
    }
    /// <p>The version number.</p>
    pub fn version(mut self, input: i64) -> Self {
        self.version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version number.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.version = input;
        self
    }
    /// <p>The version number.</p>
    pub fn get_version(&self) -> &::std::option::Option<i64> {
        &self.version
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
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    pub fn created_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.created_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    pub fn set_created_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.created_date = input;
        self
    }
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    pub fn get_created_date(&self) -> &::std::option::Option<::std::string::String> {
        &self.created_date
    }
    /// Appends an item to `compatible_runtimes`.
    ///
    /// To override the contents of this collection use [`set_compatible_runtimes`](Self::set_compatible_runtimes).
    ///
    /// <p>The layer's compatible runtimes.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn compatible_runtimes(mut self, input: crate::types::Runtime) -> Self {
        let mut v = self.compatible_runtimes.unwrap_or_default();
        v.push(input);
        self.compatible_runtimes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The layer's compatible runtimes.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn set_compatible_runtimes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Runtime>>) -> Self {
        self.compatible_runtimes = input;
        self
    }
    /// <p>The layer's compatible runtimes.</p>
    /// <p>The following list includes deprecated runtimes. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html#runtime-support-policy">Runtime deprecation policy</a>.</p>
    pub fn get_compatible_runtimes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Runtime>> {
        &self.compatible_runtimes
    }
    /// <p>The layer's open-source license.</p>
    pub fn license_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.license_info = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The layer's open-source license.</p>
    pub fn set_license_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.license_info = input;
        self
    }
    /// <p>The layer's open-source license.</p>
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
    /// Consumes the builder and constructs a [`LayerVersionsListItem`](crate::types::LayerVersionsListItem).
    pub fn build(self) -> crate::types::LayerVersionsListItem {
        crate::types::LayerVersionsListItem {
            layer_version_arn: self.layer_version_arn,
            version: self.version.unwrap_or_default(),
            description: self.description,
            created_date: self.created_date,
            compatible_runtimes: self.compatible_runtimes,
            license_info: self.license_info,
            compatible_architectures: self.compatible_architectures,
        }
    }
}
