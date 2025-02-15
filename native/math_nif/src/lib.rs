extern crate rustler;

use std::sync::Mutex;
use rustler::{ResourceArc, NifResult};

rustler::init!("Elixir.MyMath");

// Let's start with defining simple functions

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[rustler::nif]
fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

#[rustler::nif]
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

// Next, lets use a resource that can capture state

struct Adder {
    x: i64
}

#[rustler::resource_impl]
impl rustler::Resource for Adder {}

#[rustler::nif]
fn create_adder(x: i64) -> ResourceArc<Adder> {
    ResourceArc::new(Adder { x })
}

#[rustler::nif]
fn adder_add(adder: ResourceArc<Adder>, y: i64) -> i64 {
    adder.x + y
}

// Finally, lets use a resource with state that we can mutate

struct Counter {
    count: Mutex<i64>
}

#[rustler::resource_impl]
impl rustler::Resource for Counter {}

#[rustler::nif]
fn create_counter() -> ResourceArc<Counter> {
    ResourceArc::new(Counter { count: Mutex::new(0) })
}

#[rustler::nif]
fn increment_counter(counter: ResourceArc<Counter>, amount: i64) -> NifResult<i64> {
    let mut guard = counter.count.lock().map_err(|_| {
        rustler::Error::Term(Box::new("Failed to lock counter mutex"))
    })?;
    *guard += amount;
    Ok(*guard)
}

