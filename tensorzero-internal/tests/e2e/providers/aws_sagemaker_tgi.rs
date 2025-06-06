use std::collections::HashMap;

use crate::providers::common::{E2ETestProvider, E2ETestProviders};

crate::generate_provider_tests!(get_providers);
crate::generate_batch_inference_tests!(get_providers);

// The main goal of our sagemaker tests to to make sure that the AWS client
// and serialization/deserialization (including stream handling) are working correctly.
// The actual Sagemaker instance deploys some arbitrary model and provider
// (e.g. ollama serving gemma-3-1b), so it's not really useful to test things
// like tool-calling which don't depend on anything Sagemaker-specific.
//
// As a result, we leave most of the fields in `E2ETestProviders` empty.
async fn get_providers() -> E2ETestProviders {
    let standard_providers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "aws-sagemaker-tgi".to_string(),
        model_name: "gemma-3-1b-aws-sagemaker-tgi".into(),
        model_provider_name: "aws_sagemaker".into(),
        credentials: HashMap::new(),
    }];

    E2ETestProviders {
        simple_inference: standard_providers.clone(),
        extra_body_inference: vec![],
        bad_auth_extra_headers: vec![],
        reasoning_inference: vec![],
        inference_params_inference: vec![],
        inference_params_dynamic_credentials: vec![],
        tool_use_inference: vec![],
        tool_multi_turn_inference: vec![],
        dynamic_tool_use_inference: vec![],
        parallel_tool_use_inference: vec![],
        json_mode_inference: vec![],
        json_mode_off_inference: vec![],
        image_inference: vec![],
        pdf_inference: vec![],
        shorthand_inference: vec![],
    }
}
