use std::collections::HashMap;

/// acount.h

struct AccountT<'a> {
    // account_t *                    parent;
    parent: &'a AccountT<'a>,
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
