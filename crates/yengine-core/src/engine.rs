pub struct NotInitialized;
pub struct Initialized;

pub struct Engine<S> {
    state: std::marker::PhantomData<S>,
}

impl Default for Engine<NotInitialized> {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine<NotInitialized> {
    pub fn new() -> Self {
        Self { state: std::marker::PhantomData }
    }

    pub fn run(self) -> Result<Engine<Initialized>, String> {
        let engine = Engine {
            state: std::marker::PhantomData,
        };

        Ok(engine)
    }
}

impl Engine<Initialized> {
    pub fn shutdown(self) {
    }
}
