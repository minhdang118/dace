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

