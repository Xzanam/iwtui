use std:: {io};
use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;





#[derive(Debug, Default)]
struct App { 
    should_quit : bool
}
 
impl App {   
    fn  run(&mut self, terminal : &mut ratatui::DefaultTerminal)  ->  io::Result<()>{ 

        while !self.should_quit { 
            terminal.draw(|frame| self.render(frame))? ;
            
            if let  Some(key)  = event::read()?.as_key_press_event() { 

                match key.code { 
                    KeyCode::Char('q') =>  self.should_quit = true, 
                    _ =>  todo!()
                }
                
            }
        }
        Ok(())
    }
    
    fn render(&mut self, frame: &mut Frame)   { 

    }

    
    fn handle_input(&self, event : Event)   { 
        
        todo!();
    }

}

#[tokio::main]
async fn main() ->  color_eyre::Result<()>{
    color_eyre::install()?;

    let mut  app = App::default();
    
    ratatui::run(|terminal| app.run(terminal) );
    
    
    Ok(())
}
