
use imgui::*;
use super::support as support;

macro_rules! ig_dynamic_str {
    ($x:expr) => {
        unsafe { ImStr::from_utf8_with_nul_unchecked(format!("{}\0", $x).as_bytes()) }
    }
}

macro_rules! ig_make_label {
    ( $x:expr, $y:expr ) => {
        ig_dynamic_str!(format!("{}::{}", $x, $y))
    }
}

pub struct State {
    sudoku: [[i32; 9]; 9],
    notify_text: &'static str,
}

pub fn create_window(){
    let mut state = State{
        sudoku: [[9; 9]; 9],
        notify_text: "",
    };
    
    let system = support::init(file!());
    system.main_loop(move |_run, ui| {
        sudoku_ui(ui, &mut state);
    });
}

pub fn sudoku_ui(ui: &Ui, state: &mut State) {
    let w = Window::new(im_str!("Sudoku Solver"))
        .size([600.0, 500.0], Condition::FirstUseEver)
        .position([20.0, 140.0], Condition::FirstUseEver);
    w.build(ui, || {

        ui.text(state.notify_text);

        for i in 0..9 {
            for j in 0..9{
                ui.input_int(ig_make_label!("Box", format!("{},{}", i, j)), &mut state.sudoku[i][j]);
                //ui.same_line_with_spacing();
            }
            ui.new_line()
            
        }
        
        if ui.button(im_str!("Solve!"), [90.0, 30.0]) {
        }
    });
}
