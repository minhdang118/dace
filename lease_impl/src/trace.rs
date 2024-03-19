use crate::utils::Cache;
use dace::ast::{LoopBound, Node, Stmt};
use std::collections::HashMap;
use std::rc::Rc;

pub fn trace_lease(code: &mut Rc<Node>) {
    let mut cache = Cache {
        status: HashMap::new(),
        size: 0,
        miss: 0,
    };
    trace_lease_helper(code, &[], &mut cache);
}

#[allow(non_snake_case)]
fn trace_lease_helper(code: &Rc<Node>, ivec: &[i32], cache: &mut Cache) {
    match &code.stmt {
        Stmt::Ref(ary_ref) => {
            cache.access(ary_ref.name.clone(), ary_ref.lease);
        }
        Stmt::Loop(aloop) => {
            if let LoopBound::Fixed(lb) = aloop.lb {
                if let LoopBound::Fixed(ub) = aloop.ub {
                    (lb..ub).for_each(|i| {
                        aloop.body.iter().for_each(|stmt| {
                            let mut myvec = ivec.to_owned();
                            myvec.push(i);
                            trace_lease_helper(stmt, &myvec, cache)
                        })
                    })
                } else {
                    panic!("dynamic loop upper bound is not supported")
                }
            } else {
                panic!("dynamic loop lower bound is not supported")
            }
        }
        Stmt::Block(block) => {
            block
                .iter()
                .for_each(|stmt| trace_lease_helper(stmt, ivec, cache));
        }
        Stmt::Branch(branch_stmt) => {
            if (branch_stmt.cond)(ivec) {
                trace_lease_helper(&branch_stmt.then_body, ivec, cache)
            } else if let Some(else_body) = &branch_stmt.else_body {
                trace_lease_helper(else_body, ivec, cache)
            }
        }
    }
}
