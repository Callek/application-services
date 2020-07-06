/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// A somewhat mixed-bag of all telemetry we want to collect. The idea is that
// the app will "pull" telemetry via a new API whenever it thinks there might
// be something to record.
// It's considered a temporary solution until either we can record it directly
// (eg, via glean) or we come up with something better.

use serde_derive::*;

/// The reason a tab/command was received.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReceivedReason {
    /// A push notification for the command was received.
    Push,
    /// Discovered while handling a push notification for a later message.
    PushMissed,
    /// Explicit polling for missed commands.
    Poll,
}

#[derive(Debug, Serialize)]
pub struct SentCommand {
    pub flow_id: String,
    pub stream_id: String,
}

#[derive(Debug, Serialize)]
pub struct ReceivedCommand {
    pub flow_id: String,
    pub stream_id: String,
    pub reason: ReceivedReason,
}

// We have a naive strategy to avoid unbounded memory growth - the intention
// is that if any platform lets things grow to hit these limits, it's probably
// never going to consume anything - so it doesn't matter what we discard (ie,
// there's no good reason to have a smarter circular buffer etc)
const MAX_TAB_EVENTS: usize = 200;

#[derive(Debug, Default, Serialize)]
pub struct FxaTelemetry {
    commands_sent: Vec<SentCommand>,
    commands_received: Vec<ReceivedCommand>,
}

impl FxaTelemetry {
    pub fn new() -> Self {
        FxaTelemetry {
            ..Default::default()
        }
    }

    pub fn record_tab_sent(&mut self, sent: SentCommand) {
        if self.commands_sent.len() < MAX_TAB_EVENTS {
            self.commands_sent.push(sent);
        }
    }

    pub fn record_tab_received(&mut self, recd: ReceivedCommand) {
        if self.commands_received.len() < MAX_TAB_EVENTS {
            self.commands_received.push(recd);
        }
    }
}
