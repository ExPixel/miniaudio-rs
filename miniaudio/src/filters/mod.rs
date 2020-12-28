pub mod band_pass_filtering;
pub mod biquad_filtering;
pub mod high_pass_filtering;
pub mod high_shelf_filter;
pub mod low_pass_filtering;
pub mod low_shelf_filter;
pub mod notching_filter;
pub mod peaking_eq_filter;

use crate::base::Error;
use crate::frames::{Frames, FramesMut};

pub trait Filter {
    fn process_pcm_frames(&mut self, output: &mut FramesMut, input: &Frames) -> Result<(), Error>;
}

fn ensure_same_format(output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
    if output.format() != input.format() {
        ma_debug_panic!(
            "output and input format did not match (output: {:?}, input: {:?}",
            output.format(),
            input.format()
        );
        Err(Error::InvalidArgs)
    } else {
        Ok(())
    }
}

fn ensure_same_frame_count(output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
    if output.frame_count() != input.frame_count() {
        ma_debug_panic!(
            "output and input buffers did not have the same frame count (output: {}, input: {})",
            output.frame_count(),
            input.frame_count()
        );
        Err(Error::InvalidArgs)
    } else {
        Ok(())
    }
}

fn ensure_frames_compat(output: &mut FramesMut, input: &Frames) -> Result<(), Error> {
    ensure_same_format(output, input)?;
    ensure_same_frame_count(output, input)
}
