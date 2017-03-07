use events::{MainFromControl, MainFromGame, MainFromRender, MainToControl, MainToGame, MainToRender};
use utils::DuoChannel;

pub fn make_event_clumps<ID>() -> (FrontEventClump<ID>, BackEventClump<ID>)
    where ID: Send + Eq
{
    let (front_control, back_control) = DuoChannel::new_both();
    let (front_game, back_game) = DuoChannel::new_both();
    let (front_render, back_render) = DuoChannel::new_both();

    (FrontEventClump::new(front_control, front_game, front_render), BackEventClump::new(back_control, back_game, back_render))
}

pub struct BackEventClump<ID>
    where ID: Send + Eq
{
    control: Option<DuoChannel<MainFromControl, MainToControl>>,
    game: Option<DuoChannel<MainFromGame, MainToGame>>,
    render: Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>>,
}

impl<ID> BackEventClump<ID>
    where ID: Send + Eq
{
    fn new(control: DuoChannel<MainFromControl, MainToControl>, game: DuoChannel<MainFromGame, MainToGame>, render: DuoChannel<MainFromRender<ID>, MainToRender<ID>>) -> BackEventClump<ID> {
        BackEventClump {
            control: Some(control),
            game: Some(game),
            render: Some(render),
        }
    }

    pub fn take_control(&mut self) -> Option<DuoChannel<MainFromControl, MainToControl>> {
        self.control.take()
    }

    pub fn take_game(&mut self) -> Option<DuoChannel<MainFromGame, MainToGame>> {
        self.game.take()
    }

    pub fn take_render(&mut self) -> Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>> {
        self.render.take()
    }
}

pub struct FrontEventClump<ID>
    where ID: Send + Eq
{
    control: Option<DuoChannel<MainToControl, MainFromControl>>,
    game: Option<DuoChannel<MainToGame, MainFromGame>>,
    render: Option<DuoChannel<MainToRender<ID>, MainFromRender<ID>>>,
}

impl<ID> FrontEventClump<ID>
    where ID: Send + Eq
{
    fn new(control: DuoChannel<MainToControl, MainFromControl>, game: DuoChannel<MainToGame, MainFromGame>, render: DuoChannel<MainToRender<ID>, MainFromRender<ID>>) -> FrontEventClump<ID> {
        FrontEventClump {
            control: Some(control),
            game: Some(game),
            render: Some(render),
        }
    }

    pub fn get_mut_control(&mut self) -> Option<&mut DuoChannel<MainToControl, MainFromControl>> {
        self.control.as_mut()
    }

    pub fn get_mut_game(&mut self) -> Option<&mut DuoChannel<MainToGame, MainFromGame>> {
        self.game.as_mut()
    }

    pub fn get_mut_render(&mut self) -> Option<&mut DuoChannel<MainToRender<ID>, MainFromRender<ID>>> {
        self.render.as_mut()
    }
}
