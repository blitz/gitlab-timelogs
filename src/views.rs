/*
MIT License

Copyright (c) 2024 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! Provides transform functions for different views into the data.

use crate::gitlab_api::types::ResponseNode;
use chrono::{Datelike, IsoWeek, NaiveDate};
use std::collections::BTreeMap;
use std::time::Duration;

/// Returns the nodes per [`IsoWeek`].
pub fn to_nodes_by_week<'a>(
    nodes: &[&'a ResponseNode],
) -> BTreeMap<IsoWeek, Vec<&'a ResponseNode>> {
    let weeks = nodes
        .iter()
        .map(|node| node.datetime().iso_week())
        .collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    for week in weeks {
        let nodes_of_week = nodes
            .iter()
            .filter(|node| node.datetime().iso_week() == week)
            .cloned()
            .collect::<Vec<_>>();

        map.entry(week).or_insert(nodes_of_week);
    }
    map
}

/// Returns the nodes per [`NaiveDate`].
pub fn to_nodes_by_day<'a>(
    nodes: &[&'a ResponseNode],
) -> BTreeMap<NaiveDate, Vec<&'a ResponseNode>> {
    let days = nodes.iter().map(|node| node.datetime()).collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    for day in days {
        let nodes_of_week = nodes
            .iter()
            .filter(|node| node.datetime() == day)
            .cloned()
            .collect::<Vec<_>>();

        map.entry(day).or_insert(nodes_of_week);
    }
    map
}

/// Returns the time spent per day.
pub fn to_time_spent_sum(nodes: &[&ResponseNode]) -> Duration {
    nodes.iter().map(|node| node.timeSpent().1).sum()
}
