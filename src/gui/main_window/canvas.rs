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
        event: &iced::Event,       // <-- Referencja do Event, poprawnie
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> std::option::Option<iced::widget::Action<Message>> {
        // 1. Sprawdź, czy kursor znajduje się wewnątrz granic (in bounds) tego elementu
        if let Some(cursor_position) = cursor.position_in(bounds) {
            
            // 2. Dopasuj referencję do Event (używamy ref przed polami lub dereferencji)
            if let Event::Mouse(mouse::Event::WheelScrolled { delta }) = *event {
                
                // Wyciągamy wartość przesunięcia w pionie
                let delta_y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } => y,   // Tradycyjna mysz
                    mouse::ScrollDelta::Pixels { y, .. } => y,  // Gładzik / Trackpad
                };

                // 3. W nowym Iced do przechwycenia i wysłania wiadomości służy Action::publish
                // Automatycznie oznacza to wydarzenie jako Captured w drzewie widżetów.
                return Some(iced::widget::Action::publish(
                    Message::CanvasScrolled(
                        delta_y, 
                        cursor_position 
                    )
                ));
            }
        }

        // Jeśli kursor jest poza elementem lub zdarzenie to nie scroll, ignorujemy je
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

        let Some(selected_project) = &self.state.selected_project else {
            return vec![];
        };

        let Some(project) = self.state.open_projects_state.get(selected_project) else {
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
