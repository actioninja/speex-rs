use crate::SpeexBits;
use speex_sys::{SpeexMode, SPEEX_MODEID_NB, SPEEX_MODEID_UWB, SPEEX_MODEID_WB};
use std::ffi::c_void;
use std::marker::{PhantomData, PhantomPinned};

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ModeId {
    NarrowBand = SPEEX_MODEID_NB,
    WideBand = SPEEX_MODEID_WB,
    UltraWideBand = SPEEX_MODEID_UWB,
}

impl ModeId {
    pub fn get_mode(self) -> &'static SpeexMode {
        unsafe {
            let ptr = speex_sys::speex_lib_get_mode(self as i32);
            // speexmodes are hard constants defined within the codebase itself, so the backing
            // memory *should* always be valid. Should.
            let reference: &'static SpeexMode = &*ptr;
            reference
        }
    }
}

#[repr(C)]
pub struct SpeexEncoderHandle {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

impl SpeexEncoderHandle {
    pub fn create(mode: &SpeexMode) -> *mut Self {
        let ptr = unsafe {
            let mode_ptr = mode as *const SpeexMode;
            speex_sys::speex_encoder_init(mode_ptr)
        };
        ptr as *mut SpeexEncoderHandle
    }

    pub fn destroy(handle: *mut SpeexEncoderHandle) {
        unsafe { speex_sys::speex_encoder_destroy(handle as *mut c_void) }
    }
}

pub struct SpeexEncoder {
    encoder_handle: *mut SpeexEncoderHandle,
    mode: &'static SpeexMode,
}

impl SpeexEncoder {
    pub fn new(mode: ModeId) -> Self {
        let mode = mode.get_mode();
        let encoder_handle = SpeexEncoderHandle::create(mode);
        Self {
            encoder_handle,
            mode,
        }
    }
}

impl Drop for SpeexEncoder {
    fn drop(&mut self) {
        SpeexEncoderHandle::destroy(self.encoder_handle);
    }
}

#[repr(C)]
pub struct SpeexDecoderHandle {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

impl SpeexDecoderHandle {
    pub fn create(mode: &SpeexMode) -> *mut Self {
        let ptr = unsafe {
            let mode_ptr = mode as *const SpeexMode;
            speex_sys::speex_decoder_init(mode_ptr)
        };
        ptr as *mut SpeexDecoderHandle
    }

    pub fn destroy(handle: *mut SpeexDecoderHandle) {
        unsafe {
            speex_sys::speex_decoder_destroy(handle as *mut c_void);
        }
    }
}

pub struct SpeexDecoder {
    encoder_handle: *mut SpeexDecoderHandle,
    mode: &'static SpeexMode,
}

impl SpeexDecoder {
    pub fn new(mode: ModeId) -> Self {
        let mode = mode.get_mode();
        let encoder_handle = SpeexDecoderHandle::create(mode);
        Self {
            encoder_handle,
            mode,
        }
    }
}

impl Drop for SpeexDecoder {
    fn drop(&mut self) {
        SpeexDecoderHandle::destroy(self.encoder_handle);
    }
}
