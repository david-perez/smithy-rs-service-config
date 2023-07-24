// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
pub struct Operation;

impl ::aws_smithy_http_server::operation::OperationShape for Operation {
    const ID: ::aws_smithy_http_server::shape_id::ShapeId =
        ::aws_smithy_http_server::shape_id::ShapeId::new(
            "com.amazonaws.simple#Operation",
            "com.amazonaws.simple",
            "Operation",
        );

    type Input = crate::input::OperationInput;
    type Output = crate::output::OperationOutput;
    type Error = std::convert::Infallible;
}

impl ::aws_smithy_http_server::instrumentation::sensitivity::Sensitivity for Operation {
    type RequestFmt = ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::sensitivity::uri::MakeUri<
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
            ::aws_smithy_http_server::instrumentation::MakeIdentity,
        >,
    >;
    type ResponseFmt = ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt<
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
        ::aws_smithy_http_server::instrumentation::MakeIdentity,
    >;

    fn request_fmt() -> Self::RequestFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::RequestFmt::new()
    }

    fn response_fmt() -> Self::ResponseFmt {
        ::aws_smithy_http_server::instrumentation::sensitivity::ResponseFmt::new()
    }
}
