/*!
 * context.h
 */

use std::path::{Path, PathBuf};

use crate::journal::JournalT;

const MAX_LINE: usize = 4096;

/// parse_context_t
pub(crate) struct ParseContextT {
    // static const MAX_LINE: usize = 4096;

    // shared_ptr<std::istream> stream;

    // path                   pathname;
    pathname: PathBuf,
    // path                   current_directory;
    current_directory: PathBuf,
    // journal_t *            journal;
    journal: JournalT,
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
