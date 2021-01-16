/*
    dynamic virtual table
    should be managed by GC
*/

struct Table {
    // before calling selector
    // after calling selector
    selector: usize,      // vm closure capturing self
    state: usize,         // containing normal fields
    virtual_table: usize, // hash table contains type args and rets
}

impl Table {}
