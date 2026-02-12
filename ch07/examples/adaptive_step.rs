use ode_solvers::dopri5::*;
use ode_solvers::*;

type State = Vector2<f64>;

struct Oscillator;

impl ode_solvers::System<f64, State> for Oscillator {
    fn system(&self, _t: f64, y: &State, dy: &mut State) {
        // 単振動: dx/dt = v, dv/dt = -x
        dy[0] = y[1];
        dy[1] = -y[0];
    }
}

fn main() {
    let system = Oscillator;
    let y0 = State::new(1.0, 0.0);
    let (t_start, t_end) = (0.0, 10.0);

    // Dormand-Prince 5(4) 法を使用
    let mut stepper = Dopri5::new(system, t_start, t_end, 0.1, y0, 1.0e-8, 1.0e-8);
    let res = stepper.integrate();

    if let Ok(stats) = res {
        println!(
            "Integration finished. Total steps: {}",
            stats.accepted_steps
        );
        let values = stepper.y_out();
        println!("Final state: {:?}", values.last().unwrap());
    }
}
