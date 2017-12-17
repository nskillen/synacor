#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum OpRes<E> {
    Success,
    Failure(E),
}

impl<E> OpRes<E> {
    pub fn is_success(&self) -> bool {
        match *self {
            OpRes::Success => true,
            OpRes::Failure(_) => false,
        }
    }
    pub fn is_failure(&self) -> bool { !self.is_success() }

    pub fn unwrap_failure(self) -> E {
        match self {
            OpRes::Success => panic!("Attempted to unwrap_failure on success"),
            OpRes::Failure(e) => e,
        }
    }
}

use std::ops::Try;

impl<E> Try for OpRes<E> {
    type Ok = ();
    type Error = E;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            OpRes::Success => Ok(()),
            OpRes::Failure(e) => Err(e),
        }
    }

    fn from_error(v: Self::Error) -> Self { OpRes::Failure(v) }
    fn from_ok(_v: Self::Ok) -> Self { OpRes::Success }
}
/*
trait MaybeExt<T,E> {
    fn to_opres(&self) -> OpRes<E>;
    fn from_opres(o: &OpRes<E>) -> Self;
}

impl<T: Default,E> MaybeExt<T,E> for Result<T, E> {
    fn to_opres(&self) -> OpRes<E> {
        match *self {
            Ok(_) => OpRes::Success,
            Err(e) => OpRes::Failure(e),
        }
    }

    fn from_opres(o: &OpRes<E>) -> Self {
        match *o {
            OpRes::Success => Ok(T::default()),
            OpRes::Failure(ref e) => Err(*e)
        }
    }
}

impl<T: Default,E: Default> MaybeExt<T,E> for Option<T> {
    fn to_opres(&self) -> OpRes<E> {
        match *self {
            Some(_) => OpRes::Success,
            None => OpRes::Failure(E::default()),
        }
    }

    fn from_opres(o: &OpRes<E>) -> Self {
        match *o {
            OpRes::Success => Some(T::default()),
            OpRes::Failure(ref e) => None
        }
    }
}
*/