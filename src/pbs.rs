//! Data structures for (de-)serialization as generated by `prost-build`.

/// Data structures generated by prost from simulation.
pub mod simulation {
    include!(concat!(env!("OUT_DIR"), "/viguno.v1.simulation.rs"));
}
