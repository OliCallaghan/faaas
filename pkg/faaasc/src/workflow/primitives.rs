pub struct Workflow(pub Primitive);

pub enum Primitive {
    Linear(Box<Primitive>, Box<Primitive>),
    Task(String),
}
