// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_function_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::FunctionConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FunctionConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FunctionName" => {
                            builder = builder.set_function_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FunctionArn" => {
                            builder = builder.set_function_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Runtime" => {
                            builder = builder.set_runtime(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Runtime::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Role" => {
                            builder = builder.set_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Handler" => {
                            builder = builder.set_handler(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CodeSize" => {
                            builder = builder.set_code_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Timeout" => {
                            builder = builder.set_timeout(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MemorySize" => {
                            builder = builder.set_memory_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "LastModified" => {
                            builder = builder.set_last_modified(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CodeSha256" => {
                            builder = builder.set_code_sha256(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Version" => {
                            builder = builder.set_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "VpcConfig" => {
                            builder = builder.set_vpc_config(crate::protocol_serde::shape_vpc_config_response::de_vpc_config_response(tokens)?);
                        }
                        "DeadLetterConfig" => {
                            builder = builder.set_dead_letter_config(crate::protocol_serde::shape_dead_letter_config::de_dead_letter_config(tokens)?);
                        }
                        "Environment" => {
                            builder = builder.set_environment(crate::protocol_serde::shape_environment_response::de_environment_response(tokens)?);
                        }
                        "KMSKeyArn" => {
                            builder = builder.set_kms_key_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TracingConfig" => {
                            builder =
                                builder.set_tracing_config(crate::protocol_serde::shape_tracing_config_response::de_tracing_config_response(tokens)?);
                        }
                        "MasterArn" => {
                            builder = builder.set_master_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RevisionId" => {
                            builder = builder.set_revision_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Layers" => {
                            builder = builder.set_layers(crate::protocol_serde::shape_layers_reference_list::de_layers_reference_list(tokens)?);
                        }
                        "State" => {
                            builder = builder.set_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::State::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "StateReason" => {
                            builder = builder.set_state_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "StateReasonCode" => {
                            builder = builder.set_state_reason_code(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::StateReasonCode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LastUpdateStatus" => {
                            builder = builder.set_last_update_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::LastUpdateStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LastUpdateStatusReason" => {
                            builder = builder.set_last_update_status_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "LastUpdateStatusReasonCode" => {
                            builder = builder.set_last_update_status_reason_code(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::LastUpdateStatusReasonCode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "FileSystemConfigs" => {
                            builder = builder
                                .set_file_system_configs(crate::protocol_serde::shape_file_system_config_list::de_file_system_config_list(tokens)?);
                        }
                        "PackageType" => {
                            builder = builder.set_package_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::PackageType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ImageConfigResponse" => {
                            builder = builder
                                .set_image_config_response(crate::protocol_serde::shape_image_config_response::de_image_config_response(tokens)?);
                        }
                        "SigningProfileVersionArn" => {
                            builder = builder.set_signing_profile_version_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SigningJobArn" => {
                            builder = builder.set_signing_job_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Architectures" => {
                            builder = builder.set_architectures(crate::protocol_serde::shape_architectures_list::de_architectures_list(tokens)?);
                        }
                        "EphemeralStorage" => {
                            builder = builder.set_ephemeral_storage(crate::protocol_serde::shape_ephemeral_storage::de_ephemeral_storage(tokens)?);
                        }
                        "SnapStart" => {
                            builder = builder.set_snap_start(crate::protocol_serde::shape_snap_start_response::de_snap_start_response(tokens)?);
                        }
                        "RuntimeVersionConfig" => {
                            builder = builder
                                .set_runtime_version_config(crate::protocol_serde::shape_runtime_version_config::de_runtime_version_config(tokens)?);
                        }
                        "LoggingConfig" => {
                            builder = builder.set_logging_config(crate::protocol_serde::shape_logging_config::de_logging_config(tokens)?);
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
