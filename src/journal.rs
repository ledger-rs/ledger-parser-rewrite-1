/*!
 * journal.cc
 */

use crate::{
    account::AccountT,
    context::{ParseContextStackT, ParseContextT},
    textual::InstanceT,
};

///journal_t
pub struct JournalT<'a> {
    master: Box<AccountT<'a>>,
    bucket: Option<Box<AccountT<'a>>>,
    // xacts

    // known_payees:
    was_loaded: bool,
    check_payees: bool,
    day_break: bool,
    recursive_aliases: bool,
    no_aliases: bool,

    current_context: Option<&'a ParseContextT<'a>>,
}

impl JournalT<'_> {
    pub fn new() -> Self {
        Self {
            master: Box::new(AccountT::new()),
            bucket: None,
            current_context: None,
            was_loaded: false,
            check_payees: false,
            day_break: false,
            //   checking_style    = CHECK_NORMAL;
            recursive_aliases: false,
            no_aliases: false,
        }
    }

    pub fn read(&self, context: &ParseContextStackT) -> usize {
        let mut count: usize = 0;

        //let current: ParseContextT(context.get_current());
        let current: ParseContextT;

        count = read_textual(context);

        todo!()
    }
}

pub(crate) fn read_textual(context_stack: &ParseContextStackT) -> usize {
    // TRACE_START(parsing_total, 1, "Total time spent parsing text:");
    // {
    //   instance_t instance(context_stack, context_stack.get_current(), NULL,
    //                       checking_style == journal_t::CHECK_PERMISSIVE);
    let instance: InstanceT;
    // temporary init
    instance = InstanceT::new(context_stack, context_stack.get_current(), None, false);

    //   instance.apply_stack.push_front
    //     (application_t("account", context_stack.get_current().master));
    //   instance.parse();
    instance.parse();

    // }
    // TRACE_STOP(parsing_total, 1);

    // // Apply any deferred postings at this time
    // master->apply_deferred_posts();

    // // These tracers were started in textual.cc
    // TRACE_FINISH(xact_text, 1);
    // TRACE_FINISH(xact_details, 1);
    // TRACE_FINISH(xact_posts, 1);
    // TRACE_FINISH(xacts, 1);
    // TRACE_FINISH(instance_parse, 1); // report per-instance timers
    // TRACE_FINISH(parsing_total, 1);

    // if (context_stack.get_current().errors > 0)
    //   throw error_count(context_stack.get_current().errors,
    //                     context_stack.get_current().last);

    // return context_stack.get_current().count;

    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read() {
        todo!()
    }

    #[test]
    fn test_read_textual() {
        todo!()
    }
}
