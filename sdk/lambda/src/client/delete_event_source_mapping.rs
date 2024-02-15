// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEventSourceMapping`](crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`uuid(impl Into<String>)`](crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingFluentBuilder::uuid) / [`set_uuid(Option<String>)`](crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingFluentBuilder::set_uuid):<br>required: **true**<br><p>The identifier of the event source mapping.</p><br>
    /// - On success, responds with [`DeleteEventSourceMappingOutput`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput) with field(s):
    ///   - [`uuid(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::uuid): <p>The identifier of the event source mapping.</p>
    ///   - [`starting_position(Option<EventSourcePosition>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::starting_position): <p>The position in a stream from which to start reading. Required for Amazon Kinesis and Amazon DynamoDB Stream event sources. <code>AT_TIMESTAMP</code> is supported only for Amazon Kinesis streams, Amazon DocumentDB, Amazon MSK, and self-managed Apache Kafka.</p>
    ///   - [`starting_position_timestamp(Option<DateTime>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::starting_position_timestamp): <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading. <code>StartingPositionTimestamp</code> cannot be in the future.</p>
    ///   - [`batch_size(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::batch_size): <p>The maximum number of records in each batch that Lambda pulls from your stream or queue and sends to your function. Lambda passes all of the records in the batch to the function in a single call, up to the payload limit for synchronous invocation (6 MB).</p> <p>Default value: Varies by service. For Amazon SQS, the default is 10. For all other services, the default is 100.</p> <p>Related setting: When you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    ///   - [`maximum_batching_window_in_seconds(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::maximum_batching_window_in_seconds): <p>The maximum amount of time, in seconds, that Lambda spends gathering records before invoking the function. You can configure <code>MaximumBatchingWindowInSeconds</code> to any value from 0 seconds to 300 seconds in increments of seconds.</p> <p>For streams and Amazon SQS event sources, the default batching window is 0 seconds. For Amazon MSK, Self-managed Apache Kafka, Amazon MQ, and DocumentDB event sources, the default batching window is 500 ms. Note that because you can only change <code>MaximumBatchingWindowInSeconds</code> in increments of seconds, you cannot revert back to the 500 ms default batching window after you have changed it. To restore the default batching window, you must create a new event source mapping.</p> <p>Related setting: For streams and Amazon SQS event sources, when you set <code>BatchSize</code> to a value greater than 10, you must set <code>MaximumBatchingWindowInSeconds</code> to at least 1.</p>
    ///   - [`parallelization_factor(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::parallelization_factor): <p>(Kinesis and DynamoDB Streams only) The number of batches to process concurrently from each shard. The default value is 1.</p>
    ///   - [`event_source_arn(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::event_source_arn): <p>The Amazon Resource Name (ARN) of the event source.</p>
    ///   - [`filter_criteria(Option<FilterCriteria>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::filter_criteria): <p>An object that defines the filter criteria that determine whether Lambda should process an event. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-eventfiltering.html">Lambda event filtering</a>.</p>
    ///   - [`function_arn(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::function_arn): <p>The ARN of the Lambda function.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::last_modified): <p>The date that the event source mapping was last updated or that its state changed.</p>
    ///   - [`last_processing_result(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::last_processing_result): <p>The result of the last Lambda invocation of your function.</p>
    ///   - [`state(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::state): <p>The state of the event source mapping. It can be one of the following: <code>Creating</code>, <code>Enabling</code>, <code>Enabled</code>, <code>Disabling</code>, <code>Disabled</code>, <code>Updating</code>, or <code>Deleting</code>.</p>
    ///   - [`state_transition_reason(Option<String>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::state_transition_reason): <p>Indicates whether a user or Lambda made the last change to the event source mapping.</p>
    ///   - [`destination_config(Option<DestinationConfig>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::destination_config): <p>(Kinesis and DynamoDB Streams only) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    ///   - [`topics(Option<Vec::<String>>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::topics): <p>The name of the Kafka topic.</p>
    ///   - [`queues(Option<Vec::<String>>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::queues): <p>(Amazon MQ) The name of the Amazon MQ broker destination queue to consume.</p>
    ///   - [`source_access_configurations(Option<Vec::<SourceAccessConfiguration>>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::source_access_configurations): <p>An array of the authentication protocol, VPC components, or virtual host to secure and define your event source.</p>
    ///   - [`self_managed_event_source(Option<SelfManagedEventSource>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::self_managed_event_source): <p>The self-managed Apache Kafka cluster for your event source.</p>
    ///   - [`maximum_record_age_in_seconds(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::maximum_record_age_in_seconds): <p>(Kinesis and DynamoDB Streams only) Discard records older than the specified age. The default value is -1, which sets the maximum age to infinite. When the value is set to infinite, Lambda never discards old records.</p><note>  <p>The minimum valid value for maximum record age is 60s. Although values less than 60 and greater than -1 fall within the parameter's absolute range, they are not allowed</p> </note>
    ///   - [`bisect_batch_on_function_error(Option<bool>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::bisect_batch_on_function_error): <p>(Kinesis and DynamoDB Streams only) If the function returns an error, split the batch in two and retry. The default value is false.</p>
    ///   - [`maximum_retry_attempts(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::maximum_retry_attempts): <p>(Kinesis and DynamoDB Streams only) Discard records after the specified number of retries. The default value is -1, which sets the maximum number of retries to infinite. When MaximumRetryAttempts is infinite, Lambda retries failed records until the record expires in the event source.</p>
    ///   - [`tumbling_window_in_seconds(Option<i32>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::tumbling_window_in_seconds): <p>(Kinesis and DynamoDB Streams only) The duration in seconds of a processing window for DynamoDB and Kinesis Streams event sources. A value of 0 seconds indicates no tumbling window.</p>
    ///   - [`function_response_types(Option<Vec::<FunctionResponseType>>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::function_response_types): <p>(Kinesis, DynamoDB Streams, and Amazon SQS) A list of current response type enums applied to the event source mapping.</p>
    ///   - [`amazon_managed_kafka_event_source_config(Option<AmazonManagedKafkaEventSourceConfig>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::amazon_managed_kafka_event_source_config): <p>Specific configuration settings for an Amazon Managed Streaming for Apache Kafka (Amazon MSK) event source.</p>
    ///   - [`self_managed_kafka_event_source_config(Option<SelfManagedKafkaEventSourceConfig>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::self_managed_kafka_event_source_config): <p>Specific configuration settings for a self-managed Apache Kafka event source.</p>
    ///   - [`scaling_config(Option<ScalingConfig>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::scaling_config): <p>(Amazon SQS only) The scaling configuration for the event source. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html#events-sqs-max-concurrency">Configuring maximum concurrency for Amazon SQS event sources</a>.</p>
    ///   - [`document_db_event_source_config(Option<DocumentDbEventSourceConfig>)`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingOutput::document_db_event_source_config): <p>Specific configuration settings for a DocumentDB event source.</p>
    /// - On failure, responds with [`SdkError<DeleteEventSourceMappingError>`](crate::operation::delete_event_source_mapping::DeleteEventSourceMappingError)
    pub fn delete_event_source_mapping(&self) -> crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingFluentBuilder {
        crate::operation::delete_event_source_mapping::builders::DeleteEventSourceMappingFluentBuilder::new(self.handle.clone())
    }
}
