use crate::session::SessionT;

/**
global.h

*/

pub struct GlobalScopeT<'a> {
    // shared_ptr<session_t> session_ptr;
    session_ptr: SessionT<'a>

}

impl GlobalScopeT<'_> {
    pub fn new() -> Self {
//   epoch = CURRENT_TIME();

        Self { session_ptr: SessionT::new()  }
    }

    pub fn session(&self) -> &SessionT {
        &self.session_ptr
    }

    fn show_version_info() {
        println!("Ledger ");
    }
}