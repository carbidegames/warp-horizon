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
        while run_condition(&self.model) {
            // Make the frontend raise any needed frontend events
            self.frontend.process_events(&mut self.dispatcher, &mut self.model);

            // Update the game
            self.dispatcher.dispatch(&mut self.model, UpdateEvent { delta: 0.16 });

            // Render the game
            self.frontend.render(&self.model);
        }
    }
}
