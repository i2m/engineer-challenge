use crate::operation::Operation;

///////////////////////////////////////////////////////////////////////////////
/// Workflow definition
///////////////////////////////////////////////////////////////////////////////

/// Workflow as Free Monad
pub enum Workflow<T> {
    Return(T),
    Run(Operation<Box<Workflow<T>>>),
}

/// Workflow as Functor instance methods
impl<T: 'static> Workflow<T> {
    pub fn map<U: 'static>(self, f: impl FnOnce(T) -> U + 'static) -> Workflow<U> {
        self.and_then(|t| Workflow::Return(f(t)))
    }
}

/// Workflow as Monad instance methods
impl<T: 'static> Workflow<T> {
    pub fn from(t: T) -> Self {
        Workflow::Return(t)
    }

    pub fn and_then<U: 'static>(self, f: impl FnOnce(T) -> Workflow<U> + 'static) -> Workflow<U> {
        match self {
            Workflow::Return(t) => f(t),
            Workflow::Run(a) => Workflow::Run(a.map(|a| Box::new(a.and_then(f)))),
        }
    }
}

/// Workflow helper methods
impl<T: 'static> Workflow<T> {
    pub fn lift(a: Operation<T>) -> Self {
        Workflow::Run(a.map(|t| Box::new(Workflow::Return(t))))
    }
}
