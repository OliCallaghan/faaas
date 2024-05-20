use std::{collections::HashMap, marker::PhantomData};

use bitmaps::Bitmap;
use wasmtime::{
    component::{bindgen, Resource},
    Result,
};

use self::faaas::task::types::{
    Host, HostTaskContext, HostTaskError, HostTaskOutput, TaskContext, TaskError, TaskOutput,
    TaskStatus,
};

bindgen!();

struct Task<'a> {
    ctxs: ResourceTable<'a, HashMap<String, u32>, TaskContext>,
}

#[derive(Default)]
struct ResourceTable<'a, T, R>
where
    T: Default,
{
    phantom_resource: PhantomData<&'a R>,
    reps: Bitmap<512>,
    ress: HashMap<u32, T>,
}

impl<'a, T: Default, R: 'static> ResourceTable<'a, T, R> {
    pub fn new() -> Self {
        // ResourceTable::<T, R>::default()
        Self {
            phantom_resource: PhantomData::default(),
            reps: Default::default(),
            ress: Default::default(),
        }
    }

    pub fn new_resource(&mut self) -> Resource<R> {
        let rep = self.reps.first_false_index().expect("available index") as u32;
        let res = T::default();

        self.ress.insert(rep, res);

        Resource::<R>::new_own(rep)
    }

    pub fn lookup(&mut self, res: Resource<R>) -> &mut T {
        self.ress
            .get_mut(&res.rep())
            .expect("resource to be present")
    }
}

impl<'a> Host for Task<'a> {}

impl<'a> HostTaskContext for Task<'a> {
    fn get(&mut self, res: Resource<TaskContext>, key: String) -> Result<u32> {
        Ok(self.ctxs.lookup(res).get(&key).map_or(0, |&v| v))
    }

    fn set(&mut self, res: Resource<TaskContext>, key: String, value: u32) -> Result<()> {
        self.ctxs.lookup(res).insert(key, value);

        Ok(())
    }

    fn drop(&mut self, rep: Resource<TaskContext>) -> Result<()> {
        Ok(())
    }

    fn clone(&mut self, self_: Resource<TaskContext>) -> Result<Resource<TaskContext>> {
        // Needs to clone hash table actually
        Ok(self.ctxs.new_resource())
    }

    fn merge(
        &mut self,
        fst: Resource<TaskContext>,
        snd: Resource<TaskContext>,
    ) -> Result<Resource<TaskContext>> {
        Ok(Resource::<TaskContext>::new_own(0))
    }
}

impl<'a> HostTaskError for Task<'a> {
    fn drop(&mut self, rep: Resource<TaskError>) -> Result<()> {
        Ok(())
    }
}

impl<'a> HostTaskOutput for Task<'a> {
    fn drop(&mut self, rep: Resource<TaskOutput>) -> Result<()> {
        Ok(())
    }

    fn get_status(&mut self, self_: Resource<TaskOutput>) -> Result<TaskStatus> {
        Ok(TaskStatus::Success)
    }

    fn new(&mut self) -> Result<Resource<TaskOutput>> {
        Ok(Resource::<TaskOutput>::new_own(0))
    }

    fn set_status(&mut self, self_: Resource<TaskOutput>, status: TaskStatus) -> Result<()> {
        Ok(())
    }
}
