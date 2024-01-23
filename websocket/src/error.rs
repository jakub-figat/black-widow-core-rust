pub(crate) type HandlerResult = Result<(), HandlerError>;

pub(crate) enum HandlerError {
    ActionError(String),
    SenderError(String),
}
