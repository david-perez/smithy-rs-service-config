// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[derive(
    std::clone::Clone, std::cmp::Eq, std::cmp::PartialEq, std::fmt::Debug, std::hash::Hash,
)]
pub struct OperationInput {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl OperationInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl OperationInput {
    /// Creates a new builder-style object to manufacture [`OperationInput`](crate::input::OperationInput).
    pub fn builder() -> crate::input::operation_input::Builder {
        crate::input::operation_input::Builder::default()
    }
}
impl crate::constrained::Constrained for crate::input::OperationInput {
    type Unconstrained = crate::input::operation_input::Builder;
}
/// See [`OperationInput`](crate::input::OperationInput).
///
pub mod operation_input {

    impl std::convert::From<Builder> for crate::input::OperationInput {
        fn from(builder: Builder) -> Self {
            builder.build()
        }
    }
    /// A builder for [`OperationInput`](crate::input::OperationInput).
    #[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub(crate) fn set_message(mut self, input: Option<impl Into<std::string::String>>) -> Self {
            self.message = input.map(|v| v.into());
            self
        }
        /// Consumes the builder and constructs a [`OperationInput`](crate::input::OperationInput).
        pub fn build(self) -> crate::input::OperationInput {
            self.build_enforcing_all_constraints()
        }
        fn build_enforcing_all_constraints(self) -> crate::input::OperationInput {
            crate::input::OperationInput {
                message: self.message,
            }
        }
    }
}
