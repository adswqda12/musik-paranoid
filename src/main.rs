use std::io::{self,  Write, };
use crossterm::{
    execute, queue,
    style::{self}, cursor, terminal::{self, enable_raw_mode, disable_raw_mode},
    event::{read, KeyEvent, KeyCode}
};
fn clear(stdout : &mut io::Stdout) -> std::io::Result<()>
{
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?; 
    Ok(())
}
fn draw_init(stdout : &mut io::Stdout) -> io::Result<()>
{
    let _ = enable_raw_mode();
    let _ = clear(stdout);
    for x in 0..150 {
        queue!(stdout, cursor::MoveTo(x, 0), style::Print("-"))?;
    }
    queue!(stdout, cursor::Hide)?;
    let _ = stdout.flush();
    Ok(())
}
fn exit_clean(stdout : &mut io::Stdout) -> io::Result<()>
{
    queue!(stdout, cursor::Show)?;
    let _ = disable_raw_mode();
    let _ = stdout.flush();
    Ok(())
}
fn get_keystroke(stdout : &mut io::Stdout)
{
    let mut index = 0;
    loop {
        if let Ok(event) = read(){
            match event{
                crossterm::event::Event::Key(KeyEvent { code , modifiers, state, kind} )  => {
                    match code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if index < 1{
                                index += 1
                            }

                        }

                        KeyCode::Up | KeyCode::Char('k') => {
                            if index > 0 {
                                index -= 1
                            }
                        }
                        _=> break
                    }
                    let _ = draw_menu(stdout, index);
                }
                _=>{
                    eprintln!("other event");
                }
            }

        }

    }

}
//
fn draw_menu(stdout : &mut io::Stdout, index : u8) -> std::io::Result<()>
{
    let _ = clear(stdout);
    let _ = draw_init(stdout);
    let mut opt1 = "opt1";
    let mut opt2 = "opt2";
    let binding_opt1 = opt1.to_owned() + "<";
    let binding_opt2 = opt2.to_owned() + "<";
    match index{
        0 => opt1 = &binding_opt1,
        1 => opt2 = &binding_opt2,
        _=> _ = "<"
    }
    queue!(stdout, cursor::MoveTo(0,1), style::Print(opt1), cursor::MoveTo(0,2), style::Print(opt2))?;
    let _ = stdout.flush();
    Ok(())
}
fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let _ = draw_menu(&mut stdout,0);
    let _ = get_keystroke(&mut stdout);
    let _ = exit_clean(&mut stdout);
    Ok(())

}
