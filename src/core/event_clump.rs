use events::{GameFromMainMenu, GameToMainMenu, MainFromControl, MainFromGame, MainFromRender, MainToControl, MainToGame, MainToRender};
use utils::DuoChannel;

pub fn make_event_clumps<ID>() -> (FrontEventClump<ID>, BackEventClump<ID>)
    where ID: Send + Eq
{
    let (front_main_x_control, back_main_x_control) = DuoChannel::new_both();
    let (front_main_x_game, back_main_x_game) = DuoChannel::new_both();
    let (front_main_x_render, back_main_x_render) = DuoChannel::new_both();
    let (front_game_x_main_menu, back_game_x_main_menu) = DuoChannel::new_both();

    (FrontEventClump::new(front_main_x_control, front_main_x_game, front_main_x_render), BackEventClump::new(back_main_x_control, back_main_x_game, back_main_x_render, front_game_x_main_menu, back_game_x_main_menu))
}

pub struct BackEventClump<ID>
    where ID: Send + Eq
{
    main_x_control: Option<DuoChannel<MainFromControl, MainToControl>>,
    main_x_game: Option<DuoChannel<MainFromGame, MainToGame>>,
    main_x_render: Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>>,
    front_game_x_main_menu: Option<DuoChannel<GameToMainMenu, GameFromMainMenu>>,
    back_game_x_main_menu: Option<DuoChannel<GameFromMainMenu, GameToMainMenu>>,
}

impl<ID> BackEventClump<ID>
    where ID: Send + Eq
{
    fn new(main_x_control: DuoChannel<MainFromControl, MainToControl>,
           main_x_game: DuoChannel<MainFromGame, MainToGame>,
           main_x_render: DuoChannel<MainFromRender<ID>, MainToRender<ID>>,
           front_game_x_main_menu: DuoChannel<GameToMainMenu, GameFromMainMenu>,
           back_game_x_main_menu: DuoChannel<GameFromMainMenu, GameToMainMenu>)
           -> BackEventClump<ID> {
        BackEventClump {
            main_x_control: Some(main_x_control),
            main_x_game: Some(main_x_game),
            main_x_render: Some(main_x_render),
            front_game_x_main_menu: Some(front_game_x_main_menu),
            back_game_x_main_menu: Some(back_game_x_main_menu),
        }
    }

    pub fn take_main_x_control(&mut self) -> Option<DuoChannel<MainFromControl, MainToControl>> {
        self.main_x_control.take()
    }

    pub fn take_main_x_game(&mut self) -> Option<DuoChannel<MainFromGame, MainToGame>> {
        self.main_x_game.take()
    }

    pub fn take_main_x_render(&mut self) -> Option<DuoChannel<MainFromRender<ID>, MainToRender<ID>>> {
        self.main_x_render.take()
    }

    pub fn take_front_game_x_main_menu(&mut self) -> Option<DuoChannel<GameToMainMenu, GameFromMainMenu>> {
        self.front_game_x_main_menu.take()
    }

    pub fn take_back_game_x_main_menu(&mut self) -> Option<DuoChannel<GameFromMainMenu, GameToMainMenu>> {
        self.back_game_x_main_menu.take()
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
