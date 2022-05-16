use core::fmt::Display;

pub struct FSM<'a> {
    pub states: Vec<i8>,
    pub initial_state: i8,
    pub accepting_states: Vec<i8>,
    pub next_state: &'a dyn Fn(i8, char) -> i8,
}

#[derive(Debug)]
pub struct RunResult {
    pub recognized: bool,
    pub value: String,
    pub state: i8,
}

pub const NO_NEXT_STATE: i8 = -1;

impl Display for FSM<'_> { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?} {} {:?}", self.states, self.initial_state, self.accepting_states)
    }
}

impl FSM<'_> {
    pub(crate) fn run(&self, input: &str) -> RunResult {
        let mut current_state = self.initial_state;
        let mut length: usize = 0;

        for (i, s) in input.chars().enumerate() {
            let next_state = (self.next_state)(current_state, s);

            if next_state == NO_NEXT_STATE {
                break;
            }

            current_state = next_state;
            length = i;
        }

        return RunResult {
            recognized: self.accepting_states.iter().any(|&i| i == current_state),
            value: input[0..length + 1].to_string(),
            state: current_state,
        };
    }
}
