use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use glutin::Event;

use sys::{control, render, mapper, overworld_control};
use ::game;

#[derive(Debug)]
pub struct GameEventHub {
    pub control_channel: Option<control::Channel>,
    pub render_channel: Option<render::Channel>,
    pub game_channel: Option<game::Channel>,
    pub mapper_channel_mapper: Option<mapper::channel::Mapper>,
    pub mapper_channel_game: Option<mapper::channel::Game>,
    pub overworld_control_channel_overworld: Option<overworld_control::channel::Overworld>,
    pub overworld_control_channel_control: Option<overworld_control::channel::Control>,
}

impl GameEventHub {
    pub fn new(
        control_channel: control::Channel,
        render_channel: render::Channel,
        game_channel: game::Channel,
        mapper_channel_mapper: mapper::channel::Mapper,
        mapper_channel_game: mapper::channel::Game,
        overworld_control_overworld: overworld_control::channel::Overworld,
        overworld_control_control: overworld_control::channel::Control,
    ) -> GameEventHub {
        GameEventHub {
            control_channel: Some(control_channel),
            render_channel: Some(render_channel),
            game_channel: Some(game_channel),
            mapper_channel_mapper: Some(mapper_channel_mapper),
            mapper_channel_game: Some(mapper_channel_game),
            overworld_control_channel_overworld: Some(overworld_control_overworld),
            overworld_control_channel_control: Some(overworld_control_control),
        }
    }
}

#[derive(Debug)]
pub struct DevEventHub {
    send_to_control: Sender<control::RecvEvent>,
    recv_from_control: Receiver<control::SendEvent>,
    send_to_render: Sender<render::RecvEvent>,
    recv_from_render: Receiver<render::SendEvent>,
    send_to_game: Sender<game::RecvEvent>,
    recv_from_game: Receiver<game::SendEvent>,
}

impl DevEventHub{
    pub fn new() -> (DevEventHub, GameEventHub) {
        let (send_to_control, recv_to_control) = channel();
        let (send_from_control, recv_from_control) = channel();
        let (send_to_render, recv_to_render) = channel();
        let (send_from_render, recv_from_render) = channel();
        let (send_to_game, recv_to_game) = channel();
        let (send_from_game, recv_from_game) = channel();
        let (send_mapper_from_game, recv_mapper_from_game) = channel();
        let (send_mapper_to_game, recv_mapper_to_game) = channel();
        let (send_overworld_control_from_control, recv_control_from_overworld_control) = channel();
        let (send_overworld_control_to_control, recv_overworld_control_to_control) = channel();

        (
            DevEventHub::new_internal(
                send_to_control, recv_from_control,
                send_to_render, recv_from_render,
                send_to_game, recv_from_game,
            ),
            GameEventHub::new(
                (send_from_control, recv_to_control),
                (send_from_render, recv_to_render),
                (send_from_game, recv_to_game),
                (send_mapper_to_game, recv_mapper_from_game),
                (send_mapper_from_game, recv_mapper_to_game),
                (send_overworld_control_to_control, recv_control_from_overworld_control),
                (send_overworld_control_from_control, recv_overworld_control_to_control)
            )
        )
    }

    fn new_internal(
        send_to_control: Sender<control::RecvEvent>,
        recv_from_control: Receiver<control::SendEvent>,
        send_to_render: Sender<render::RecvEvent>,
        recv_from_render: Receiver<render::SendEvent>,
        send_to_game: Sender<game::RecvEvent>,
        recv_from_game: Receiver<game::SendEvent>,
    ) -> DevEventHub {
        DevEventHub {
            send_to_control: send_to_control,
            recv_from_control: recv_from_control,
            send_to_render: send_to_render,
            recv_from_render: recv_from_render,
            send_to_game: send_to_game,
            recv_from_game: recv_from_game,

        }
    }

    pub fn send_to_control(&mut self, event: control::RecvEvent) {
        match self.send_to_control.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to control error: {}", err),
        }
    }

    pub fn recv_from_control(&mut self) -> control::SendEvent {
        match self.recv_from_control.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from control error: {}", err),
        }
    }

    pub fn try_recv_from_control(&mut self) -> Option<control::SendEvent> {
        match self.recv_from_control.try_recv() {
            Ok(event) => Some(event),
            Err(err) => match err {
                TryRecvError::Empty => None,
                TryRecvError::Disconnected => panic!("try recv from control was disconnected"),
            }
        }
    }

    pub fn send_to_render(&mut self, event: render::RecvEvent) {
        match self.send_to_render.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to render error: {}", err),
        }
    }

    pub fn recv_from_render(&mut self) -> render::SendEvent {
        match self.recv_from_render.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from render err: {}", err),
        }
    }

    pub fn send_to_game(&mut self, event: game::RecvEvent) {
        match self.send_to_game.send(event) {
            Ok(()) => (),
            Err(err) => error!("send to game error: {}", err),
        }
    }

    pub fn recv_from_game(&mut self) -> game::SendEvent {
        match self.recv_from_game.recv() {
            Ok(event) => event,
            Err(err) => panic!("recv from game err: {}", err),
        }
    }

    pub fn try_recv_from_game(&mut self) -> Option<game::SendEvent> {
        match self.recv_from_game.try_recv() {
            Ok(event) => Some(event),
            Err(err) => match err {
                TryRecvError::Empty => None,
                TryRecvError::Disconnected => panic!("try recv from game was disconnected"),
            },
        }
    }

    pub fn process_glutin(&mut self, event: Event) {
        use glutin::Event::{MouseMoved, MouseInput, KeyboardInput, Resized};
        use glutin::VirtualKeyCode;
        use glutin::ElementState::{Pressed, Released};
        match event {
            MouseMoved(x, y) => self.send_to_control(control::RecvEvent::MouseMoved(x as u32, y as u32)),
            MouseInput(state, button) => self.send_to_control(control::RecvEvent::MouseInput(match state {
                Pressed => true,
                Released => false,
            },
            button)),
            KeyboardInput(state, _, Some(VirtualKeyCode::D)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Right)) => match state {
                Pressed => self.send_to_control(control::RecvEvent::Right(true)),
                Released => self.send_to_control(control::RecvEvent::Right(false)),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::A)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Left)) => match state {
                Pressed => self.send_to_control(control::RecvEvent::Left(true)),
                Released => self.send_to_control(control::RecvEvent::Left(false)),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::W)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Up)) => match state {
                Pressed => self.send_to_control(control::RecvEvent::Up(true)),
                Released => self.send_to_control(control::RecvEvent::Up(false)),
            },
            KeyboardInput(state, _, Some(VirtualKeyCode::S)) |
            KeyboardInput(state, _, Some(VirtualKeyCode::Down)) => match state {
                Pressed => self.send_to_control(control::RecvEvent::Down(true)),
                Released => self.send_to_control(control::RecvEvent::Down(false)),
            },
            Resized(width, height) => self.send_to_control(control::RecvEvent::Resize(width, height)),
            _ => (),
        }
    }
}
