/*!
 * Rewrite of textual.cc
 */

use crate::context::{ParseContextStackT, ParseContextT};

/// instance_t
pub(crate) struct InstanceT<'a> {
    // parse_context_stack_t&   context_stack;
    context_stack: &'a ParseContextStackT,
    // parse_context_t&         context;
    context: &'a ParseContextT,
    // std::istream&            in;
    // instance_t *             parent;
    // std::list<application_t> apply_stack;
    // bool                     no_assertions;
    // #if defined(TIMELOG_SUPPORT)
    // time_log_t               timelog;
    // #endif
}

impl<'a> InstanceT<'a> {
    pub(crate) fn new(
        context_stack: &'a ParseContextStackT,
        context: &'a ParseContextT,
        // instance_t *           _parent = NULL,
        // const bool             _no_assertions = false
    ) -> Self {
        Self { context_stack, context }
    }

    pub(crate) fn parse(&self) {
        //     INFO("Parsing file " << context.pathname);
        //todo: log::info!("Parsing file {}", self.context.pathname);

        //     TRACE_START(instance_parse, 1, "Done parsing file " << context.pathname);

        //     if (! in.good() || in.eof())
        //       return;

        //     context.linenum  = 0;
        //     context.curr_pos = in.tellg();

        //     bool error_flag = false;

        //     while (in.good() && ! in.eof()) {
        //       try {
        //         read_next_directive(error_flag);
        //       }
        //       catch (const std::exception& err) {
        //         error_flag = true;

        //         string current_context = error_context();

        //         if (parent) {
        //           std::list<instance_t *> instances;

        //           for (instance_t * instance = parent;
        //                instance;
        //                instance = instance->parent)
        //             instances.push_front(instance);

        //           foreach (instance_t * instance, instances)
        //             add_error_context(_f("In file included from %1%")
        //                               % instance->context.location());
        //         }
        //         add_error_context(_f("While parsing file %1%") % context.location());

        //         if (caught_signal != NONE_CAUGHT)
        //           throw;

        //         string err_context = error_context();
        //         if (! err_context.empty())
        //           std::cerr << err_context << std::endl;

        //         if (! current_context.empty())
        //           std::cerr << current_context << std::endl;

        //         std::cerr << _("Error: ") << err.what() << std::endl;
        //         context.errors++;
        //         if (! current_context.empty())
        //             context.last = current_context + "\n" + err.what();
        //         else
        //             context.last = err.what();
        //       }
        //     }

        //     if (apply_stack.front().value.type() == typeid(optional<datetime_t>))
        //       epoch = boost::get<optional<datetime_t> >(apply_stack.front().value);

        //     apply_stack.pop_front();

        //   #if defined(TIMELOG_SUPPORT)
        //     timelog.close();
        //   #endif // TIMELOG_SUPPORT

        //     TRACE_STOP(instance_parse, 1);
    }
}
