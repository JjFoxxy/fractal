use crate::utils::*;
use ode_solvers::dopri5::*;
use ode_solvers::*;
use raster::{Color, Image};
use std::ops::Range;

type State = Vector3<f64>;
type Time = f64;

struct Rossler {
    a: f64,
    b: f64,
    c: f64,
}

impl ode_solvers::System<Time, State> for Rossler {
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        dy[0] = -y[1] - y[2];
        dy[1] = y[0] + self.a * y[1];
        dy[2] = self.b + y[2] * (y[0] - self.c);
    }
}

fn draw_private(
    image: &mut Image,
    trajectory: &Vec<State>,
    range_x: &Range<f64>,
    range_y: &Range<f64>,
) {
    for point in trajectory {
        let point = real_to_image_transform(
            (point.x, point.y),
            (range_x, range_y),
            (image.width, image.height),
        );
        image.set_pixel(
            point.0,
            point.1,
            Color {
                r: (0),
                g: (0),
                b: (0),
                a: (255),
            },
        );
    }
}

pub fn draw_rossler(image: &mut Image, range_x: &Range<f64>, range_y: &Range<f64>) {
    let initial = State::new(1., 1., 1.);

    let system = Rossler {
        a: 0.2,
        b: 0.2,
        c: 5.7,
    };

    let t_start = 0.;
    let t_end = 100.;
    let dt = 1e-3;
    let rtol = 1e-4;
    let atol = 1e-4;
    let mut stepper = Dopri5::new(system, t_start, t_end, dt, initial, rtol, atol);

    let res = stepper.integrate();
    match res {
        Ok(stats) => {
            println!("{stats}");
            draw_private(image, stepper.y_out(), range_x, range_y);
        }
        Err(e) => println!("Integration error {e}"),
    }
}
