use crate::gate::{and, not};
use crate::latch;

/// Rising edge triggered D flip-flop
#[derive(Debug)]
pub struct DFlipflop {
    master: latch::DLatch,
    slave: latch::DLatch,
}

impl DFlipflop {
    /// Creates a new D flip-flop in the reset state
    pub fn new() -> Self {
        DFlipflop {
            master: latch::DLatch::new(),
            slave: latch::DLatch::new(),
        }
    }

    /// Updates the flip-flop based on new inputs. The flip-flop triggers on the rising edge of the
    /// clock.
    ///
    /// Note: D must be set to true before the CLK signal changes.
    pub fn update(&mut self, clk: bool, d: bool) {
        self.master.set(not(&clk), d);
        self.slave.set(clk, self.master.q());
    }

    pub fn clear(&mut self) {
        self.master.clear();
        self.slave.clear();
    }

    pub fn q(&self) -> bool {
        self.slave.q()
    }

    pub fn qn(&self) -> bool {
        self.slave.qn()
    }
}

impl Default for DFlipflop {
    fn default() -> Self {
        Self::new()
    }
}

/// Edge-triggered SR flip-flop
#[derive(Clone, Copy)]
pub struct SRFlipflop {
    master: latch::GatedSRLatch,
    slave: latch::GatedSRLatch,
}

impl SRFlipflop {
    /// Creates a new gated SR flip-flop in the reset state
    pub fn new() -> Self {
        SRFlipflop {
            master: latch::GatedSRLatch::new(),
            slave: latch::GatedSRLatch::new(),
        }
    }

    /// Updates the flip-flop based on new inputs. The flip-flop triggers on the rising edge of the
    /// clock.
    pub fn update(&mut self, clk: bool, s: bool, r: bool) {
        self.master.set(s, not(&clk), r);
        self.slave.set(self.master.q(), clk, self.master.qn());
    }

    pub fn q(&self) -> bool {
        self.slave.q()
    }

    pub fn qn(&self) -> bool {
        self.slave.qn()
    }
}

impl Default for SRFlipflop {
    fn default() -> Self {
        Self::new()
    }
}

/// Rising edge triggered JK flip-flop
#[derive(Clone, Copy)]
pub struct JKFlipflop {
    sr_flipflop: SRFlipflop,
}

impl JKFlipflop {
    /// Creates a new JK flip-flop in the reset state
    pub fn new() -> Self {
        JKFlipflop {
            sr_flipflop: SRFlipflop::new(),
        }
    }

    /// Updates the flip-flop based on new inputs. The flip-flop triggers on the rising edge of the
    /// clock.
    pub fn update(&mut self, clk: bool, j: bool, k: bool) {
        let s = and(&[j, self.sr_flipflop.qn()]);
        let r = and(&[k, self.sr_flipflop.q()]);

        self.sr_flipflop.update(clk, s, r);
    }

    pub fn q(&self) -> bool {
        self.sr_flipflop.q()
    }

    pub fn qn(&self) -> bool {
        self.sr_flipflop.qn()
    }
}

impl Default for JKFlipflop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_flipflop() {
        // Reference timing diagram:
        // https://www.build-electronic-circuits.com/wp-content/uploads/2022/11/clock-4.png

        let mut flipflop = DFlipflop::new();

        // Start with Q low
        let mut expect_q = false;
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge and D low
        let mut clk = true;
        let mut d = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Keep the clock high and set D high
        d = true;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge and keep D high
        clk = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge and keep D high
        clk = true;
        flipflop.update(clk, d);
        expect_q = true;
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge and D low
        clk = false;
        d = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge
        clk = true;
        expect_q = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);
    }

    #[test]
    fn test_sr_flipflop() {
        let mut flipflop = SRFlipflop::new();

        // Start with Q low
        let mut expect_q = false;
        assert_eq!(flipflop.q(), expect_q);

        let mut clk = true;
        let mut s = false;
        let mut r = false;

        // Send clock rising edge with S and R low
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge with S and R low
        clk = false;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Set S high but don't toggle clock
        s = true;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge with S high
        clk = true;
        expect_q = true;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge
        clk = false;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Set R high but don't toggle clock
        r = true;
        s = false;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge with R high
        clk = true;
        expect_q = false;
        flipflop.update(clk, s, r);
        assert_eq!(flipflop.q(), expect_q);
    }

    #[test]
    fn test_jk_flipflop() {
        let mut flipflop = JKFlipflop::new();

        // Start with Q low
        let mut expect_q = false;
        assert_eq!(flipflop.q(), expect_q);

        let mut clk = true;
        let mut j = false;
        let mut k = false;

        // Send clock rising edge with J and K low
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge with J and K low
        clk = false;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Set J high but don't toggle clock
        j = true;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge with J high
        clk = true;
        expect_q = true;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge
        clk = false;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Set K high but don't toggle clock
        k = true;
        j = false;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge with K high
        clk = true;
        expect_q = false;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge with J and K high
        clk = false;
        j = true;
        k = true;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge with J and K high
        clk = true;
        expect_q = true;
        flipflop.update(clk, j, k);
        assert_eq!(flipflop.q(), expect_q);
    }
}
