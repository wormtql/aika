use std::marker::PhantomData;
use std::ops::Add;
use std::rc::Rc;
use cgmath::{BaseFloat, Vector3, Vector4};
use crate::material_graph::{MaterialGraphContext, OutputValue};
use crate::texture::Texture2DTrait;

pub struct Texture2DNode<F> {
    pub texture: Rc<dyn Texture2DTrait<F>>,
}

impl<F> Texture2DNode<F> where F: BaseFloat {
    pub fn new(texture: Rc<dyn Texture2DTrait<F>>) -> Self {
        Self {
            texture
        }
    }
}

impl<F> OutputValue<F, Vector3<F>> for Texture2DNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector3<F> {
        self.texture.sample(context.uv)
    }
}

impl<F> OutputValue<F, Vector4<F>> for Texture2DNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector4<F> {
        let v3 = OutputValue::<F, Vector3<F>>::get_value(self, context);
        Vector4::new(v3.x, v3.y, v3.z, F::zero())
    }
}

impl<F> OutputValue<F, F> for Texture2DNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> F {
        let v3 = OutputValue::<F, Vector3<F>>::get_value(self, context);
        v3.x
    }
}

pub struct Vector3ConstantNode<F> {
    pub value: Vector3<F>,
}

impl<F> OutputValue<F, Vector3<F>> for Vector3ConstantNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector3<F> {
        self.value
    }
}

pub struct FloatConstantNode<F> {
    pub value: F,
}

impl<F> OutputValue<F, F> for FloatConstantNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> F {
        self.value
    }
}

impl<F> OutputValue<F, Vector3<F>> for FloatConstantNode<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector3<F> {
        Vector3::new(self.value, self.value, self.value)
    }
}

pub struct AddNode<F, L, R> {
    pub left: Rc<L>,
    pub right: Rc<R>,
    _float: PhantomData<F>
}

impl<F, L, R, O> OutputValue<F, O> for AddNode<F, L, R>
where
    F: BaseFloat,
    L: OutputValue<F, O>,
    R: OutputValue<F, O>,
    O: Add<Output = O>
{
    fn get_value(&self, context: &MaterialGraphContext<F>) -> O {
        let left = self.left.get_value(context);
        let right = self.right.get_value(context);
        left + right
    }
}

impl<F> OutputValue<F, Vector3<F>> for F where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector3<F> {
        Vector3::new(*self, *self, *self)
    }
}

impl<F> OutputValue<F, F> for F where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> F {
        *self
    }
}

impl<F> OutputValue<F, Vector3<F>> for Vector3<F> where F: BaseFloat {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> Vector3<F> {
        *self
    }
}
