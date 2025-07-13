use std::os::raw::c_int;
use yengine_core::{Engine, NotInitialized, Initialized};

pub struct EngineHandle {
    _private: [u8; 0],
}

enum EngineState {
    NotInitialized(Engine<NotInitialized>),
    Initialized(Engine<Initialized>),
}

#[no_mangle]
pub extern "C" fn create_engine() -> *mut EngineHandle {
    let engine = Engine::new();
    let state = EngineState::NotInitialized(engine);
    let boxed = Box::new(state);
    Box::into_raw(boxed) as *mut EngineHandle
}

#[no_mangle]
pub extern "C" fn run_engine(handle: *mut EngineHandle) -> c_int {
    if handle.is_null() {
        return 1;
    }

    let state_ptr = handle as *mut EngineState;
    let state = unsafe { &mut *state_ptr };
    
    match state {
        EngineState::NotInitialized(engine) => {
            let engine = std::mem::replace(engine, Engine::new());
            match engine.run() {
                Ok(initialized_engine) => {
                    *state = EngineState::Initialized(initialized_engine);
                    0
                }
                Err(_) => 1,
            }
        }
        EngineState::Initialized(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn shutdown_engine(handle: *mut EngineHandle) {
    if !handle.is_null() {
        let state_ptr = handle as *mut EngineState;
        unsafe {
            let is_initialized = match &*state_ptr {
                EngineState::Initialized(_) => true,
                _ => false,
            };
            
            if is_initialized {
                if let EngineState::Initialized(engine) = *Box::from_raw(state_ptr) {
                    engine.shutdown();
                }
            } else {
                let _ = Box::from_raw(state_ptr);
            }
        }
    }
}
