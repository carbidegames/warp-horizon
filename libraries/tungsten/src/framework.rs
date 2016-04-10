use time::PreciseTime;
use EventDispatcher;

pub trait Frontend<M> {
    fn process_events(&mut self, dispatcher: &mut EventDispatcher<M>, model: &mut M);
    fn render(&mut self, model: &M);
}

pub struct UpdateEvent {
    pub delta: f32
}

pub struct Framework<M, F> {
    model: M,
    frontend: F,
    dispatcher: EventDispatcher<M>,
}

impl<M: 'static, F: Frontend<M>> Framework<M, F> {
    pub fn new(model: M, frontend: F, dispatcher: EventDispatcher<M>) -> Self {
        Framework {
            model: model,
            frontend: frontend,
            dispatcher: dispatcher,
        }
    }

    pub fn run<RC: Fn(&M) -> bool>(mut self, run_condition: RC) {
        let mut delta = 0.05;
        while run_condition(&self.model) {
            // Keep track of the start of the frame
            let start = PreciseTime::now();

            // Make the frontend raise any needed frontend events
            self.frontend.process_events(&mut self.dispatcher, &mut self.model);

            // Update the game TODO: Run this at a predictable interval
            self.dispatcher.dispatch(&mut self.model, UpdateEvent { delta: delta });

            // Render the game TODO: Only do this if the world updated
            self.frontend.render(&self.model);

            // Sleep a bit
            // TODO: Only sleep if the world didn't update
            ::std::thread::sleep(::std::time::Duration::from_millis(1));

            // Measure the delta
            let duration = start.to(PreciseTime::now());
            delta = duration.num_microseconds().unwrap() as f32 / 1_000_000.0;
        }
    }
}
