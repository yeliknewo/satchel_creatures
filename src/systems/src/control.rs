use glutin::MouseButton;

use std::sync::mpsc::{TryRecvError, Sender, Receiver};

use specs::{self, RunArg};

//*************************************************************************************************

use math::{OrthographicHelper, Point2};

use comps::{Transform, Camera, Clickable};
use comps::moving::Dir;

use super::overworld_control;

use utils::{Delta, Coord, GfxCoord};

//*************************************************************************************************

pub type Channel = (
    Sender<SendEvent>,
    Receiver<RecvEvent>
);

#[derive(Debug)]
pub enum RecvEvent {
    Right(bool),
    Left(bool),
    Up(bool),
    Down(bool),
    Resize(u32, u32),
    MouseMoved(u32, u32),
    MouseInput(bool, MouseButton),
}

#[derive(Debug)]
pub enum SendEvent {
    Resize,
}

#[derive(Debug, Copy, Clone)]
enum Sign {
    Pos,
    Zero,
    Neg,
}

#[derive(Debug)]
pub struct System {
    channel: Channel,
    overworld_channel: overworld_control::channel::Control,
    move_h: Sign,
    move_v: Sign,
    move_speed_mult: Point2,
    resize: Vec<(u32, u32)>,
    mouse_location: Point2,
    mouse_button: Vec<(bool, MouseButton)>,
    screen_resolution: Point2,
    ortho_helper: OrthographicHelper,
}

impl System {
    pub fn new(
        channel: Channel,
        overworld_channel: overworld_control::channel::Control,
        move_speed_mult: Point2,
        mouse_location: Point2,
        screen_resolution: Point2,
        ortho_helper: OrthographicHelper,
    ) -> System {
        System {
            channel: channel,
            overworld_channel: overworld_channel,
            move_h: Sign::Zero,
            move_v: Sign::Zero,
            move_speed_mult: move_speed_mult,
            resize: vec!(),
            mouse_location: mouse_location,
            mouse_button: vec!(),
            screen_resolution: screen_resolution,
            ortho_helper: ortho_helper,
        }
    }

    fn check_input(&mut self) {
        loop {
            match self.channel.1.try_recv() {
                Ok(event) => match event {
                    RecvEvent::MouseMoved(x, y) => {
                        self.mouse_location = Point2::new(
                            x as Coord / self.screen_resolution.get_x(),
                            y as Coord / self.screen_resolution.get_y()
                        );
                    },
                    RecvEvent::MouseInput(pressed, mouse_button) => self.mouse_button.push((pressed, mouse_button)),
                    RecvEvent::Right(pressed) => {
                        if pressed {
                            self.move_h = Sign::Pos;
                        } else if let Sign::Pos = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    RecvEvent::Left(pressed) => {
                        if pressed {
                            self.move_h = Sign::Neg;
                        } else if let Sign::Neg = self.move_h {
                            self.move_h = Sign::Zero;
                        }
                    },
                    RecvEvent::Up(pressed) => {
                        if pressed {
                            self.move_v = Sign::Pos;
                        } else if let Sign::Pos = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    RecvEvent::Down(pressed) => {
                        if pressed {
                            self.move_v = Sign::Neg;
                        } else if let Sign::Neg = self.move_v {
                            self.move_v = Sign::Zero;
                        }
                    },
                    RecvEvent::Resize(width, height) => {
                        match self.channel.0.send(SendEvent::Resize) {
                            Ok(()) => (),
                            Err(err) => error!("resize channel 0 send error: {}", err),
                        };
                        self.resize.push((width, height));
                    },
                },
                Err(TryRecvError::Empty) => return,
                Err(err) => {
                    error!("check input channel try recv error: {}", err);
                    return;
                },
            }
        }
    }
}

impl specs::System<Delta> for System {
    fn run(&mut self, arg: RunArg, _: Delta) {
        use specs::Join;

        self.check_input();

        let (transforms, mut cameras, mut clickables) = arg.fetch(|w|
            (
                w.read::<Transform>(),
                w.write::<Camera>(),
                w.write::<Clickable>(),
            )
        );

        let mut camera_opt = None;

        for mut camera in (&mut cameras).iter() {
            if camera.is_main() {
                // match (self.move_h, self.move_v) {
                //     (Sign::Zero, Sign::Zero) => (),
                //     (h, v) => {
                //         // let move_h = match h {
                //         //     Sign::Pos => 1.0,
                //         //     Sign::Zero => 0.0,
                //         //     Sign::Neg => -1.0,
                //         // };
                //         // let move_v = match v {
                //         //     Sign::Pos => 1.0,
                //         //     Sign::Zero => 0.0,
                //         //     Sign::Neg => -1.0,
                //         // };
                //         // let offset = c.get_offset();
                //         // c.set_offset(Point2::new(
                //         //     move_h * delta_time * self.move_speed_mult.get_x() + offset.get_x(),
                //         //     move_v * delta_time * self.move_speed_mult.get_y() + offset.get_y()
                //         // ));
                //     },
                // }
                for &(width, height) in &self.resize {
                    self.ortho_helper.set_aspect_ratio(width as GfxCoord / height as GfxCoord);
                    camera.set_proj(&self.ortho_helper);
                    self.screen_resolution = Point2::new(width as Coord, height as Coord);
                }
                camera_opt = Some(camera);
                break;
            }
        }

        let camera = match camera_opt {
            Some(c) => c,
            None => panic!("run camera opt was none"),
        };

        match (self.move_h, self.move_v) {
            (Sign::Pos, Sign::Zero) => self.overworld_channel.0.send(overworld_control::RecvEvent::Move(Dir::Right)).unwrap(),
            (Sign::Neg, Sign::Zero) => self.overworld_channel.0.send(overworld_control::RecvEvent::Move(Dir::Left)).unwrap(),
            (Sign::Zero, Sign::Pos) => self.overworld_channel.0.send(overworld_control::RecvEvent::Move(Dir::Up)).unwrap(),
            (Sign::Zero, Sign::Neg) => self.overworld_channel.0.send(overworld_control::RecvEvent::Move(Dir::Down)).unwrap(),
            (Sign::Zero, Sign::Zero) |
            _ => self.overworld_channel.0.send(overworld_control::RecvEvent::Move(Dir::Stay)).unwrap(),
        }

        if let Some(input) = self.mouse_button.pop() {
            match input {
                (true, MouseButton::Left) => {
                    for (transform, mut clickable) in (&transforms, &mut clickables).iter() {
                        if clickable.get_hitbox().check_collide_point(camera.screen_to_world_point(self.mouse_location.clone()) + transform.get_gui_offset()) {
                            *clickable.get_mut_clicked() = true;
                        } else if clickable.get_clicked() {
                            *clickable.get_mut_clicked() = false;
                        }
                    }
                },
                _ => (),
            }
        }
    }
}
