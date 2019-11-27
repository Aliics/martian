//! The lifeblood of `martian`, the true decision maker and work horse of this
//! crate. This is majorly focused on how you handle requests made to your
//! service. Built to hopefully be easy to use, but configurable if you are
//! into pumping out the most performance you possibly can out of a thread.

#[cfg(test)]
mod tests;
