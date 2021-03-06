#[link(name = "issue_2526",
       vers = "0.2",
       uuid = "54cc1bc9-02b8-447c-a227-75ebc923bc29")];
#[crate_type = "lib"];

use std;

export context;

struct arc_destruct<T:Const> {
  _data: int,
  drop {}
}

fn arc_destruct<T: Const>(data: int) -> arc_destruct<T> {
    arc_destruct {
        _data: data
    }
}

fn arc<T: Const>(_data: T) -> arc_destruct<T> {
    arc_destruct(0)
}

fn init() -> arc_destruct<context_res> unsafe {
    arc(context_res())
}

struct context_res {
    ctx : int,

    drop { }
}

fn context_res() -> context_res {
    context_res {
        ctx: 0
    }
}

type context = arc_destruct<context_res>;

impl context {
    fn socket() { }
}
