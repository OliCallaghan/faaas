use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use wac_graph::{CompositionGraph, EncodeOptions};

pub mod workflow;

use workflow::extract::ComponentResolver;
use workflow::generate::Generate;
use workflow::primitives::{Primitive, Workflow};
use workflow::register::Register;
