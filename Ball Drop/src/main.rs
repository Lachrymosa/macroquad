use macroquad::prelude::*;
struct Ball {
    pos: Vec2,
    vel: f32,
    acc: f32,
    rad: f32,
}

fn bounce(mut vel: f32) -> f32 {
    vel = vel * -0.75;
    return vel;
}

#[macroquad::main("Ball Drop")]
async fn main() {
    let mut ball = Ball {
        pos: Vec2::new(screen_width() / 2.0 - 5.0,50.0),
        vel: 0.0, // change to cause a starting velocity, negative is up.
        acc: 0.8, // change to affect gravity
        rad: 50.0 // change to manipulate ball radius size.
    };


    show_mouse(true);
    loop {
        clear_background(RED);
        let mouse = mouse_position();
        let (_mouse_x, _mouse_y) = mouse;
        if is_mouse_button_down(MouseButton::Left) {
            ball.pos = Vec2::from(mouse);
            ball.vel = 0.0;
        }

        if ball.pos.y > screen_height() - ball.rad {
            ball.vel = bounce(ball.vel);
            ball.pos.y = screen_height() - ball.rad;
        }

        ball.vel += ball.acc;
        ball.pos.y += ball.vel;
        
        let vel_text = format!("Velocity: {}", ball.vel);
        
        draw_text(&*vel_text, 10.0, 10.0, 20.0, BLACK);
        draw_circle(ball.pos.x, ball.pos.y, ball.rad, GREEN);

        next_frame().await
    }
}

