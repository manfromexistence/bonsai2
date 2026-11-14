//! Hardware-adaptive frame routing.
//!
//! # The claim under test
//!
//! "The GPU is always faster than the CPU for UI, so put everything on the
//! GPU."
//!
//! Measured, that is false for the majority of real frames. Waking a GPU
//! compute pipeline costs roughly 50–150 µs of submission latency, while
//! re-rasterising a 32×32 dirty rectangle with AVX2 on the CPU costs single
//! digit microseconds. Linebender moved their pure-compute Vello renderer into
//! `research/` and shipped `vello_cpu` + `vello_gpu` for exactly this reason.
//!
//! So the correct architecture is a router, not a commitment:
//!
//! * **CPU micro-patch** — cheap dirty regions, caret blink, hover, typing.
//!   The GPU stays in a low-power state.
//! * **GPU macro-frame** — large dirty areas, 120/240 Hz scrolling, expensive
//!   effects.
//!
//! [`Scheduler::route`] makes that decision from a single scalar.

/// A cost estimate for one frame.
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameCost {
    /// Number of dirty pixels.
    pub dirty_px: u64,
    /// Average number of layers stacked over those pixels.
    pub layer_depth: u32,
    /// Number of animated curves being evaluated.
    pub animated_curves: u64,
}

impl FrameCost {
    /// Builds a cost from a dirty-rect area.
    #[must_use]
    pub fn area(width: u64, height: u64, layer_depth: u32) -> Self {
        Self {
            dirty_px: width * height,
            layer_depth,
            animated_curves: 0,
        }
    }

    /// The scalar used for routing.
    #[must_use]
    pub fn score(&self) -> u64 {
        self.dirty_px
            .saturating_mul(u64::from(self.layer_depth))
            .saturating_add(self.animated_curves.saturating_mul(64))
    }
}

/// Which backend should handle a frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// Rasterise the dirty region on the CPU.
    CpuMicro,
    /// Dispatch a full GPU frame.
    GpuMacro,
}

/// The frame router.
#[derive(Clone, Debug)]
pub struct Scheduler {
    /// Cost below which the CPU handles the frame.
    pub micro_threshold: u64,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self {
            micro_threshold: 16_384,
        }
    }
}

impl Scheduler {
    /// Builds a scheduler with an explicit threshold.
    #[must_use]
    pub fn new(micro_threshold: u64) -> Self {
        Self { micro_threshold }
    }

    /// Chooses a backend for a frame.
    #[must_use]
    pub fn route(&self, cost: &FrameCost) -> Route {
        if cost.score() < self.micro_threshold {
            Route::CpuMicro
        } else {
            Route::GpuMacro
        }
    }
}

/// Counters for what the renderer actually did.
#[derive(Clone, Copy, Debug, Default)]
pub struct Telemetry {
    /// Total frames.
    pub frames: u64,
    /// Frames handled on the CPU.
    pub cpu_frames: u64,
    /// Frames handled on the GPU.
    pub gpu_frames: u64,
    /// Pixels shaded.
    pub pixels_shaded: u64,
    /// Pixels skipped by the occlusion culler.
    pub pixels_culled: u64,
}

impl Telemetry {
    /// Records a routed frame.
    pub fn record(&mut self, route: Route) {
        self.frames += 1;
        match route {
            Route::CpuMicro => self.cpu_frames += 1,
            Route::GpuMacro => self.gpu_frames += 1,
        }
    }

    /// Fraction of frames handled on the CPU.
    #[must_use]
    pub fn cpu_fraction(&self) -> f64 {
        if self.frames == 0 {
            0.0
        } else {
            self.cpu_frames as f64 / self.frames as f64
        }
    }
}
