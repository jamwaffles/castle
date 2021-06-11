mod db_event;
mod event;
mod event_builder;

pub use event::Event;
use event_builder::EventBuilder;
use serde::Serialize;
use uuid::Uuid;

/// An entity to apply events to.
pub trait Entity {
    /// The type of this entity as a plural `underscore_case` string.
    const ENTITY_TYPE: &'static str;

    /// Get the ID of this entity.
    fn entity_id(&self) -> Uuid;
}

pub trait EventData: Serialize + Sized {
    /// The entity to bind this event to
    type Entity: Entity;

    // /// The type of builder this event can be used with
    // type Builder: EventBuilder<Self>;

    /// Get the event type/identifier in PascalCase like `UserCreated` or `PasswordChanged`.
    fn event_type() -> &'static str;

    fn into_builder(self) -> EventBuilder<Self> {
        EventBuilder::from(self)
    }

    // /// Convert the event into a builder with a given session ID
    // ///
    // /// This is a convenience method to shorten `Event {}.into_builder().session_id(id)` to
    // /// `Event {}.with_session_id(id)`.
    // fn with_session_id(self, session_id: Uuid) -> Self::Builder {
    //     Self::Builder::new(self).session_id(session_id)
    // }

    // /// Convert the event into a builder
    // fn into_builder(self) -> Self::Builder {
    //     Self::Builder::new(self)
    // }

    // /// Wrap this event data in an [`Event`] and assign the given entity ID.
    // fn with_entity_id(self, entity_id: Uuid) -> Event<Self> {
    //     Event {
    //         entity_id,
    //         ..Event::from(self)
    //     }
    // }

    //   /// Wrap this event data in an [`Event`] and assign the given entity ID.
    // fn with_session_id(self, entity_id: Uuid) -> Event<Self> {
    //     Event {
    //         entity_id,
    //         ..Event::from(self)
    //     }
    // }
}

pub trait Create<D>: Entity + Sized
where
    D: EventData,
{
    /// Create an instance of this entity from the given event.
    fn create_from(event: &Event<D>) -> Self;
}

pub trait CreatePersister<D>: Create<D>
where
    D: EventData,
{
    fn create(event: impl Into<Event<D>>) -> Persister<D, Self> {
        let event = event.into();

        let entity = Self::create_from(&event);

        Persister { entity, event }
    }
}

/// Blanket impl
impl<D, C> CreatePersister<D> for C
where
    C: Create<D>,
    D: EventData,
{
}

#[non_exhaustive]
pub struct Persister<D, E>
where
    D: EventData,
    E: Entity,
{
    pub event: Event<D>,
    pub entity: E,
}

pub trait Persistable<S, Out = Self>: Sized
where
    S: Storage,
{
    fn persist(self, storage: &mut S) -> Result<Out, S::Error>;
}

pub trait Storage {
    type Error;
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::db_event::DBEvent;

    use super::*;

    #[test]
    fn create() {
        // ---
        // Storage provider "crate"
        // ---

        #[derive(Debug)]
        struct FakeStorage {
            items: Vec<User>,
            events: Vec<DBEvent>,
        }

        impl Storage for FakeStorage {
            type Error = ();
        }

        impl<D, E> Persistable<FakeStorage, E> for Persister<D, E>
        where
            D: EventData,
            E: Entity + Persistable<FakeStorage>,
        {
            fn persist(
                self,
                storage: &mut FakeStorage,
            ) -> Result<E, <FakeStorage as Storage>::Error> {
                let event: DBEvent = self.event.try_into().expect("Failed to convert event");

                storage.events.push(event);

                self.entity.persist(storage)
            }
        }

        // ---
        // Client implementation
        // ---

        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        struct User {
            name: String,
            email: String,
        }

        impl Entity for User {
            const ENTITY_TYPE: &'static str = "users";

            fn entity_id(&self) -> Uuid {
                Uuid::nil()
            }
        }

        impl Persistable<FakeStorage> for User {
            fn persist(
                self,
                storage: &mut FakeStorage,
            ) -> Result<Self, <FakeStorage as Storage>::Error> {
                storage.items.push(self.clone());

                Ok(self)
            }
        }

        #[derive(serde::Serialize, serde::Deserialize)]
        struct UserCreated {
            name: String,
            email: String,
        }

        impl EventData for UserCreated {
            type Entity = User;

            fn event_type() -> &'static str {
                "UserCreated"
            }
        }

        impl Create<UserCreated> for User {
            fn create_from(event: &Event<UserCreated>) -> Self {
                Self {
                    name: event.data.name.clone(),
                    email: event.data.email.clone(),
                }
            }
        }

        let mut storage = FakeStorage {
            items: Vec::new(),
            events: Vec::new(),
        };

        let user = User::create(UserCreated {
            name: "Foo Bar".to_string(),
            email: "foo@bar.com".to_string(),
        })
        .persist(&mut storage)
        .unwrap();

        dbg!(user);

        dbg!(storage);
    }
}

// trait FallibleCreate<Evt>: Sized
// where
//     Evt: EventData,
// {
//     type Error;

//     fn try_create(event: Event<Evt>) -> Result<Self, Self::Error>;
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[derive(Debug, Clone)]
//     struct User {
//         name: String,
//         email: String,
//     }

//     #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
//     struct UserCreated {
//         name: String,
//         email: String,
//     }

//     #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
//     struct UserNameUpdated {
//         name: String,
//     }

//     #[derive(serde_derive::Serialize, serde_derive::Deserialize)]
//     struct UserDeleted;

//     impl EventData for UserCreated {}
//     impl EventData for UserNameUpdated {}
//     impl EventData for UserDeleted {}

//     impl Create<UserCreated> for User {
//         fn create(event: Event<UserCreated>) -> Self {
//             Self {
//                 name: event.data.name,
//                 email: event.data.email,
//             }
//         }
//     }

//     #[test]
//     fn create() {
//         //
//     }

//     #[test]
//     fn fallible_create() {
//         //
//     }

//     #[test]
//     fn update() {
//         //
//     }

//     #[test]
//     fn fallible_update() {
//         //
//     }

//     #[test]
//     fn delete() {
//         //
//     }

//     #[test]
//     fn fallible_delete() {
//         //
//     }
// }
