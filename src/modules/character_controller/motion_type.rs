use std::any::Any;

use super::{motion::Motion, walk::MotionTypeContext};

pub trait MotionType: 'static + Send + Sync {
    const NAME: &'static str;

    type State: Default + Send + Sync;

    fn apply(&self, state: &mut Self::State, ctx: MotionTypeContext, motion: &mut Motion);
}

pub trait DynamicMotionType: Send + Sync + Any + 'static {
    #[doc(hidden)]
    fn as_any(&self) -> &dyn Any;

    #[doc(hidden)]
    fn as_mut_any(&mut self) -> &mut dyn Any;

    #[doc(hidden)]
    fn apply(&mut self, ctx: MotionTypeContext, motion: &mut Motion);
}

pub(crate) struct BoxableMotionType<M: MotionType> {
    pub(crate) input: M,
    pub(crate) state: M::State,
}

impl<M: MotionType> BoxableMotionType<M> {
    pub(crate) fn new(motion_type: M) -> Self {
        Self {
            input: motion_type,
            state: Default::default(),
        }
    }
}

impl<M: MotionType> DynamicMotionType for BoxableMotionType<M> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }

    fn apply(&mut self, ctx: MotionTypeContext, motion: &mut Motion) {
        self.input.apply(&mut self.state, ctx, motion)
    }
}
