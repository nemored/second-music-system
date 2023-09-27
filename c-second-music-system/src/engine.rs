use super::*;

use std::{
    sync::Arc,
    num::NonZeroUsize,
};

#[no_mangle]
extern "C" fn SMS_Engine_new(
    delegate: *mut Arc<dyn SoundDelegate>,
    speaker_layout: c_int,
    sample_rate: f32,
    num_threads: c_int,
    background_loading: c_int,
) -> *mut Engine {
    if delegate.is_null() {
        panic!("SMS_Engine_new: delegate cannot be NULL!");
    }
    let speaker_layout = speaker_layout_from_int(speaker_layout)
        .expect("SMS_Engine_new: speaker_layout was not a valid \
                 SMS_SPEAKER_LAYOUT_* constant");
    let num_threads = if num_threads <= 0 { None }
    else { Some(NonZeroUsize::new(num_threads as usize).unwrap()) };
    let background_loading = background_loading != 0;
    let delegate = unsafe { delegate.as_ref().unwrap() }.clone();
    Box::into_raw(Box::new(Engine::new(
        delegate, speaker_layout, positive(sample_rate), num_threads, background_loading
    )))
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_free(p: *mut Engine) {
    drop(unsafe { Box::from_raw(p) })
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_clone_commander(engine: *mut Engine) -> *mut Commander {
    if engine.is_null() {
        panic!("SMS_Engine_clone_commander: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_ref().unwrap() };
    Box::into_raw(Box::new(engine.clone_commander()))
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_copy_live_soundtrack(engine: *mut Engine) -> *mut Soundtrack {
    if engine.is_null() {
        panic!("SMS_Engine_copy_live_soundtrack: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_ref().unwrap() };
    Box::into_raw(Box::new(engine.copy_live_soundtrack()))
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_get_speaker_layout(engine: *mut Engine) -> c_int {
    if engine.is_null() {
        panic!("SMS_Engine_get_speaker_layout: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_ref().unwrap() };
    speaker_layout_to_int(engine.get_speaker_layout())
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_get_sample_rate(engine: *mut Engine) -> f32 {
    if engine.is_null() {
        panic!("SMS_Engine_get_speaker_layout: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_ref().unwrap() };
    *engine.get_sample_rate()
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_is_loading_in_background(engine: *mut Engine) -> c_int {
    if engine.is_null() {
        panic!("SMS_Engine_is_loading_in_background: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_ref().unwrap() };
    if engine.is_loading_in_background() { 1 } else { 0 }
}

#[no_mangle]
unsafe extern "C" fn SMS_Engine_turn_handle(
    engine: *mut Engine,
    out: *mut f32,
    out_len: size_t,
) {
    if engine.is_null() {
        panic!("SMS_Engine_turn_handle: engine cannot be NULL!");
    }
    let engine = unsafe { engine.as_mut().unwrap() };
    let out = unsafe { std::slice::from_raw_parts_mut(out, out_len) };
    engine.turn_handle(out);
}
