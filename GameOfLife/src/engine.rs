use std::rc::Rc;
use std::cell::RefCell;

use crate::canvas::Canvas;
use crate::rules::GameOfLife;

use stdweb::{
    console,
    traits::IMouseEvent,
    web::{
        set_timeout,
        window,
        IEventTarget,
        event::{
            IEvent,
            MouseButton,
            MouseDownEvent,
        }
    }
};


pub fn start() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 50, 50);
    let mut cells = Rc::new(RefCell::new(GameOfLife::new(50, 50)));

    window().add_event_listener({
        let mut cells = cells.clone();
        move |event: MouseDownEvent| {
            let pos_x = ((50 * event.client_x()) / 600) as u32;
            let pos_y = ((50 * event.client_y()) / 600) as u32;

            console!(log, pos_x, pos_y);

            cells.borrow_mut().set(pos_x as usize, pos_y as usize);
        }
    });

    // cells.borrow_mut().set(10, 10);
    // cells.borrow_mut().set(12, 10);


    // cells.borrow_mut().draw(&canvas);
    game_loop(cells, Rc::new(canvas), 100);

    stdweb::event_loop();
}


fn game_loop(cells: Rc<RefCell<GameOfLife>>, canvas: Rc<Canvas>, time: u32) {
    set_timeout(
        move || {
            game_loop(cells.clone(), canvas.clone(), time);
            cells.borrow().draw(&canvas);
        }, 
        time
    );
}
