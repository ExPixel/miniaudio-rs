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
