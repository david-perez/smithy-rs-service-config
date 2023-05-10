// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_operation_output_output(
    value: &crate::output::OperationOutput,
) -> Result<String, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_operation_output::ser_operation_output(&mut object, value)?;
    object.finish();
    Ok(out)
}

pub fn ser_operation_output(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::output::OperationOutput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.message {
        object.key("message").string(var_1.as_str());
    }
    Ok(())
}