use crate::core::channel_list::ChannelList;

/// Return the name of the default view given a multi-view slice of strings.
///
/// # Returns
/// * `None` if the slice is empty
/// * A reference to the first element of the slice, otherwise
///
pub fn default_view_name<S: AsRef<str>>(multi_view: &[S]) -> Option<&str> {
    multi_view.first().map(|s| s.as_ref())
}

/// Find the name of the view to which the given channel belongs.
///
/// If the channel is not a member of any named view, return `None`
///
pub fn view_from_channel_name<'a, S: AsRef<str>>(
    channel: &str,
    multi_view: &'a [S],
) -> Option<&'a str> {
    let toks: Vec<&str> = channel.split('.').collect();

    match toks.len() {
        0 => None,
        1 => default_view_name(multi_view),
        _ => {
            // view name is second-to-last token
            let view_name = toks[toks.len() - 2];
            // if view_name is in the views, return it, otherwise empty string
            multi_view
                .iter()
                .find(|v| v.as_ref() == view_name)
                .map(|s| s.as_ref())
        }
    }
}

#[cfg(test)]
#[test]
fn test_view_from_channel_name() {
    let views = ["left", "right", "center"];

    assert_eq!(view_from_channel_name("left.diffuse", &views), Some("left"));
    assert_eq!(
        view_from_channel_name("left.diffuse.R", &views),
        None //< Shouldn't this be Some("left")?
    );
    assert!(view_from_channel_name("hero.diffuse.R", &views).is_none());
}

/// Return whether channel1 and channel2 are the same channel but in different views.
///
/// # Returns
/// * `true` if `channel1` and `channel2` are the same channel in different views
/// * `false` is `channel1` and `channel2` are not the same channel or are in the same view.
///
pub fn are_counterparts<S: AsRef<str>>(
    channel1: &str,
    channel2: &str,
    multi_view: &[S],
) -> bool {
    let toks1: Vec<&str> = channel1.split('.').collect();
    let toks2: Vec<&str> = channel2.split('.').collect();

    if toks1.is_empty() || toks2.is_empty() {
        return false;
    }

    let v1 = view_from_channel_name(channel1, multi_view);
    let v2 = view_from_channel_name(channel2, multi_view);

    // if either is in no view, then they can't be counterparts
    if v1.is_none() || v2.is_none() {
        return false;
    }

    // if they're in the same view, they can't be counterparts
    if v1 == v2 {
        return false;
    }

    // If channel1 is a default channel, they can only be counterparts if
    // channel2 is of the form <view>.<channel1>
    if toks1.len() == 1 {
        return toks2.len() == 2 && toks1[0] == toks2[1];
    }

    // If channel2 is a default channel, they can only be counterparts if
    // channel1 is of the form <view>.<channel2>
    if toks2.len() == 1 {
        return toks1.len() == 2 && toks1[1] == toks2[0];
    }

    // Neither channel is a default channel.  To be counterparts both
    // channel names must have the same number of components, and
    // all components except the penultimate one must be the same.
    if toks1.len() != toks2.len() {
        return false;
    }

    for i in 0..toks1.len() {
        if i != toks1.len() - 2 && toks1[i] != toks2[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
#[test]
fn test_are_counterparts() {
    let views = ["left", "right", "center"];

    assert_eq!(
        are_counterparts("left.diffuse", "right.diffuse", &views),
        true
    );

    assert_eq!(
        are_counterparts("left.diffuse.R", "right.diffuse.R", &views),
        false //< shouldn't this be true?
    );

    assert_eq!(
        are_counterparts("left.diffuse.R", "hero.diffuse.R", &views),
        false
    );

    assert_eq!(
        are_counterparts("left.diffuse.R", "right.specular.R", &views),
        false
    );
}

/// Return a list of all channels belonging to view `view_name`
///
pub fn channels_in_view<S: AsRef<str>>(
    view_name: &str,
    channel_list: &ChannelList,
    multi_view: &[S],
) -> ChannelList {
    let mut result = ChannelList::new();

    for (channel_name, channel) in channel_list.iter() {
        if let Some(view) = view_from_channel_name(channel_name, multi_view) {
            if view == view_name {
                result.insert(channel_name, channel);
            }
        }
    }

    result
}

/// Return a list of channels not associated with any view
///
pub fn channels_in_no_view<S: AsRef<str>>(
    channel_list: &ChannelList,
    multi_view: &[S],
) -> ChannelList {
    let mut result = ChannelList::new();

    for (channel_name, channel) in channel_list.iter() {
        if view_from_channel_name(channel_name, multi_view).is_none() {
            result.insert(channel_name, channel);
        }
    }

    result
}

/// Given a channel name, return a list of the same channel in all views.
///
pub fn channel_in_all_views<S: AsRef<str>>(
    channel_name: &str,
    channel_list: &ChannelList,
    multi_view: &[S],
) -> ChannelList {
    let mut result = ChannelList::new();

    for (name, channel) in channel_list.iter() {
        if name == channel_name
            || are_counterparts(channel_name, name, multi_view)
        {
            result.insert(name, channel);
        }
    }

    result
}

/// Given the name of a channel in one view, return the name of the corresponding
/// channel in `other_view`
///
pub fn channel_in_other_view<'a, S: AsRef<str>>(
    channel_name: &str,
    channel_list: &'a ChannelList,
    multi_view: &[S],
    other_view: &str,
) -> Option<&'a str> {
    for (name, _) in channel_list.iter() {
        match view_from_channel_name(name, multi_view) {
            Some(v)
                if are_counterparts(name, channel_name, multi_view)
                    && v == other_view =>
            {
                return Some(name);
            }
            _ => (),
        }
    }

    None
}

#[cfg(test)]
#[test]
fn test_channel_in_other_view() {
    use crate::core::channel_list::CHANNEL_HALF;

    let mut channel_list = ChannelList::new();

    channel_list.insert("left.diffuse", &CHANNEL_HALF);
    channel_list.insert("right.diffuse", &CHANNEL_HALF);
    channel_list.insert("center.diffuse", &CHANNEL_HALF);

    let views = ["left", "right", "center"];

    assert_eq!(
        channel_in_other_view("left.diffuse", &channel_list, &views, "right"),
        Some("right.diffuse")
    );

    assert_eq!(
        channel_in_other_view("left.diffuse", &channel_list, &views, "nonono"),
        None
    );

    assert_eq!(
        channel_in_other_view("left.nonono", &channel_list, &views, "right"),
        None
    );
}
