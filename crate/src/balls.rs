use ball::Ball;
use render::*;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Balls {
    balls: Vec<Ball>,
    width: f64,
    heigth: f64,
}

#[wasm_bindgen]
impl Balls {
    #[wasm_bindgen(constructor, catch)]
    pub fn new() -> Balls {
        let balls = vec![];
        let width: f64 = 0.0;
        let heigth: f64 = 0.0;
        Balls {
            balls,
            width,
            heigth,
        }
    }
    #[wasm_bindgen(js_name=makeBalls)]
    pub fn make_balls(&mut self, max_balls: u32, stage_width: f64, stage_height: f64) {
        self.balls = (0..max_balls).map(|_| Ball::default()).collect();
        self.width = stage_width;
        self.heigth = stage_height;
    }

    #[wasm_bindgen(js_name=setRandomPositionAndSpeedInBounds)]
    pub fn set_random_position_and_speed_in_bounds(&mut self) {
        let width = self.width;
        let heigth = self.heigth;
        self.balls
            .iter_mut()
            .for_each(|ref mut ball| ball.set_random_position_and_speed_in_bounds(width, heigth))
    }
    fn step(&mut self) {
        self.balls.iter_mut().for_each(|ball| {
            ball.step();
        })
    }

    fn manage_stage_border_collision(&mut self, stage_width: f64, stage_height: f64) {
        self.balls.iter_mut().for_each(|ball| {
            ball.manage_stage_border_collision(stage_width, stage_height);
        })
    }

    #[wasm_bindgen(js_name=updateBalls)]
    pub fn update_balls(&mut self) {
        
        let width = self.width;
        let heigth = self.heigth;
        // move balls
        self.step();
        // check balls vs border collision
        self.manage_stage_border_collision(width, heigth);
        // check ball vs ball collision
        
        for i in 0..self.balls.len() {
            let (ball1, ball2) = self.balls.split_at_mut(i + 1);
            for j in 0..ball2.len() {
                log("yeah".to_string());
                if ball1[i].check_ball_collision(&mut ball2[j]) == true {
                  log("yea".to_string());
                    ball1[i].resolve_ball_collision(&mut ball2[j]);
                }
                log("ye".to_string());
            }
        }
    }

    #[wasm_bindgen(js_name=drawWasmBallsToCtx)]
    pub fn draw__wasms_balls_to_ctx(
        self,
        ctx: &web_sys::CanvasRenderingContext2d,
        color: &JsValue,
    ) {
        self.balls
            .iter()
            .for_each(|ball| draw_wasm_ball_to_ctx(&ball, ctx, color));
    }
}
