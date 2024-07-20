use serde::Serialize;
use crate::voiceflow::request_structures::State;
use crate::voiceflow::request_structures::action::Action;
use crate::voiceflow::VoiceflowSession;

/// Represents the request body for a Voiceflow API call.
///
/// `VoiceflowRequestBody` contains the action to be performed, the session information,
/// and the optional state for the session.
#[derive(Debug, Serialize)]
pub(crate) struct VoiceflowRequestBody<'a> {
    /// The action to be performed in the Voiceflow API.
    action: Action,

    /// The optional session information for the Voiceflow API.
    #[serde(skip_serializing_if = "Option::is_none")]
    session: Option<&'a VoiceflowSession>,

    /// The optional state for the Voiceflow API.
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<State>,
}

impl<'a> VoiceflowRequestBody<'a> {
    /// Converts the `VoiceflowRequestBody` to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON string representation of the `VoiceflowRequestBody`.
    ///
    /// # Example
    ///
    /// ```
    /// let request_body = VoiceflowRequestBody { action, session: Some(&session), state: Some(state) };
    /// let json = request_body.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// A builder for creating a `VoiceflowRequestBody`.
///
/// `VoiceflowRequestBodyBuilder` allows for the incremental construction of a `VoiceflowRequestBody`
/// by setting various fields.
pub(crate) struct VoiceflowRequestBodyBuilder<'a> {
    action: Action,
    session: Option<&'a VoiceflowSession>,
    state: Option<State>,
}

impl<'a> VoiceflowRequestBodyBuilder<'a> {
    /// Creates a new `VoiceflowRequestBodyBuilder` with the specified action.
    ///
    /// # Parameters
    ///
    /// * `action` - The action to be performed in the Voiceflow API.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowRequestBodyBuilder`.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = VoiceflowRequestBodyBuilder::new(action);
    /// ```
    pub fn new(action: Action) -> Self {
        Self {
            action,
            session: None,
            state: None,
        }
    }

    /// Sets the session for the `VoiceflowRequestBody`.
    ///
    /// # Parameters
    ///
    /// * `session` - The optional session information for the Voiceflow API.
    ///
    /// # Returns
    ///
    /// The `VoiceflowRequestBodyBuilder` with the session set.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = builder.session(Some(&session));
    /// ```
    pub fn session(mut self, session: Option<&'a VoiceflowSession>) -> Self {
        self.session = session;
        self
    }

    /// Sets the state for the `VoiceflowRequestBody`.
    ///
    /// # Parameters
    ///
    /// * `state` - The optional state for the Voiceflow API.
    ///
    /// # Returns
    ///
    /// The `VoiceflowRequestBodyBuilder` with the state set.
    ///
    /// # Example
    ///
    /// ```
    /// let builder = builder.state(Some(state));
    /// ```
    pub fn state(mut self, state: Option<State>) -> Self {
        self.state = state;
        self
    }

    /// Builds the `VoiceflowRequestBody`.
    ///
    /// # Returns
    ///
    /// A new instance of `VoiceflowRequestBody` with the configured fields.
    ///
    /// # Example
    ///
    /// ```
    /// let request_body = builder.build();
    /// ```
    pub fn build(self) -> VoiceflowRequestBody<'a> {
        VoiceflowRequestBody {
            action: self.action,
            session: self.session,
            state: self.state,
        }
    }
}