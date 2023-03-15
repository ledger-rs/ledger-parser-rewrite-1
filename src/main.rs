use ledger_parser_rewrite_1::global::GlobalScopeT;

fn main() {
    //println!("Hello, world!");

    let mut status = 1;

    //   handle_debug_options(argc, argv);

    // INFO("Ledger starting");
    log::info!("Ledger starting");

    // Create the session object, which maintains nearly all state relating to
    // this invocation of Ledger; and register all known journal parsers.
    // global_scope = new global_scope_t(envp);
    let global_scope: GlobalScopeT = GlobalScopeT::new();

    // global_scope->session().set_flush_on_next_data_file(true);

    // if (global_scope->HANDLED(script_)) {
      // Ledger is being invoked as a script command interpreter
    //   global_scope->session().read_journal_files();
    global_scope.session().read_journal_files();

    status = 0;

}
