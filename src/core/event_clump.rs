use events::{MainFromControl, MainToControl};
use events::{MainFromRender, MainToRender};
use utils::DuoChannel;

pub fn make_event_clumps<ID>(front_control_id: ID, back_control_id: ID, front_render_id: ID, back_render_id: ID) -> (FrontEventClump<ID>, BackEventClump<ID>)
    where ID: Send + Eq
{
    let (front_control, back_control) = DuoChannel::new_both();
    let (front_render, back_render) = DuoChannel::new_both();

    (FrontEventClump::new(front_control, front_render), BackEventClump::new(back_control, back_render))
}

pub struct BackEventClump<ID>
    where ID: Send + Eq
{
    control: Option<DuoChannel<MainFromControl, MainToControl>>,
    render: Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>>,
}

impl<ID> BackEventClump<ID>
    where ID: Send + Eq
{
    fn new(control: DuoChannel<MainFromControl, MainToControl>, render: DuoChannel<MainFromRender<ID>, MainToRender<ID>>) -> BackEventClump<ID> {
        BackEventClump {
            control: Some(control),
            render: Some(render),
        }
    }

    pub fn take_render(&mut self) -> Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>> {
        self.render.take()
    }

    pub fn take_control(&mut self) -> Option<DuoChannel<MainFromControl, MainToControl>> {
        self.control.take()
    }
}

pub struct FrontEventClump<ID>
    where ID: Send + Eq
{
    control: Option<DuoChannel<MainToControl, MainFromControl>>,
    render: Option<DuoChannel<MainToRender<ID>, MainFromRender<ID>>>,
}

impl<ID> FrontEventClump<ID>
    where ID: Send + Eq
{
    fn new(control: DuoChannel<MainToControl, MainFromControl>, render: DuoChannel<MainToRender<ID>, MainFromRender<ID>>) -> FrontEventClump<ID> {
        FrontEventClump {
            control: Some(control),
            render: Some(render),
        }
    }

    pub fn get_mut_control(&mut self) -> Option<&mut DuoChannel<MainToControl, MainFromControl>> {
        self.control.as_mut()
    }

    pub fn get_mut_render(&mut self) -> Option<&mut DuoChannel<MainToRender<ID>, MainFromRender<ID>>> {
        self.render.as_mut()
    }
}
