struct Factory {
    production_per_hour: f32,
    low_speed_efficienty: f32,
    med_speed_efficienty: f32,
    high_speed_efficienty: f32,
    speed_breakpoint_exclude: (f32, f32),
}

impl Factory {

    fn production_rate_per_hour(&self, actual_speed: f32) -> f64 {
        if actual_speed > self.speed_breakpoint_exclude.0
        { (self.med_speed_efficienty * self.production_per_hour * actual_speed).into() }
        else if actual_speed > self.speed_breakpoint_exclude.1
            { (self.high_speed_efficienty * self.production_per_hour * actual_speed).into() }
        else
            { (self.low_speed_efficienty * self.production_per_hour * actual_speed).into() }
    }

    fn production_rate_per_minute(production_rate_per_hour: f64) -> u32 {
        (production_rate_per_hour / 60.0) as u32
    }
}

fn main() {
    let my_factory: Factory = Factory {
        production_per_hour: 221.0,
        low_speed_efficienty: 1.0,
        med_speed_efficienty: 0.9,
        high_speed_efficienty: 0.77,
        speed_breakpoint_exclude: (4.0, 8.0),
    };
    println!("the factory produce {:.2} car per hour", Factory::production_rate_per_hour(&my_factory, 6.0));
    println!("the factory produce {} car per minute", Factory::production_rate_per_minute(Factory::production_rate_per_hour(&my_factory, 6.0)))
}
