// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_alias_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_alias::UpdateAliasInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.function_version {
        object.key("FunctionVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.revision_id {
        object.key("RevisionId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.routing_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("RoutingConfig").start_object();
        crate::protocol_serde::shape_alias_routing_configuration::ser_alias_routing_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
