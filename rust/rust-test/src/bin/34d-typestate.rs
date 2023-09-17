struct Employee<State> {
    name: String,
    state: State,
}

impl <State> Employee<State> {
    fn transition<NextState>(self, state: NextState) -> Employee<NextState> {
        Employee { name: self.name, state: state }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct FailedTraining {
    score: u8,
}
struct OnboardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }

    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnboardingComplete>, Employee<FailedTraining>> {
        if score >= 7 {
            Ok(self.transition(OnboardingComplete { score }))
        } else {
            Err(self.transition(FailedTraining { score }))
        }
    }
}

fn main() {
    let employee = Employee::new("Sara");
    let onboarded = employee
        .read_agreement()
        .sign()
        .train(8);
    match  onboarded {
        Ok(_) => println!("onboarding complete"),
        Err(emp) => println!("training failed, scopre: {}", emp.state.score)
    }
}