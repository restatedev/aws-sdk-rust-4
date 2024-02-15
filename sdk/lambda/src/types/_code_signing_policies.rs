// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Code signing configuration <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-codesigning.html#config-codesigning-policies">policies</a> specify the validation failure action for signature mismatch or expiry.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CodeSigningPolicies {
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log.</p>
    /// <p>Default value: <code>Warn</code></p>
    pub untrusted_artifact_on_deployment: ::std::option::Option<crate::types::CodeSigningPolicy>,
}
impl CodeSigningPolicies {
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log.</p>
    /// <p>Default value: <code>Warn</code></p>
    pub fn untrusted_artifact_on_deployment(&self) -> ::std::option::Option<&crate::types::CodeSigningPolicy> {
        self.untrusted_artifact_on_deployment.as_ref()
    }
}
impl CodeSigningPolicies {
    /// Creates a new builder-style object to manufacture [`CodeSigningPolicies`](crate::types::CodeSigningPolicies).
    pub fn builder() -> crate::types::builders::CodeSigningPoliciesBuilder {
        crate::types::builders::CodeSigningPoliciesBuilder::default()
    }
}

/// A builder for [`CodeSigningPolicies`](crate::types::CodeSigningPolicies).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CodeSigningPoliciesBuilder {
    pub(crate) untrusted_artifact_on_deployment: ::std::option::Option<crate::types::CodeSigningPolicy>,
}
impl CodeSigningPoliciesBuilder {
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log.</p>
    /// <p>Default value: <code>Warn</code></p>
    pub fn untrusted_artifact_on_deployment(mut self, input: crate::types::CodeSigningPolicy) -> Self {
        self.untrusted_artifact_on_deployment = ::std::option::Option::Some(input);
        self
    }
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log.</p>
    /// <p>Default value: <code>Warn</code></p>
    pub fn set_untrusted_artifact_on_deployment(mut self, input: ::std::option::Option<crate::types::CodeSigningPolicy>) -> Self {
        self.untrusted_artifact_on_deployment = input;
        self
    }
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log.</p>
    /// <p>Default value: <code>Warn</code></p>
    pub fn get_untrusted_artifact_on_deployment(&self) -> &::std::option::Option<crate::types::CodeSigningPolicy> {
        &self.untrusted_artifact_on_deployment
    }
    /// Consumes the builder and constructs a [`CodeSigningPolicies`](crate::types::CodeSigningPolicies).
    pub fn build(self) -> crate::types::CodeSigningPolicies {
        crate::types::CodeSigningPolicies {
            untrusted_artifact_on_deployment: self.untrusted_artifact_on_deployment,
        }
    }
}
