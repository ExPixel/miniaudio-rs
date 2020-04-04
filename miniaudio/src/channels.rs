use crate::base::{Channel, ChannelMixMode};

pub struct ChannelMap<'m> {
    mixing_mode: ChannelMixMode,
    map_in: &'m [Channel],
    map_out: &'m [Channel],
}

impl<'m> ChannelMap<'m> {
    pub fn set(index: usize, channel_in: Channel, channel_out: Channel) {
    }
}

pub struct ChannelMapIn()
