//! Closed-form spring animation.
//!
//! # The claim under test
//!
//! "Animations must be ticked by the CPU, so an idle window still burns CPU and
//! a GPU-resident engine needs the CPU to keep time."
//!
//! Partly false. A damped harmonic oscillator has a **closed-form solution**:
//!
//! ```text
//! x(t) = target + e^(-ζω₀t) · (A·cos(ω_d·t) + B·sin(ω_d·t))
//! ```
//!
//! so any consumer can evaluate the animation at an arbitrary timestamp
//! without integrating it. That is what lets a GPU-side scheduler know exactly
//! when the last spring will settle ([`Spring::settle_time`]) and sleep
//! instead of spinning at 240 Hz.
//!
//! The remaining truth: *events* (mouse, keyboard) still arrive through the
//! operating system, so the CPU is never fully out of the loop. What changes is
//! that the CPU stops being a *frame ticker*.

/// A damped harmonic oscillator with an analytic solution.
#[derive(Clone, Debug)]
pub struct Spring {
    /// Current position.
    pub value: f32,
    /// Current velocity.
    pub velocity: f32,
    /// Target position.
    pub target: f32,
    /// Spring constant.
    pub stiffness: f32,
    /// Damping coefficient.
    pub damping: f32,
    /// Mass.
    pub mass: f32,
    /// Distance below which the spring counts as settled.
    pub rest_displacement: f32,
    /// Speed below which the spring counts as settled.
    pub rest_velocity: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            value: 0.0,
            velocity: 0.0,
            target: 0.0,
            stiffness: 180.0,
            damping: 20.0,
            mass: 1.0,
            rest_displacement: 0.001,
            rest_velocity: 0.001,
        }
    }
}

impl Spring {
    /// Creates a spring at `value` with default constants.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    /// Undamped angular frequency.
    #[must_use]
    pub fn omega_0(&self) -> f32 {
        (self.stiffness / self.mass).max(0.0).sqrt()
    }

    /// Damping ratio. `< 1` under-damped, `== 1` critical, `> 1` over-damped.
    #[must_use]
    pub fn zeta(&self) -> f32 {
        let m = self.mass.max(f32::EPSILON);
        self.damping / (2.0 * (self.stiffness * m).max(f32::EPSILON).sqrt())
    }

    /// Advances the spring by `dt` seconds using the closed-form solution.
    ///
    /// This is an *exact* step, not an integration: the result is identical
    /// whether you call `step(0.1)` once or `step(0.01)` ten times (up to
    /// floating-point rounding), because each call re-derives the solution
    /// from the current state.
    pub fn step(&mut self, dt: f32) -> f32 {
        if dt <= 0.0 {
            return self.value;
        }
        let w0 = self.omega_0();
        if w0 <= 0.0 {
            self.value = self.target;
            self.velocity = 0.0;
            return self.value;
        }
        let zeta = self.zeta();
        let x0 = self.value - self.target;
        let v0 = self.velocity;

        let (x, v) = if zeta < 1.0 {
            let wd = w0 * (1.0 - zeta * zeta).sqrt();
            let a = x0;
            let b = (v0 + zeta * w0 * x0) / wd;
            let decay = (-zeta * w0 * dt).exp();
            let (s, c) = (wd * dt).sin_cos();
            let env = a * c + b * s;
            (
                self.target + decay * env,
                decay * (-zeta * w0 * env + (-a * wd * s + b * wd * c)),
            )
        } else if (zeta - 1.0).abs() < 1e-6 {
            let a = x0;
            let b = v0 + w0 * x0;
            let decay = (-w0 * dt).exp();
            let inner = a + b * dt;
            (
                self.target + decay * inner,
                decay * (b - w0 * inner),
            )
        } else {
            let root = (zeta * zeta - 1.0).sqrt();
            let r1 = -w0 * (zeta - root);
            let r2 = -w0 * (zeta + root);
            let c1 = (v0 - r2 * x0) / (r1 - r2);
            let c2 = x0 - c1;
            (
                self.target + c1 * (r1 * dt).exp() + c2 * (r2 * dt).exp(),
                c1 * r1 * (r1 * dt).exp() + c2 * r2 * (r2 * dt).exp(),
            )
        };

        self.value = x;
        self.velocity = v;
        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
        }
        self.value
    }

    /// Whether the spring has come to rest within the configured thresholds.
    #[must_use]
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() <= self.rest_displacement
            && self.velocity.abs() <= self.rest_velocity
    }

    /// How long until the spring is within `epsilon` of the target.
    ///
    /// For an under-damped spring the envelope is `A·e^(-ζω₀t)`, so this is
    /// `ln(A/ε) / (ζω₀)`. A GPU scheduler can use this to decide when to stop
    /// dispatching frames without any per-frame bookkeeping.
    ///
    /// Returns `0.0` if the spring is already settled, and `f32::INFINITY`
    /// if it never settles (a critically or over-damped spring settles
    /// exponentially, so it always settles; only a zero-stiffness spring does
    /// not).
    #[must_use]
    pub fn settle_time(&self, epsilon: f32) -> f32 {
        if self.is_settled() {
            return 0.0;
        }
        let w0 = self.omega_0();
        if w0 <= 0.0 {
            return f32::INFINITY;
        }
        let zeta = self.zeta();
        let amplitude = (self.value - self.target).abs().max(self.velocity.abs() / w0).max(epsilon);
        if amplitude <= epsilon {
            return 0.0;
        }
        if zeta <= 0.0 {
            return f32::INFINITY;
        }
        (amplitude / epsilon).ln() / (zeta * w0)
    }
}
