use crate::material_graph::MaterialGraphContext;

pub trait OutputValue<F, V> {
    fn get_value(&self, context: &MaterialGraphContext<F>) -> V;
}

