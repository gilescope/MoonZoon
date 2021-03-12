pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use blocks::blocks;

pub mod component;
mod dom;
mod console;
mod hook;
mod state;
mod runtime;
mod state_map;

pub use component::*;
pub use dom::{Node, window, document}; 
pub use state::{State, CloneState};
pub use console::log;
pub use hook::{el_var, do_once};
pub use topo;
pub use topo::nested as render;

const ELEMENT_ID: &str = "app";

fn runtime_run_once() {
    runtime::run_once(|| root());
}

#[macro_export]
macro_rules! start {
    () => {
        $crate::start();
    };
}

pub fn start() {
    log!("start");
    console_error_panic_hook::set_once();

    // for revision in 0..2 {
    //     log!("revision: {}", revision);
    //     root();
    // }

    log!("-------- revision: 0 --------");
    runtime_run_once();

    log!("-------- revision: 1 --------");
    runtime_run_once();

    log!("-------- revision: 3 --------");
    runtime_run_once();
}

#[topo::nested]
fn root() {
    log!("root");

    let state_node = el_var(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });

    let rcx = RenderContext { 
        index: 0,
        state_node 
    };


    row![
        col![
            row![
                el![
                    "A1",
                ],
                button![
                    "X",
                    button::on_press(|| log!("delete A1")),
                ],
            ],
            row![
                el![
                    "A2",
                ],
                button![
                    "X",
                    button::on_press(|| log!("delete A2")),
                ],
            ],
        ],


        do_once(|| {
            log!("FIRST RUN!");

            col![
                row![
                    el![
                        "B1",
                    ],
                    button![
                        "X",
                        button::on_press(|| log!("delete B1")),
                    ],
                ],
                row![
                    el![
                        "B2",
                    ],
                    button![
                        "X",
                        button::on_press(|| log!("delete B2")),
                    ],
                ],
            ]
        }),
    ].render(rcx);
}
