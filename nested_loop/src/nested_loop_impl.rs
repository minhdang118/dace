use crate::utils::access4addr;
use dace::arybase::set_arybase;
use dace::ast::{LoopBound, Node, Stmt};
use fxhash::FxHashMap;
use hist::Hist;
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use tracing::debug;

static COUNTER: AtomicI64 = AtomicI64::new(0);

pub fn nested_loop(code: &mut Rc<Node>) {
    nested_loop_helper(code, &[]);
}

#[allow(non_snake_case)]
fn nested_loop_helper(code: &Rc<Node>, ivec: &[i32]) {
    match &code.stmt {
        // TODO: Create struct for cache status
        // Update cache status
        Stmt::Ref(ary_ref) => {}
        Stmt::Loop(aloop) => {
            if let LoopBound::Fixed(lb) = aloop.lb {
                if let LoopBound::Fixed(ub) = aloop.ub {
                    (lb..ub).for_each(|i| {
                        aloop.body.iter().for_each(|stmt| {
                            let mut myvec = ivec.to_owned();
                            myvec.push(i);
                            nested_loop_helper(stmt, &myvec)
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
            block.iter().for_each(|stmt| nested_loop_helper(stmt, ivec));
        }
        Stmt::Branch(branch_stmt) => {
            if (branch_stmt.cond)(ivec) {
                nested_loop_helper(&branch_stmt.then_body, ivec)
            } else if let Some(else_body) = &branch_stmt.else_body {
                nested_loop_helper(else_body, ivec)
            }
        }
    }
}
