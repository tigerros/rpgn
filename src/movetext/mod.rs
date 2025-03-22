use pgn_reader::Skip;
use shakmaty::san::SanPlus;

dry_mods::mods! {
    mod pub use sans, variation;
}

/// The trait for making a movetext in the [`Pgn`](crate::Pgn) using the structure of the [`pgn_reader::Visitor`].
///
/// Implementors are [`Sans`], [`Variation`] and `()`, if you would like to ignore the movetext.
/// You can also implement this yourself.
pub trait Movetext: Default {
    /// This is what is actually implementing the core functionality.
    /// The type implementing the [`Movetext`] trait is just what the [`Movetext::Agent`] outputs.
    /// That way the agent can have extra state that isn't necessary in the final output.
    type Agent;

    fn begin_game() -> Self::Agent;
    /// Skips by default.
    fn begin_variation(_agent: &mut Self::Agent) -> Skip {
        Skip(true)
    }
    /// Does nothing by default.
    fn end_variation(_agent: &mut Self::Agent) {}
    fn san(agent: &mut Self::Agent, san: SanPlus);
    // fn foo(self) -> Self { self } is actually a noop on -Copt-level=3.
    // I think it should apply to traits too, since they're just sugar.
    fn end_game(agent: Self::Agent) -> Self;
}

impl Movetext for () {
    type Agent = ();

    fn begin_game() -> Self::Agent {}
    fn san((): &mut Self::Agent, _: SanPlus) {}
    fn end_game((): Self::Agent) {}
}