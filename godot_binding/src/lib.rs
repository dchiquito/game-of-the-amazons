use amazons_core::{Coord, Dim, Move};
use godot::classes::{INode, Node, Os};
use godot::prelude::*;
use std::io::{BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct CliInterface {
    #[base]
    _base: Base<Node>,

    child_io: Option<(ChildStdin, ChildStdout)>,
}

#[godot_api]
impl INode for CliInterface {
    fn init(base: Base<Node>) -> Self {
        Self {
            _base: base,
            child_io: None,
        }
    }
}

#[godot_api]
impl CliInterface {
    #[func]
    fn start_black(&mut self) -> bool {
        let args: Vec<String> = Os::singleton()
            .get_cmdline_user_args()
            .to_vec()
            .iter()
            .map(|gs| gs.into())
            .collect();
        let black_args: Vec<&str> = args
            .iter()
            .skip_while(|s| s != &"--black")
            .skip(1)
            .take_while(|s| s != &"--white")
            .map(String::as_ref)
            .collect();
        if !black_args.is_empty() {
            let child = Command::new(black_args[0])
                .args(black_args.iter().skip(1))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start black player");
            self.child_io = Some((child.stdin.unwrap(), child.stdout.unwrap()));
            true
        } else {
            false
        }
    }

    #[func]
    fn start_white(&mut self) -> bool {
        let args: Vec<String> = Os::singleton()
            .get_cmdline_user_args()
            .to_vec()
            .iter()
            .map(|gs| gs.into())
            .collect();
        let white_args: Vec<&str> = args
            .iter()
            .skip_while(|s| s != &"--white")
            .skip(1)
            .take_while(|s| s != &"--black")
            .map(String::as_ref)
            .collect();
        if !white_args.is_empty() {
            let child = Command::new(white_args[0])
                .args(white_args.iter().skip(1))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start black player");
            self.child_io = Some((child.stdin.unwrap(), child.stdout.unwrap()));
            true
        } else {
            false
        }
    }

    #[func]
    fn is_enabled(&self) -> bool {
        self.child_io.is_some()
    }

    #[func]
    fn notify_of_move(&mut self, piece: Array<i64>, mov: Array<i64>, arrow: Array<i64>) {
        println!("called notify_of_move {piece} {mov} {arrow}");
        let piece = Coord(
            Dim::from(usize::try_from(piece.get(0).unwrap()).unwrap()),
            Dim::from(usize::try_from(piece.get(1).unwrap()).unwrap()),
        );
        let mov = Coord(
            Dim::from(usize::try_from(mov.get(0).unwrap()).unwrap()),
            Dim::from(usize::try_from(mov.get(1).unwrap()).unwrap()),
        );
        let arrow = Coord(
            Dim::from(usize::try_from(arrow.get(0).unwrap()).unwrap()),
            Dim::from(usize::try_from(arrow.get(1).unwrap()).unwrap()),
        );
        let move_string = format!("{}\n", Move::notation_for(&piece, &mov, &arrow));
        println!("notifying cli of {move_string}");
        if let Some((child_stdin, _)) = &mut self.child_io {
            child_stdin
                .write_all(move_string.as_bytes())
                .expect("IO error notifying CLI of move");
        }
    }

    #[func]
    fn get_move(&mut self) -> VariantArray {
        println!("Getting a move from the CLI");
        if let Some((_, child_stdout)) = &mut self.child_io {
            let mut buf = [0; 1024];
            if let Ok(size) = child_stdout.read(&mut buf) {
                let notation =
                    String::from_utf8(buf[0..size].to_vec()).expect("Failed utf8 string encoding");
                if let Some((piece, mov, arrow)) = Move::parse_notation(&notation) {
                    return varray![
                        array![usize::from(&piece.0) as i64, usize::from(&piece.1) as i64],
                        array![usize::from(&mov.0) as i64, usize::from(&mov.1) as i64],
                        array![usize::from(&arrow.0) as i64, usize::from(&arrow.1) as i64]
                    ];
                }
            }
        }
        panic!("Failed to get a move from the CLI")
    }
}
