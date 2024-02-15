// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListLayersOutput {
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>A list of function layers.</p>
    pub layers: ::std::option::Option<::std::vec::Vec<crate::types::LayersListItem>>,
    _request_id: Option<String>,
}
impl ListLayersOutput {
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>A list of function layers.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.layers.is_none()`.
    pub fn layers(&self) -> &[crate::types::LayersListItem] {
        self.layers.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ListLayersOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListLayersOutput {
    /// Creates a new builder-style object to manufacture [`ListLayersOutput`](crate::operation::list_layers::ListLayersOutput).
    pub fn builder() -> crate::operation::list_layers::builders::ListLayersOutputBuilder {
        crate::operation::list_layers::builders::ListLayersOutputBuilder::default()
    }
}

/// A builder for [`ListLayersOutput`](crate::operation::list_layers::ListLayersOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListLayersOutputBuilder {
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) layers: ::std::option::Option<::std::vec::Vec<crate::types::LayersListItem>>,
    _request_id: Option<String>,
}
impl ListLayersOutputBuilder {
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_marker
    }
    /// Appends an item to `layers`.
    ///
    /// To override the contents of this collection use [`set_layers`](Self::set_layers).
    ///
    /// <p>A list of function layers.</p>
    pub fn layers(mut self, input: crate::types::LayersListItem) -> Self {
        let mut v = self.layers.unwrap_or_default();
        v.push(input);
        self.layers = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of function layers.</p>
    pub fn set_layers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LayersListItem>>) -> Self {
        self.layers = input;
        self
    }
    /// <p>A list of function layers.</p>
    pub fn get_layers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LayersListItem>> {
        &self.layers
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListLayersOutput`](crate::operation::list_layers::ListLayersOutput).
    pub fn build(self) -> crate::operation::list_layers::ListLayersOutput {
        crate::operation::list_layers::ListLayersOutput {
            next_marker: self.next_marker,
            layers: self.layers,
            _request_id: self._request_id,
        }
    }
}
