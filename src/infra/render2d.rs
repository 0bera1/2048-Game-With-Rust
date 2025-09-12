use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, Document};
use std::collections::HashSet;

use crate::application::game_service::GameService;

pub struct Canvas2DRenderer {
    ctx: CanvasRenderingContext2d,
}

impl Canvas2DRenderer {
    pub fn new(window: &Window, document: &Document, canvas_id: &str) -> Result<Self, String> {
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| "Canvas bulunamadi".to_string())
            .and_then(|e| e.dyn_into::<HtmlCanvasElement>().map_err(|_| "Canvas cast hatasi".to_string()))?;

        let ctx = canvas
            .get_context("2d").map_err(|_| "2D context alinamadi".to_string())?
            .ok_or_else(|| "2D context yok".to_string())?
            .dyn_into::<CanvasRenderingContext2d>().map_err(|_| "2D context cast".to_string())?;

        Ok(Self { ctx })
    }

    pub fn draw(&self, game: &GameService) -> Result<(), String> {
        let board = game.board();
        let w = self.ctx.canvas().ok_or("Canvas yok")?.width() as f64;
        let h = self.ctx.canvas().ok_or("Canvas yok")?.height() as f64;
        self.ctx.set_fill_style(&"#faf8ef".into());
        self.ctx.fill_rect(0.0, 0.0, w, h);

        let grid = board.size as f64;
        let pad = 12.0;
        let tile_size = ((w.min(h)) - pad * (grid + 1.0)) / grid;

        for r in 0..board.size {
            for c in 0..board.size {
                let x = pad + c as f64 * (tile_size + pad);
                let y = pad + r as f64 * (tile_size + pad);

                self.ctx.set_fill_style(&"#bbada0".into());
                self.ctx.fill_rect(x, y, tile_size, tile_size);

                if let Some(tile) = board.get(r, c) {
                    let color = match tile.value {
                        2 => "#eee4da",
                        4 => "#ede0c8",
                        8 => "#f2b179",
                        16 => "#f59563",
                        32 => "#f67c5f",
                        64 => "#f65e3b",
                        128 => "#edcf72",
                        256 => "#edcc61",
                        512 => "#edc850",
                        1024 => "#edc53f",
                        2048 => "#edc22e",
                        _ => "#3c3a32",
                    };
                    self.ctx.set_fill_style(&color.into());
                    self.ctx.fill_rect(x, y, tile_size, tile_size);

                    self.ctx.set_fill_style(&if tile.value <= 4 { "#776e65" } else { "#f9f6f2" }.into());
                    self.ctx.set_font(&format!("{}px Clear Sans, Arial", (tile_size * 0.5) as i32));
                    self.ctx.set_text_align("center");
                    self.ctx.set_text_baseline("middle");
                    let _ = self.ctx.fill_text(&tile.value.to_string(), x + tile_size / 2.0, y + tile_size / 2.0);
                }
            }
        }

        // Score
        self.ctx.set_fill_style(&"#776e65".into());
        self.ctx.set_font("16px Arial");
        let _ = self.ctx.fill_text(&format!("Skor: {}", game.score()), 10.0, h - 10.0);
        Ok(())
    }

    pub fn draw_animated(&self, game: &GameService, moves: &[crate::domain::board::MoveEvent], progress: f64) -> Result<(), String> {
        let board = game.board();
        let w = self.ctx.canvas().ok_or("Canvas yok")?.width() as f64;
        let h = self.ctx.canvas().ok_or("Canvas yok")?.height() as f64;

        // background
        self.ctx.set_fill_style(&"#faf8ef".into());
        self.ctx.fill_rect(0.0, 0.0, w, h);

        let grid = board.size as f64;
        let pad = 12.0;
        let tile_size = ((w.min(h)) - pad * (grid + 1.0)) / grid;

        // draw grid cells background
        for r in 0..board.size {
            for c in 0..board.size {
                let x = pad + c as f64 * (tile_size + pad);
                let y = pad + r as f64 * (tile_size + pad);
                self.ctx.set_fill_style(&"#bbada0".into());
                self.ctx.fill_rect(x, y, tile_size, tile_size);
            }
        }

        // hide destination cells which are moving to avoid double-draw
        let mut hidden: HashSet<(usize, usize)> = HashSet::new();
        for m in moves { hidden.insert((m.to_row, m.to_col)); }

        // draw board tiles except hidden
        for r in 0..board.size {
            for c in 0..board.size {
                if hidden.contains(&(r, c)) { continue; }
                if let Some(tile) = board.get(r, c) {
                    self.draw_tile(tile.value, pad, tile_size, c as f64, r as f64, 1.0)?;
                }
            }
        }

        // moving overlays
        let p = progress.clamp(0.0, 1.0);
        let ease = |t: f64| 1.0 - (1.0 - t) * (1.0 - t) * (1.0 - t); // easeOutCubic
        let ep = ease(p);
        for m in moves {
            let fx = m.from_col as f64;
            let fy = m.from_row as f64;
            let tx = m.to_col as f64;
            let ty = m.to_row as f64;
            let ix = fx + (tx - fx) * ep;
            let iy = fy + (ty - fy) * ep;
            // merge pop near end
            let mut scale = 1.0;
            if m.merged_into_value.is_some() {
                if p > 0.8 { scale = 1.0 + 0.15 * (1.0 - (1.0 - (p - 0.8) / 0.2)); }
            }
            self.draw_tile(m.value, pad, tile_size, ix, iy, scale)?;
        }

        // draw merged targets pop at end (after movers) to show bigger new value
        if p >= 0.8 {
            for m in moves {
                if let Some(new_val) = m.merged_into_value {
                    self.draw_tile(new_val, pad, tile_size, m.to_col as f64, m.to_row as f64, 1.0 + 0.15 * ((p - 0.8) / 0.2))?;
                }
            }
        }

        // score
        self.ctx.set_fill_style(&"#776e65".into());
        self.ctx.set_font("16px Arial");
        let _ = self.ctx.fill_text(&format!("Skor: {}", game.score()), 10.0, h - 10.0);
        Ok(())
    }

    fn draw_tile(&self, value: u32, pad: f64, tile_size: f64, grid_x: f64, grid_y: f64, scale: f64) -> Result<(), String> {
        let x = pad + grid_x * (tile_size + pad);
        let y = pad + grid_y * (tile_size + pad);
        let cx = x + tile_size / 2.0;
        let cy = y + tile_size / 2.0;
        let s = tile_size * scale;
        let sx = cx - s / 2.0;
        let sy = cy - s / 2.0;
        let color = match value {
            2 => "#eee4da",
            4 => "#ede0c8",
            8 => "#f2b179",
            16 => "#f59563",
            32 => "#f67c5f",
            64 => "#f65e3b",
            128 => "#edcf72",
            256 => "#edcc61",
            512 => "#edc850",
            1024 => "#edc53f",
            2048 => "#edc22e",
            _ => "#3c3a32",
        };
        self.ctx.set_fill_style(&color.into());
        self.ctx.fill_rect(sx, sy, s, s);
        self.ctx.set_fill_style(&if value <= 4 { "#776e65" } else { "#f9f6f2" }.into());
        self.ctx.set_font(&format!("{}px Clear Sans, Arial", (tile_size * 0.5 * scale) as i32));
        self.ctx.set_text_align("center");
        self.ctx.set_text_baseline("middle");
        let _ = self.ctx.fill_text(&value.to_string(), cx, cy);
        Ok(())
    }
}

