use std::any::Any;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::vm::value::Value;
use crate::obj::object::{GloomObjRef, Object, ObjectType};
use crate::vm::machine::GloomVM;

pub struct GloomListIter {
    rf : GloomObjRef,
    curr : Cell<usize>
}

impl GloomListIter {
    #[inline]
    pub fn new(rf : GloomObjRef) -> GloomObjRef {
        GloomObjRef::new(Rc::new(
            GloomListIter {
                rf,
                curr: Cell::new(0)
            }
        ))
    }
}

impl Debug for GloomListIter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?} #{}",self.rf,self.curr.get())
    }
}

impl Object for GloomListIter{
    fn obj_type(&self) -> ObjectType {
        ObjectType::ListIter
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn drop_by_vm(&self, _ : &GloomVM, _ : &GloomObjRef) {}

    fn iter(&self, _ : &GloomObjRef) -> GloomObjRef {
        panic!()
    }

    fn at(&self, _ : &mut usize) -> Option<Value> {
        panic!()
    }

    fn next(&self) -> Option<Value> {
        let mut index = self.curr.get();
        let option = self.rf.at(&mut index);
        self.curr.set(index);
        option
    }
}