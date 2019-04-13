use crate::{
    callbacks::ResultFromCallback, sys, to_result::ToResult, Discord, DiscordResult, Image,
    ImageHandle,
};

/// # Images
impl<'a> Discord<'a> {
    // tested, takes a few seconds, returns the same handle as inputted
    pub fn fetch_image<F>(&mut self, handle: ImageHandle, refresh: bool, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<ImageHandle>) + 'a,
    {
        unsafe {
            ffi!(self.get_image_manager().fetch(handle.into(), refresh)(
                ResultFromCallback::new(callback)
            ))
        }
    }

    // tested, works fine, doubt it has much use
    pub fn image_dimensions(&mut self, handle: ImageHandle) -> DiscordResult<(u32, u32)> {
        let mut dimensions = sys::DiscordImageDimensions::default();

        unsafe {
            ffi!(self
                .get_image_manager()
                .get_dimensions(handle.into(), &mut dimensions,))
        }
        .to_result()?;

        Ok((dimensions.width, dimensions.height))
    }

    // tested, returns linear RGBA V<u8> supposedly
    pub fn image(&mut self, handle: ImageHandle) -> DiscordResult<Image> {
        let (width, height) = self.image_dimensions(handle)?;
        let mut data: Vec<u8> = vec![0; (4 * width * height) as usize];

        unsafe {
            ffi!(self.get_image_manager().get_data(
                handle.into(),
                data[..].as_mut_ptr(),
                data.len() as u32
            ))
        }
        .to_result()?;

        Ok(Image {
            width,
            height,
            data,
        })
    }
}
