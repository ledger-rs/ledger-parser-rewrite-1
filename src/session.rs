use std::path::PathBuf;

/**
Session
 */
use crate::{account::AccountT, context::ParseContextStackT, journal::JournalT};

pub struct SessionT {
    // unique_ptr<journal_t> journal;
    journal: JournalT,
    // parse_context_stack_t parsing_context;
    parsing_context: ParseContextStackT,

    // Options
    data_files: Vec<PathBuf>,
}

impl SessionT {
    pub fn new() -> Self {
        Self { journal: JournalT::new(), parsing_context: (), data_files: () }
    }

    //   journal_t * read_journal(const path& pathname);
    // fn read_journal(&pathname: path) {
    //     todo!()
    // }

    // journal_t * session_t::read_journal_files()
    pub fn read_journal_files(&self) -> JournalT {
        //   INFO_START(journal, "Read journal file");

        // string master_account;
        let master_account: String;
        // if (HANDLED(master_account_))
        //   master_account = HANDLER(master_account_).str();
        master_account = "".to_owned();

        self.read_data(&master_account);

        // INFO_FINISH(journal);

        // return journal.get();
        todo!("self.journal.get()");
    }

    // std::size_t session_t::read_data(const string& master_account)
    pub fn read_data(&self, master_account: &str) {
        // bool populated_data_files = false;

        // if (HANDLER(file_).data_files.empty()) {
        // populated_data_files = true;

        // std::size_t xact_count = 0;
        let mut xact_count = 0;

        let acct: AccountT;
        //   if (master_account.empty())
        //     acct = journal->master;
        //   else
        //     acct = journal->find_account(master_account);

        // optional<path> price_db_path;

        // foreach (const path& pathname, HANDLER(file_).data_files) {
        for pathname in &self.data_files {
            //todo self.parsing_context.

            // xact_count += journal->read(parsing_context);
            xact_count += self.journal.read(&self.parsing_context);
        }
    }
}
