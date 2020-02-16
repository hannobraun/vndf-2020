pub mod strong;
pub mod weak;

pub use self::{
    strong::Strong,
    weak::Weak,
};


use std::{
    fmt,
    hash::{
        Hash,
        Hasher,
    }
};

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};


/// A handle that can be either strong or weak
///
/// There are situations where you might want to support using either a strong
/// or a weak handle. One such situation is a networked multiplayer scenario
/// where components can either live on the server (where they have strong
/// handles), or on the client (where they have weak handles).
///
/// This distinction is necessary because of the different attributes of strong
/// and weak handles. Strong handles keep the components they're referrring to
/// alive while they exist, which is necessary in the actual core game code on
/// the server. They also enable users to get a component from a store without
/// having to check whether the component actually exists. We have a strong
/// handle. We know it exists.
///
/// But strong handles can't be serialized and sent to the client. A strong
/// handle has no meaning without an internal reference to its store (to manage
/// the reference counting). How would one serialize it on its own? That's why
/// we need to send weak handles to the client.
///
/// We don't want to write two versions of all our components (one version with
/// strong handles, one with weak ones), so we need some way to abstract over
/// both types of handles. Ideally, we would do so at compile-time, using a type
/// parameter on the component type. We could even specify a default type for
/// this type argument, to make the more important case for that specific
/// component that much more convenient.
///
/// Using such a solution, we could write code that only exists on the server
/// against the strong-handled version, while making code existing on both sides
/// generic. All of that can't be, however, as handles have a type parameter
/// themselves, meaning that using them in such a way would require higher-
/// kinded types, which aren't supported in Rust (yet).
///
/// Hence, this enum. While it allows us to write code that is generic over the
/// type of handle used, it comes with runtime overhead, and makes code that
/// "knows" it's only ever going to deal with strong handles panicky. Still,
/// it's the best solution we have at this point. Once higher-kinded types are
/// available, it can be replaced with a trait.
pub enum Handle<T> {
    Strong(Strong<T>),
    Weak(Weak<T>),
}

impl<T> Handle<T> {
    pub fn strong(self) -> Strong<T> {
        match self {
            Self::Strong(handle) => handle,
            Self::Weak(_)        => panic!("Expected strong handle; was weak"),
        }
    }
    pub fn weak(&self) -> Weak<T> {
        match self {
            Self::Strong(handle) => handle.into(),
            Self::Weak(handle)   => *handle,
        }
    }
}

impl<T> From<Strong<T>> for Handle<T> {
    fn from(handle: Strong<T>) -> Self {
        Handle::Strong(handle)
    }
}

impl<T> From<Weak<T>> for Handle<T> {
    fn from(handle: Weak<T>) -> Self {
        Handle::Weak(handle)
    }
}

impl<T> From<Handle<T>> for Weak<T> {
    fn from(handle: Handle<T>) -> Self {
        handle.weak()
    }
}

impl<T> From<&Handle<T>> for Weak<T> {
    fn from(handle: &Handle<T>) -> Self {
        handle.weak()
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Strong(handle) => Self::Strong(handle.clone()),
            Self::Weak(handle)   => Self::Weak(*handle),
        }
    }
}

impl<T> fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Strong(handle) => {
                write!(f, "Handle::Strong(")?;
                handle.fmt(f)?;
                write!(f, ")")?;
            }
            Self::Weak(handle) => {
                write!(f, "Handle::Weak(")?;
                handle.fmt(f)?;
                write!(f, ")")?;
            }
        }

        Ok(())
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Strong(a), Self::Strong(b)) => a.eq(b),
            (Self::Weak(a), Self::Weak(b))     => a.eq(b),

            _ => false,
        }
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        match self {
            Self::Strong(handle) => handle.hash(state),
            Self::Weak(handle)   => handle.hash(state),
        }
    }
}

impl<T> Serialize for Handle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self {
            Self::Strong(_)    => panic!("Can't serialize strong handle"),
            Self::Weak(handle) => handle.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for Handle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let handle = Weak::deserialize(deserializer)?;
        Ok(Self::Weak(handle))
    }
}
