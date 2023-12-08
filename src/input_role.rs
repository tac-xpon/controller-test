#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum InputRole {
    Button0 = 0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
    Button10,
    Button11,
    Up,
    Right,
    Down,
    Left,
    Up2,
    Right2,
    Down2,
    Left2,
    _EndOfVariants
}

#[allow(dead_code)]
impl InputRole {
    #[allow(non_upper_case_globals)]
    pub const Whole: Self = Self::_EndOfVariants;
}

use std::collections::BTreeMap;
pub struct InputRoleMap<T> {
    pub input_map: BTreeMap<T, (bool, Vec<InputRole>)>,
}

impl<T: Ord + Copy> InputRoleMap<T> {
    pub fn new() -> Self {
        Self {
            input_map: BTreeMap::new()
        }
    }

    pub fn assign(&mut self, set_list: &[(T, InputRole)]) {
        for key_set in set_list {
            if let Some((_, role_list)) = self.input_map.get_mut(&key_set.0) {
                role_list.push(key_set.1);
            } else {
                self.input_map.insert(key_set.0, (false, vec![key_set.1]));
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct InputRoleState {
    state_and_history: [(bool, u32); InputRole::_EndOfVariants as usize],
}

#[allow(dead_code)]
impl InputRoleState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_all(&mut self) {
        for s_and_h in &mut self.state_and_history {
            *s_and_h = (false, 0);
        }
    }

    pub fn clear_state(&mut self) {
        for s_and_h in &mut self.state_and_history {
            s_and_h.0 = false;
        }
    }

    pub fn update_state<T: Ord>(&mut self, map: &InputRoleMap<T>) {
        for (_, (state, list)) in map.input_map.iter() {
            if *state {
                for r in list {
                    self.set_true(*r);
                }
            }
        }

    }
    pub fn clear_history(&mut self) {
        for s_and_h in &mut self.state_and_history {
            s_and_h.1 = 0;
        }
    }

    pub fn update_history(&mut self) {
        for s_and_h in &mut self.state_and_history {
            s_and_h.1 = (s_and_h.1 << 1) + if s_and_h.0 { 1 } else { 0 };
        }
    }

    pub fn get(&self, input_role: InputRole) -> (bool, u32) {
        if input_role != InputRole::Whole {
            self.state_and_history[input_role as usize]
        } else {
            let (mut whole_state, mut whole_histroy) = (false, 0);
            for s_and_h in self.state_and_history {
                whole_state |= s_and_h.0;
                whole_histroy |= s_and_h.1;
            }
            (whole_state, whole_histroy)
        }
    }

    pub fn set(&mut self, input_role: InputRole, state: bool) {
        self.state_and_history[input_role as usize].0 = state;
    }

    pub fn set_true(&mut self, input_role: InputRole) {
        self.set(input_role, true);
    }

    pub fn set_false(&mut self, input_role: InputRole) {
        self.set(input_role, false);
    }
}
