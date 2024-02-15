// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_event_source_mapping_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::EventSourceMappingConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EventSourceMappingConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "UUID" => {
                            builder = builder.set_uuid(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "StartingPosition" => {
                            builder = builder.set_starting_position(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EventSourcePosition::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "StartingPositionTimestamp" => {
                            builder = builder.set_starting_position_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "BatchSize" => {
                            builder = builder.set_batch_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaximumBatchingWindowInSeconds" => {
                            builder = builder.set_maximum_batching_window_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ParallelizationFactor" => {
                            builder = builder.set_parallelization_factor(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "EventSourceArn" => {
                            builder = builder.set_event_source_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FilterCriteria" => {
                            builder = builder.set_filter_criteria(crate::protocol_serde::shape_filter_criteria::de_filter_criteria(tokens)?);
                        }
                        "FunctionArn" => {
                            builder = builder.set_function_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "LastModified" => {
                            builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastProcessingResult" => {
                            builder = builder.set_last_processing_result(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "State" => {
                            builder = builder.set_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "StateTransitionReason" => {
                            builder = builder.set_state_transition_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DestinationConfig" => {
                            builder = builder.set_destination_config(crate::protocol_serde::shape_destination_config::de_destination_config(tokens)?);
                        }
                        "Topics" => {
                            builder = builder.set_topics(crate::protocol_serde::shape_topics::de_topics(tokens)?);
                        }
                        "Queues" => {
                            builder = builder.set_queues(crate::protocol_serde::shape_queues::de_queues(tokens)?);
                        }
                        "SourceAccessConfigurations" => {
                            builder = builder.set_source_access_configurations(
                                crate::protocol_serde::shape_source_access_configurations::de_source_access_configurations(tokens)?,
                            );
                        }
                        "SelfManagedEventSource" => {
                            builder = builder.set_self_managed_event_source(
                                crate::protocol_serde::shape_self_managed_event_source::de_self_managed_event_source(tokens)?,
                            );
                        }
                        "MaximumRecordAgeInSeconds" => {
                            builder = builder.set_maximum_record_age_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "BisectBatchOnFunctionError" => {
                            builder = builder
                                .set_bisect_batch_on_function_error(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "MaximumRetryAttempts" => {
                            builder = builder.set_maximum_retry_attempts(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "TumblingWindowInSeconds" => {
                            builder = builder.set_tumbling_window_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "FunctionResponseTypes" => {
                            builder = builder.set_function_response_types(
                                crate::protocol_serde::shape_function_response_type_list::de_function_response_type_list(tokens)?,
                            );
                        }
                        "AmazonManagedKafkaEventSourceConfig" => {
                            builder = builder.set_amazon_managed_kafka_event_source_config(
                                crate::protocol_serde::shape_amazon_managed_kafka_event_source_config::de_amazon_managed_kafka_event_source_config(
                                    tokens,
                                )?,
                            );
                        }
                        "SelfManagedKafkaEventSourceConfig" => {
                            builder = builder.set_self_managed_kafka_event_source_config(
                                crate::protocol_serde::shape_self_managed_kafka_event_source_config::de_self_managed_kafka_event_source_config(
                                    tokens,
                                )?,
                            );
                        }
                        "ScalingConfig" => {
                            builder = builder.set_scaling_config(crate::protocol_serde::shape_scaling_config::de_scaling_config(tokens)?);
                        }
                        "DocumentDBEventSourceConfig" => {
                            builder = builder.set_document_db_event_source_config(
                                crate::protocol_serde::shape_document_db_event_source_config::de_document_db_event_source_config(tokens)?,
                            );
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
