#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileKind {
    Classic,
    NoLongBreak,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub kind: ProfileKind,
    pub work_secs: u64,
    pub short_break_secs: u64,
    pub long_break_secs: u64,
    pub cycles_before_long: u32,
}

pub struct TimerState {
    pub phase: Phase,
    pub remaining_secs: u64,
    pub running: bool,
    pub cycle_count: u32,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            phase: Phase::Work,
            remaining_secs: 0,
            running: false,
            cycle_count: 0,
        }
    }

    pub fn tick(&mut self, profile: &Profile) {
        if !self.running {
            return;
        }

        if self.remaining_secs > 0 {
            self.remaining_secs -= 1;
        }

        if self.remaining_secs == 0 {
            if self.phase == Phase::Work {
                self.cycle_count += 1;
            } else if self.phase == Phase::LongBreak {
                self.cycle_count = 0;
            }

            self.phase = Self::next_phase(self.phase, self.cycle_count, profile);
            self.remaining_secs = self.phase_duration(self.phase, profile);
            self.running = false;
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self, profile: &Profile) {
        self.phase = Phase::Work;
        self.cycle_count = 0;
        self.remaining_secs = self.phase_duration(self.phase, profile);
        self.running = false;
    }

    pub fn skip(&mut self, profile: &Profile) {
        if self.phase == Phase::Work {
            self.cycle_count += 1;
        } else if self.phase == Phase::LongBreak {
            self.cycle_count = 0;
        }

        self.phase = Self::next_phase(self.phase, self.cycle_count, profile);
        self.remaining_secs = self.phase_duration(self.phase, profile);
        self.running = false;
    }

    pub fn next_phase(current: Phase, cycle_count: u32, profile: &Profile) -> Phase {
        match current {
            Phase::Work => {
                if profile.kind == ProfileKind::Classic && cycle_count >= profile.cycles_before_long {
                    Phase::LongBreak
                } else {
                    Phase::ShortBreak
                }
            }
            Phase::ShortBreak | Phase::LongBreak => Phase::Work,
        }
    }

    fn phase_duration(&self, phase: Phase, profile: &Profile) -> u64 {
        match phase {
            Phase::Work => profile.work_secs,
            Phase::ShortBreak => profile.short_break_secs,
            Phase::LongBreak => profile.long_break_secs,
        }
    }
}
