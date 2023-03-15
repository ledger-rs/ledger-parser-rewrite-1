use std::collections::HashMap;

/// acount.h

//#[derive(Default, Debug)]
pub struct AccountT<'a> {
    // account_t *                    parent;
    parent: Option<&'a AccountT<'a>>,
    // string                         name;
    name: String,
    // optional<string>               note;
    note: Option<String>,
    // unsigned short                 depth;
    depth: u16,
    // accounts_map                   accounts;
    accounts: HashMap<String, AccountT<'a>>,
    // posts_list                     posts;
    // todo: posts: Vec<PostT>
    // optional<deferred_posts_map_t> deferred_posts;
    // optional<expr_t>               value_expr;

    // mutable string   _fullname;
    full_name: String,
}

impl AccountT<'_> {
    pub fn new() -> Self {
        Self {
            parent: None,
            name: String::new(),
            note: None,
            depth: 0,
            accounts: HashMap::new(),
            full_name: String::new(),
        }
    }
}