// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAccounts`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::set_next_token):<br>required: **false**<br><p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::set_max_results):<br>required: **false**<br><p>This is the number of items clients can request per page.</p><br>
    ///   - [`access_token(impl Into<String>)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::access_token) / [`set_access_token(Option<String>)`](crate::operation::list_accounts::builders::ListAccountsFluentBuilder::set_access_token):<br>required: **true**<br><p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p><br>
    /// - On success, responds with [`ListAccountsOutput`](crate::operation::list_accounts::ListAccountsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_accounts::ListAccountsOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
    ///   - [`account_list(Option<Vec::<AccountInfo>>)`](crate::operation::list_accounts::ListAccountsOutput::account_list): <p>A paginated response with the list of account information and the next token if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAccountsError>`](crate::operation::list_accounts::ListAccountsError)
    pub fn list_accounts(&self) -> crate::operation::list_accounts::builders::ListAccountsFluentBuilder {
        crate::operation::list_accounts::builders::ListAccountsFluentBuilder::new(self.handle.clone())
    }
}
