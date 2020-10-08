// External includes.

// Standard includes.

// Internal includes.
use super::SupportsDunGen;

pub trait DoesDunGen<'a> {
    fn dun_gen<'b>(&self, target: &mut dyn SupportsDunGen<'b>);
}

pub trait DoesDunGenStatic<'a> {
    fn dun_gen_static<'b>(target: &mut dyn SupportsDunGen<'b>);
}
