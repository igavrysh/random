// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
struct Luggage<State> {
    id: String,
    state: State
}

impl<State> Luggage<State> {
    fn transition<NewState>(self, new_state: NewState) -> Luggage<NewState> {
        Luggage {
            id: self.id,
            state: new_state,
        }
    }
}

#[derive(Debug)]
struct CheckIn;

#[derive(Debug)]
struct OnLoad;

#[derive(Debug)]
struct Offload;

#[derive(Debug)]
struct AwaitingPickup;

#[derive(Debug)]
struct EndCustody;

impl Luggage<CheckIn> {
    fn new(id: String) -> Self {
        Self {
            id: id,
            state: CheckIn,
        }
    }

    fn checkin(self) -> Luggage<OnLoad> {
        self.transition(OnLoad)
    } 
}

impl Luggage<OnLoad> {
    fn onload(self) -> Luggage<Offload> {
        self.transition(Offload)
    }
}

impl Luggage<Offload> {
    fn offload(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn awaiting_pickup(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}

fn main() {
    let luggage = Luggage::new("abc123".to_owned())
        .checkin()
        .onload()
        .offload()
        .awaiting_pickup();

    println!("{:?}", luggage);
    println!("Luggage state: {:?}", luggage.state);
}
