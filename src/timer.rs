#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseTransition {
    WorkEnded,
    ShortBreakEnded,
    LongBreakEnded,
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

    pub fn update(&mut self, dt_secs: u64, profile: &Profile) -> Option<PhaseTransition> {
        if !self.running {
            return None;
        }

        if self.remaining_secs > dt_secs {
            self.remaining_secs -= dt_secs;
            None
        } else {
            self.remaining_secs = 0;
            self.running = false;
            
            match self.phase {
                Phase::Work => Some(PhaseTransition::WorkEnded),
                Phase::ShortBreak => Some(PhaseTransition::ShortBreakEnded),
                Phase::LongBreak => Some(PhaseTransition::LongBreakEnded),
            }
        }
    }

    pub fn advance_phase(&mut self, profile: &Profile) {
        let is_long_break = self.phase == Phase::LongBreak;
        let is_short_break_no_long = self.phase == Phase::ShortBreak && profile.kind == ProfileKind::NoLongBreak;
        
        if self.phase == Phase::Work {
            self.cycle_count += 1;
        } else if is_long_break || is_short_break_no_long {
            self.cycle_count = 0;
        }

        self.phase = Self::next_phase(self.phase, self.cycle_count, profile);
        self.remaining_secs = self.phase_duration(self.phase, profile);
        
        // Timer stops after the last phase (enters Idle/paused state)
        if is_long_break || is_short_break_no_long {
            self.running = false;
        } else {
            self.running = true; // Auto-start next phase
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

#[cfg(test)]
mod tests {
    use super::*;

    fn classic_profile() -> Profile {
        Profile {
            kind: ProfileKind::Classic,
            work_secs: 3,        // 3 segons per testar ràpid
            short_break_secs: 2,
            long_break_secs: 4,
            cycles_before_long: 2,
        }
    }

    #[test]
    fn timer_counts_down() {
        let mut t = TimerState::new();
        t.reset(&classic_profile());
        t.running = true;
        t.update(1, &classic_profile());
        assert_eq!(t.remaining_secs, 2);
    }

    #[test]
    fn work_ends_and_transitions() {
        let mut t = TimerState::new();
        t.reset(&classic_profile());
        t.running = true;
        let result = t.update(10, &classic_profile()); // més que els 3s de treball
        assert_eq!(result, Some(PhaseTransition::WorkEnded));
    }

    #[test]
    fn advance_phase_work_to_short_break() {
        let p = classic_profile();
        let mut t = TimerState::new();
        t.reset(&p);
        t.update(10, &p);
        t.advance_phase(&p);
        assert_eq!(t.phase, Phase::ShortBreak);
        assert!(t.running);
    }

    #[test]
    fn cycle_resets_after_long_break() {
        let p = classic_profile();
        let mut t = TimerState::new();
        t.reset(&p);
        t.running = true;
        
        // Simulem fins arribar a LongBreak
        t.update(10, &p); t.advance_phase(&p); // Work -> ShortBreak
        t.update(10, &p); t.advance_phase(&p); // ShortBreak -> Work
        t.update(10, &p); t.advance_phase(&p); // Work -> LongBreak
        
        assert_eq!(t.phase, Phase::LongBreak);
        
        t.update(10, &p); t.advance_phase(&p); // LongBreak -> Work
        assert_eq!(t.cycle_count, 0);
        assert!(!t.running); // s'atura després del descans llarg
    }
}
