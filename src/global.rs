use crate::session::SessionT;

/**
global.h

*/

pub struct GlobalScopeT {
    // shared_ptr<session_t> session_ptr;
    session_ptr: SessionT

}

impl GlobalScopeT {
    pub fn new() -> Self {
        Self { session_ptr: todo!()  }
    }

    pub fn session(&self) -> &SessionT {
        &self.session_ptr
    }

    fn show_version_info() {
        println!("Ledger ");
    }
}