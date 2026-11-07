//! `--timings` (SR-CLI-MCP decision 2; SR-RULE-AUTHORING decision 3):
//! where a run spent its time — each phase, with how often it ran, and each
//! rule. Off by default; when off nothing is measured.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// The phases of a run (SR-ARCHITECTURE decision 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Compile,
    Discover,
    Read,
    Parse,
    Semantic,
    Rules,
    Suppress,
    /// The formatter engine, wherever it runs: `format`, the format step
    /// of `fix`, and `check`'s `format/unformatted`.
    Format,
    /// The project stage (SR-ARCHITECTURE decision 4.4; W-168), each part
    /// apart. `graph-read`: the root walked and every file read for the graph.
    GraphRead,
    /// Every file's graph facts, wall time: the three below, in parallel.
    GraphFacts,
    /// Facts extracted from a file the cache did not have.
    GraphExtract,
    /// The facts cache asked, hit or miss.
    GraphCacheRead,
    /// Facts written to the cache.
    GraphCacheWrite,
    /// The file set and manifests indexed for the resolver.
    GraphIndex,
    /// Import specifiers resolved to files; counted per call.
    GraphResolve,
    /// Uses, re-exports and entry points linked, the resolving left out.
    GraphLink,
    /// The project rules run on the linked graph.
    GraphRules,
}

impl Phase {
    pub const ALL: [Phase; 17] = [
        Phase::Compile,
        Phase::Discover,
        Phase::Read,
        Phase::Parse,
        Phase::Semantic,
        Phase::Rules,
        Phase::Suppress,
        Phase::Format,
        Phase::GraphRead,
        Phase::GraphFacts,
        Phase::GraphExtract,
        Phase::GraphCacheRead,
        Phase::GraphCacheWrite,
        Phase::GraphIndex,
        Phase::GraphResolve,
        Phase::GraphLink,
        Phase::GraphRules,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Phase::Compile => "compile",
            Phase::Discover => "discover",
            Phase::Read => "read",
            Phase::Parse => "parse",
            Phase::Semantic => "semantic",
            Phase::Rules => "rules",
            Phase::Suppress => "suppress",
            Phase::Format => "format",
            Phase::GraphRead => "graph-read",
            Phase::GraphFacts => "graph-facts",
            Phase::GraphExtract => "graph-extract",
            Phase::GraphCacheRead => "graph-cache-read",
            Phase::GraphCacheWrite => "graph-cache-write",
            Phase::GraphIndex => "graph-index",
            Phase::GraphResolve => "graph-resolve",
            Phase::GraphLink => "graph-link",
            Phase::GraphRules => "graph-rules",
        }
    }
}

#[derive(Default)]
struct Clock {
    nanos: AtomicU64,
    count: AtomicU64,
}

impl Clock {
    fn add(&self, nanos: u64) {
        self.add_n(nanos, 1);
    }

    fn add_n(&self, nanos: u64, count: u64) {
        self.nanos.fetch_add(nanos, Ordering::Relaxed);
        self.count.fetch_add(count, Ordering::Relaxed);
    }
}

/// The counters of one run, shared by every worker thread.
pub struct Timings {
    started: Instant,
    phases: [Clock; 17],
    rules: Vec<Clock>,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

/// Adds the time until it is dropped to one phase.
pub(crate) struct Guard<'a> {
    clock: &'a Clock,
    start: Instant,
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.clock.add(self.start.elapsed().as_nanos() as u64);
    }
}

impl Timings {
    pub(crate) fn new(rules: usize) -> Timings {
        Timings {
            started: Instant::now(),
            phases: Default::default(),
            rules: (0..rules).map(|_| Clock::default()).collect(),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }

    pub(crate) fn guard(&self, phase: Phase) -> Guard<'_> {
        Guard { clock: &self.phases[phase as usize], start: Instant::now() }
    }

    pub(crate) fn add(&self, phase: Phase, since: Instant) {
        self.phases[phase as usize].add(since.elapsed().as_nanos() as u64);
    }

    /// `nanos` measured elsewhere, over `count` runs of the phase.
    pub(crate) fn add_nanos(&self, phase: Phase, nanos: u64, count: u64) {
        self.phases[phase as usize].add_n(nanos, count);
    }

    pub(crate) fn rule(&self, index: usize, since: Instant) {
        if let Some(c) = self.rules.get(index) {
            c.add(since.elapsed().as_nanos() as u64);
        }
    }

    pub(crate) fn cache(&self, hit: bool) {
        if hit { &self.cache_hits } else { &self.cache_misses }.fetch_add(1, Ordering::Relaxed);
    }
}

/// One line of the phase table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhaseTiming {
    pub phase: &'static str,
    /// Summed over worker threads.
    pub nanos: u64,
    pub count: u64,
}

/// One rule's time, summed over files and threads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleTiming {
    pub rule: String,
    pub nanos: u64,
    /// Node visits (and fix computations) the rule was timed for.
    pub calls: u64,
}

/// What `--timings` prints.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimingReport {
    /// From loading the project to the report.
    pub wall_nanos: u64,
    pub threads: usize,
    pub phases: Vec<PhaseTiming>,
    /// Slowest first; rules that never ran are left out.
    pub rules: Vec<RuleTiming>,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl crate::Project {
    /// The timings so far, when the project was loaded with `timings` on.
    pub fn timings(&self) -> Option<TimingReport> {
        let t = self.timings.as_ref()?;
        let phases = Phase::ALL
            .iter()
            .map(|p| {
                let c = &t.phases[*p as usize];
                PhaseTiming { phase: p.name(), nanos: c.nanos.load(Ordering::Relaxed), count: c.count.load(Ordering::Relaxed) }
            })
            .collect();
        let mut rules: Vec<RuleTiming> = self
            .compiled
            .iter()
            .zip(&t.rules)
            .map(|(r, c)| RuleTiming { rule: r.id().to_owned(), nanos: c.nanos.load(Ordering::Relaxed), calls: c.count.load(Ordering::Relaxed) })
            .filter(|r| r.calls > 0)
            .collect();
        rules.sort_by(|a, b| b.nanos.cmp(&a.nanos).then_with(|| a.rule.cmp(&b.rule)));
        Some(TimingReport {
            wall_nanos: t.started.elapsed().as_nanos() as u64,
            threads: self.pool.current_num_threads(),
            phases,
            rules,
            cache_hits: t.cache_hits.load(Ordering::Relaxed),
            cache_misses: t.cache_misses.load(Ordering::Relaxed),
        })
    }
}
