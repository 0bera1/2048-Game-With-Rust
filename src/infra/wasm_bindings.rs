use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{window, KeyboardEvent};

use crate::application::game_service::GameService;
use crate::domain::direction::Direction;

#[wasm_bindgen]
pub struct WasmGameService {
    inner: GameService,
}

#[wasm_bindgen]
impl WasmGameService {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> WasmGameService {
        WasmGameService { inner: GameService::new(size) }
    }

    pub fn reset(&mut self) { self.inner.reset(); }
    pub fn score(&self) -> u32 { self.inner.score() }
    pub fn is_over(&self) -> bool { self.inner.is_over() }
    pub fn is_won(&self) -> bool { self.inner.is_won() }

    pub fn slide_left(&mut self) -> bool { self.inner.slide(Direction::Left) }
    pub fn slide_right(&mut self) -> bool { self.inner.slide(Direction::Right) }
    pub fn slide_up(&mut self) -> bool { self.inner.slide(Direction::Up) }
    pub fn slide_down(&mut self) -> bool { self.inner.slide(Direction::Down) }
}

use std::cell::RefCell;
use std::rc::Rc;
thread_local! {
    static GLOBAL_GAME: RefCell<Option<Rc<RefCell<WasmGameService>>>> = RefCell::new(None);
    static GLOBAL_CANVAS_ID: RefCell<Option<String>> = RefCell::new(None);
    static GLOBAL_ANIM: RefCell<Option<AnimState>> = RefCell::new(None);
}

#[derive(Clone)]
struct AnimState {
    moves: Vec<crate::domain::board::MoveEvent>,
    start_ms: f64,
    duration_ms: f64,
    direction: Direction,
}

#[wasm_bindgen]
pub fn start(canvas_id: String) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;

    let game_rc = Rc::new(RefCell::new(WasmGameService::new(4)));
    GLOBAL_GAME.with(|g| g.replace(Some(Rc::clone(&game_rc))));
    GLOBAL_CANVAS_ID.with(|c| c.replace(Some(canvas_id.clone())));

    {
        let renderer = crate::infra::render2d::Canvas2DRenderer::new(&window, &document, &canvas_id)
            .map_err(|e| JsValue::from_str(&e))?;
        renderer.draw(&game_rc.borrow().inner).map_err(|e| JsValue::from_str(&e))?;
    }

    // RAF loop
    {
        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let canvas_id_for_draw = canvas_id.clone();
        *g.borrow_mut() = Some(Closure::<dyn FnMut(f64)>::wrap(Box::new(move |now: f64| {
            let maybe_anim = GLOBAL_ANIM.with(|a| a.borrow().clone());
            if let Some(anim) = maybe_anim {
                let progress = ((now - anim.start_ms) / anim.duration_ms).clamp(0.0, 1.0);
                if let (Some(w), Some(d)) = (web_sys::window(), web_sys::window().and_then(|w| w.document())) {
                    GLOBAL_GAME.with(|g| {
                        if let Some(gref) = g.borrow().as_ref() {
                            if let Ok(renderer) = crate::infra::render2d::Canvas2DRenderer::new(&w, &d, &canvas_id_for_draw) {
                                let _ = renderer.draw_animated(&gref.borrow().inner, &anim.moves, progress);
                            }
                        }
                    });
                }
                if progress >= 1.0 {
                    // finalize: apply real slide and draw static board
                    GLOBAL_GAME.with(|g| {
                        if let Some(gref) = g.borrow().as_ref() {
                            let mut real = gref.borrow_mut();
                            let _ = real.inner.slide(anim.direction);
                            if let (Some(w), Some(d)) = (web_sys::window(), web_sys::window().and_then(|w| w.document())) {
                                if let Ok(renderer) = crate::infra::render2d::Canvas2DRenderer::new(&w, &d, &canvas_id_for_draw) {
                                    let _ = renderer.draw(&real.inner);
                                }
                            }
                        }
                    });
                    GLOBAL_ANIM.with(|a| a.replace(None));
                }
            }
            if let Some(w) = web_sys::window() {
                let _ = w.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
            }
        })));
        if let Some(w) = web_sys::window() {
            let _ = w.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
        }
    }

    {
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            // ignore input if animating
            let anim_busy = GLOBAL_ANIM.with(|a| a.borrow().is_some());
            if anim_busy { return; }

            let dir = match event.key().as_str() {
                "ArrowLeft" | "a" | "A" => Some(Direction::Left),
                "ArrowRight" | "d" | "D" => Some(Direction::Right),
                "ArrowUp" | "w" | "W" => Some(Direction::Up),
                "ArrowDown" | "s" | "S" => Some(Direction::Down),
                _ => None
            };
            if let Some(direction) = dir {
                // prepare animation events from current board WITHOUT applying state yet
                let (_did_move_unused, moves) = GLOBAL_GAME.with(|g| {
                    if let Some(gref) = g.borrow().as_ref() {
                        let gs_ref = gref.borrow();
                        let mut board_clone = gs_ref.inner.board().clone();
                        board_clone.slide_with_animations(direction)
                    } else { (false, vec![]) }
                });
                let effective_move = moves.iter().any(|e| e.merged_into_value.is_some() || e.from_row != e.to_row || e.from_col != e.to_col);
                if effective_move {
                    let start = web_sys::window().unwrap().performance().unwrap().now();
                    GLOBAL_ANIM.with(|a| a.replace(Some(AnimState { moves, start_ms: start, duration_ms: 140.0, direction })));
                }
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);

        document
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

