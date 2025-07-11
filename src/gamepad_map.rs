//! Provides proper mappings for controllers that don't have them
//!
//! also includes gilrs helper things

/// REV's PS4-like controller map
pub static REV_CONTROLLER_CUSTOM_SDL_MAPPING_LINUX: &str = "03000000120c0000182e000011010000,REV Robotics FTC Controller,a:b1,b:b2,x:b0,y:b3,back:b8,guide:b12,start:b9,leftstick:b10,rightstick:b11,leftshoulder:b4,rightshoulder:b5,dpup:h0.1,dpdown:h0.4,dpleft:h0.8,dpright:h0.2,leftx:a0,lefty:a1,rightx:a2,righty:a5,lefttrigger:a3,righttrigger:a4,platform:Linux,";

#[derive(Debug)]
/// A wrapper around gilrs::Gilrs that Implements tokio_stream::Stream
pub struct AsyncGilrs(pub gilrs::Gilrs);

impl tokio_stream::Stream for AsyncGilrs {
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.get_mut().0.next_event() {
            None => std::task::Poll::Pending,
            Some(e) => std::task::Poll::Ready(Some(e)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    type Item = gilrs::Event;
}
