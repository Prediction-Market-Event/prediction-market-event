use nostr::{event::Kind, key::Keys};
#[allow(unused_imports)]
use nostr::{
    event::{Event as NostrEvent, EventBuilder as NostrEventBuilder},
    key::PublicKey,
    types::Filter,
    util::JsonUtil,
    UnsignedEvent as NostrUnsignedEvent,
};

pub type Res<T> = Result<T, crate::Error>;
pub type JsonStr = str;
pub type JsonString = String;

pub trait NostrEventUtils {
    const KIND_U16: u16;
    const KIND: Kind = Kind::Custom(Self::KIND_U16);

    type CreateParameter;

    /// Creates [NostrEventBuilder] using [`Self::CreateParameter`] parameters.
    fn create_nostr_event_builder(param: &Self::CreateParameter) -> Res<NostrEventBuilder>;
    /// Returns [NostrUnsignedEvent] as [JsonString] using event builder created in [`Self::create_nostr_event_builder`]
    fn create_nostr_unsigned_event_json(
        param: &Self::CreateParameter,
        public_key: &str,
    ) -> Res<JsonString> {
        let public_key = PublicKey::parse(public_key)?;
        let builder = Self::create_nostr_event_builder(param)?;
        let nostr_unsigned_event = builder.to_unsigned_event(public_key);
        let nostr_unsigned_event_json = nostr_unsigned_event.try_as_json()?;

        Ok(nostr_unsigned_event_json)
    }
    /// Returns [NostrEvent] as [JsonString] using event builder created in [`Self::create_nostr_event_builder`]
    fn create_nostr_signed_event_json(
        param: &Self::CreateParameter,
        secret_key: &str,
    ) -> Res<JsonString> {
        let keys = Keys::parse(secret_key)?;
        let builder = Self::create_nostr_event_builder(param)?;
        let nostr_event = builder.to_event(&keys)?;
        let nostr_event_json = nostr_event.try_as_json()?;

        Ok(nostr_event_json)
    }

    type InterpretResult;

    /// Interpret [NostrEvent] and return [`Self::InterpretResult`].
    fn interpret_nostr_event(nostr_event: &NostrEvent) -> Res<Self::InterpretResult>;
    /// Accepts [NostrEvent] as [JsonString].
    ///
    /// Return information can be found in [`Self::interpret_nostr_event`]
    fn interpret_nostr_event_json(json: &JsonStr) -> Res<Self::InterpretResult> {
        let nostr_event = NostrEvent::from_json(json)?;
        let interpret_result = Self::interpret_nostr_event(&nostr_event)?;

        Ok(interpret_result)
    }

    /// Returns [`Self`] [NostrEvent] [Filter]
    fn filter() -> Filter;
    /// Returns [Filter] created in [`Self::filter()`] as [JsonString]
    fn filter_json() -> JsonString {
        Self::filter().try_as_json().unwrap()
    }
}
