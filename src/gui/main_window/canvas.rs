use iced::{
    Color, Element, Event, Length, Point, Rectangle, Renderer, Theme, mouse, 
    widget::canvas::{self, Canvas, Frame, Geometry, Path},
};

use crate::{Message, State};

pub struct VectorCanvas<'a> {
    state: &'a State,
}

impl<'a> VectorCanvas<'a> {
    pub fn new(state: &'a State) -> Self {
        Self { state }
    }
}

impl<'a> canvas::Program<Message> for VectorCanvas<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> std::option::Option<iced::widget::Action<Message>> {
        if let Some(cursor_position) = cursor.position_in(bounds) {
            if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = *event {
                let delta_y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,   // Mouse
                    mouse::ScrollDelta::Pixels { y, .. } => y,  // Trackpad
                };

                return Some(iced::widget::Action::publish(
                    Message::CanvasScrolled(
                        delta_y, 
                        cursor_position 
                    )
                ));
            }
        }
        None
    }

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let Some(selected_project) = self.state.open_project else {
            return vec![];
        };

        let Some(project) = self.state.open_projects.get(selected_project) else {
            return vec![];
        };


        let background = Path::rectangle(
            Point::ORIGIN,
            bounds.size(),
        );

        frame.fill(
            &background,
            Color::from_rgb(0.1, 0.1, 0.1),
        );

        frame.translate(project.offset);
        frame.scale(project.zoom);
        
        let rectangle = Path::rectangle(
            Point::new(100.0, 100.0),
            iced::Size::new(100.0, 100.0),
        );

        frame.fill(
            &rectangle,
            Color::from_rgb(0.2, 0.6, 1.0),
        );

        vec![frame.into_geometry()]
    }
}

pub fn draw(state: &State) -> Element<'_, Message> {
    Canvas::new(VectorCanvas::new(state))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub fn scrolled(state: &mut State, delta: f32, _point: Point) {
    if let Some(open_project) = state.open_project {
        if let Some(project) = state.open_projects.get_mut(open_project) {
            let factor = delta * state.settings.zoom_speed * project.zoom;
            if state.control_pressed {
                project.offset = project.offset + iced::Vector{ x: 0.0, y: -factor };
            } else if state.shift_pressed {
                project.offset = project.offset + iced::Vector{ x: factor, y: 0.0 };
            } else {
                let factor = if delta > 0.0 { 1.1 } else { 0.9 };

                project.zoom *= factor;
                project.zoom = project.zoom.clamp(0.05, 50.0);
            }
        }
    }
}
