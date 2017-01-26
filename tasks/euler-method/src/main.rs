fn main() {
    let step_sizes = vec![2.0, 5.0, 10.0];
    // Initial temperature
    const T_0: f32 = 100.0;
    // Room temperature
    const T_R: f32 = 20.0;
    // Starting time
    const T_START: f32 = 0.0;
    // Finish time
    const T_END: f32 = 100.0;
    // Newton law of cooling
    fn cooling(temperature: f32) -> f32 {
        -0.07 * (temperature - T_R)
    };

    for step in step_sizes {
        println!("\n**Current step size {:?}** \n", step);
        euler(&cooling, T_0, T_START, T_END, step);
    }
}

fn euler<F>(func: &F, initial: f32, start_time: f32, end_time: f32, h: f32)
    where F: Fn(f32) -> f32
{
    let mut curr_time = start_time;
    let mut y_n = initial;
    while curr_time <= end_time {
        println!("Current temp {:?} at {:?}", y_n, curr_time);
        y_n = y_n + h * func(y_n);
        curr_time += h;
    }
}
