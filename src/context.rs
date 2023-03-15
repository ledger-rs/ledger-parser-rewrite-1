/*!
 * context.h
 */

use std::path::PathBuf;

use crate::journal::JournalT;

const MAX_LINE: usize = 4096;

/// parse_context_t
pub struct ParseContextT<'a> {
    // static const MAX_LINE: usize = 4096;

    // shared_ptr<std::istream> stream;

    // path                   pathname;
    pathname: PathBuf,
    // path                   current_directory;
    current_directory: PathBuf,
    // journal_t *            journal;
    journal: JournalT<'a>,
    // account_t *            master;
    // todo: master: AccountT
    // scope_t *              scope;
    // char                   linebuf[MAX_LINE + 1];
    // std::istream::pos_type line_beg_pos;
    // std::istream::pos_type curr_pos;
    // std::size_t            linenum;
    // std::size_t            errors;
    // std::size_t            count;
    // std::size_t            sequence;
    // std::string            last;
  
}

/// parse_context_stack_t
pub struct ParseContextStackT<'a> {
    parsing_context: Vec<ParseContextT<'a>>
}

impl ParseContextStackT<'_> {
    pub fn new() -> Self {
        Self { parsing_context: vec![] }
    }
    
    pub fn get_current(&self) -> &ParseContextT {
        //assert(! parsing_context.empty());
        assert!(!self.parsing_context.is_empty());

        //return parsing_context.front();
        &self.parsing_context[0]
    }
}
