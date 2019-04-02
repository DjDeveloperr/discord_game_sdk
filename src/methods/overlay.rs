use crate::activity::Action;
use crate::event::OverlayEvent;
use crate::prelude::*;

/// # Overlay
impl Discord {
    pub fn is_overlay_enabled(&self) -> Result<bool> {
        let mut enabled = false;

        ffi!(self
            .get_overlay_manager()
            .is_enabled(&mut enabled as *mut _))?;

        Ok(enabled)
    }

    pub fn is_overlay_opened(&self) -> Result<bool> {
        let mut opened = false;

        ffi!(self.get_overlay_manager().is_locked(&mut opened as *mut _))?;

        Ok(opened)
    }

    pub fn set_overlay_opened<F>(&self, opened: bool, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().set_locked(
            opened,
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn open_invite_overlay<F>(&self, action: Action, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().open_activity_invite(
            action.to_sys(),
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn open_guild_invite_overlay<S, F>(&self, code: S, mut callback: F)
    where
        S: AsRef<str>,
        F: FnMut(Result<()>),
    {
        let _ = std::ffi::CString::new(code.as_ref())
            .map_err(DeveloperViolation::from)
            .map_err(Error::from)
            .and_then(|cstring| {
                ffi!(self.get_overlay_manager().open_guild_invite(
                    cstring.as_ptr(),
                    &mut callback as *mut _ as *mut _,
                    Some(simple_callback::<F>)
                ))
            })
            .map_err(|e| callback(Err(e)));
    }

    pub fn open_voice_settings<F>(&self, mut callback: F)
    where
        F: FnMut(Result<()>),
    {
        let _ = ffi!(self.get_overlay_manager().open_voice_settings(
            &mut callback as *mut _ as *mut _,
            Some(simple_callback::<F>)
        ))
        .map_err(|e| callback(Err(e)));
    }

    pub fn overlay_events_reader(&mut self) -> shrev::ReaderId<OverlayEvent> {
        self.overlay_events.register_reader()
    }

    pub fn overlay_events(
        &self,
        reader: &mut shrev::ReaderId<OverlayEvent>,
    ) -> shrev::EventIterator<OverlayEvent> {
        self.overlay_events.read(reader)
    }
}