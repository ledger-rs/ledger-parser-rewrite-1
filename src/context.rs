/*!
 * context.h
 */

/// parse_context_t
pub(crate) struct ParseContextT {}

/// parse_context_stack_t
pub(crate) struct ParseContextStackT {
    parsing_context: Vec<ParseContextT>
}

impl ParseContextStackT {
    pub fn get_current(&self) -> &ParseContextT {
        //assert(! parsing_context.empty());
        assert!(!self.parsing_context.is_empty());

        //return parsing_context.front();
        &self.parsing_context[0]
    }
}
