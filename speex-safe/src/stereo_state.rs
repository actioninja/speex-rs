////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2023.                                                         /
// This Source Code Form is subject to the terms of the Mozilla Public License,/
// v. 2.0. If a copy of the MPL was not distributed with this file, You can    /
// obtain one at http://mozilla.org/MPL/2.0/.                                  /
////////////////////////////////////////////////////////////////////////////////

use speex_sys::SpeexStereoState as SysStereoState;

/// Handling for speex stereo files.
pub struct SpeexStereoState {
    backing: SysStereoState,
}

impl SpeexStereoState {
    /// Creates a new SpeexStereoState.
    pub fn new() -> Self {
        let backing = unsafe {
            let ptr = speex_sys::speex_stereo_state_init();
            *ptr
        };

        Self { backing }
    }

    /// Resets a SpeexStereoState to its original state.
    pub fn reset(&mut self) {
        let ptr = &mut self.backing as *mut SysStereoState;
        unsafe { speex_sys::speex_stereo_state_reset(ptr) }
    }
}

impl Default for SpeexStereoState {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SpeexStereoState {
    fn drop(&mut self) {
        unsafe {
            speex_sys::speex_stereo_state_destroy(&mut self.backing);
        }
    }
}
