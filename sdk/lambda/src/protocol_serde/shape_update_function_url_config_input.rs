// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_function_url_config_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_function_url_config::UpdateFunctionUrlConfigInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.auth_type {
        object.key("AuthType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cors {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Cors").start_object();
        crate::protocol_serde::shape_cors::ser_cors(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.invoke_mode {
        object.key("InvokeMode").string(var_4.as_str());
    }
    Ok(())
}
