// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>AssumeRole</code> request, including temporary Amazon Web Services credentials that can be used to make Amazon Web Services requests.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AssumeRoleOutput {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub credentials: ::std::option::Option<crate::types::Credentials>,
    /// <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    pub assumed_role_user: ::std::option::Option<crate::types::AssumedRoleUser>,
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub packed_policy_size: ::std::option::Option<i32>,
    /// <p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation.</p>
    /// <p>You can require users to specify a source identity when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>
    /// <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    pub source_identity: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl AssumeRoleOutput {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn credentials(&self) -> ::std::option::Option<&crate::types::Credentials> {
        self.credentials.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    pub fn assumed_role_user(&self) -> ::std::option::Option<&crate::types::AssumedRoleUser> {
        self.assumed_role_user.as_ref()
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn packed_policy_size(&self) -> ::std::option::Option<i32> {
        self.packed_policy_size
    }
    /// <p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation.</p>
    /// <p>You can require users to specify a source identity when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>
    /// <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    pub fn source_identity(&self) -> ::std::option::Option<&str> {
        self.source_identity.as_deref()
    }
}
impl ::std::fmt::Debug for AssumeRoleOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssumeRoleOutput");
        formatter.field("credentials", &"*** Sensitive Data Redacted ***");
        formatter.field("assumed_role_user", &self.assumed_role_user);
        formatter.field("packed_policy_size", &self.packed_policy_size);
        formatter.field("source_identity", &self.source_identity);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for AssumeRoleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssumeRoleOutput {
    /// Creates a new builder-style object to manufacture [`AssumeRoleOutput`](crate::operation::assume_role::AssumeRoleOutput).
    pub fn builder() -> crate::operation::assume_role::builders::AssumeRoleOutputBuilder {
        crate::operation::assume_role::builders::AssumeRoleOutputBuilder::default()
    }
}

/// A builder for [`AssumeRoleOutput`](crate::operation::assume_role::AssumeRoleOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AssumeRoleOutputBuilder {
    pub(crate) credentials: ::std::option::Option<crate::types::Credentials>,
    pub(crate) assumed_role_user: ::std::option::Option<crate::types::AssumedRoleUser>,
    pub(crate) packed_policy_size: ::std::option::Option<i32>,
    pub(crate) source_identity: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl AssumeRoleOutputBuilder {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn credentials(mut self, input: crate::types::Credentials) -> Self {
        self.credentials = ::std::option::Option::Some(input);
        self
    }
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn set_credentials(mut self, input: ::std::option::Option<crate::types::Credentials>) -> Self {
        self.credentials = input;
        self
    }
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p><note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn get_credentials(&self) -> &::std::option::Option<crate::types::Credentials> {
        &self.credentials
    }
    /// <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    pub fn assumed_role_user(mut self, input: crate::types::AssumedRoleUser) -> Self {
        self.assumed_role_user = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    pub fn set_assumed_role_user(mut self, input: ::std::option::Option<crate::types::AssumedRoleUser>) -> Self {
        self.assumed_role_user = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) and the assumed role ID, which are identifiers that you can use to refer to the resulting temporary security credentials. For example, you can reference these credentials as a principal in a resource-based policy by using the ARN or assumed role ID. The ARN and ID include the <code>RoleSessionName</code> that you specified when you called <code>AssumeRole</code>.</p>
    pub fn get_assumed_role_user(&self) -> &::std::option::Option<crate::types::AssumedRoleUser> {
        &self.assumed_role_user
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn packed_policy_size(mut self, input: i32) -> Self {
        self.packed_policy_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn set_packed_policy_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.packed_policy_size = input;
        self
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn get_packed_policy_size(&self) -> &::std::option::Option<i32> {
        &self.packed_policy_size
    }
    /// <p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation.</p>
    /// <p>You can require users to specify a source identity when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>
    /// <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    pub fn source_identity(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_identity = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation.</p>
    /// <p>You can require users to specify a source identity when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>
    /// <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    pub fn set_source_identity(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_identity = input;
        self
    }
    /// <p>The source identity specified by the principal that is calling the <code>AssumeRole</code> operation.</p>
    /// <p>You can require users to specify a source identity when they assume a role. You do this by using the <code>sts:SourceIdentity</code> condition key in a role trust policy. You can use source identity information in CloudTrail logs to determine who took actions with a role. You can use the <code>aws:SourceIdentity</code> condition key to further control access to Amazon Web Services resources based on the value of source identity. For more information about using source identity, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_control-access_monitor.html">Monitor and control actions taken with assumed roles</a> in the <i>IAM User Guide</i>.</p>
    /// <p>The regex used to validate this parameter is a string of characters consisting of upper- and lower-case alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@-</p>
    pub fn get_source_identity(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_identity
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssumeRoleOutput`](crate::operation::assume_role::AssumeRoleOutput).
    pub fn build(self) -> crate::operation::assume_role::AssumeRoleOutput {
        crate::operation::assume_role::AssumeRoleOutput {
            credentials: self.credentials,
            assumed_role_user: self.assumed_role_user,
            packed_policy_size: self.packed_policy_size,
            source_identity: self.source_identity,
            _request_id: self._request_id,
        }
    }
}
impl ::std::fmt::Debug for AssumeRoleOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssumeRoleOutputBuilder");
        formatter.field("credentials", &"*** Sensitive Data Redacted ***");
        formatter.field("assumed_role_user", &self.assumed_role_user);
        formatter.field("packed_policy_size", &self.packed_policy_size);
        formatter.field("source_identity", &self.source_identity);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
