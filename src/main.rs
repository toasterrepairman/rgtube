// #################################################
// ## rgtube by toaster repairguy#1101            ##
// ##                                             ## 
// ## software is provided as-is without warranty ##
// ## or liability                                ##
// #################################################

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::io::BufReader;

fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("rgtube.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let print_operation = gtk::PrintOperation::new();

    // initialize widgets
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let console: gtk::TextView = builder.get_object("console").unwrap();
    let button: gtk::Button = builder.get_object("downloadbutton").unwrap();
    let input: gtk::Entry = builder.get_object("link").unwrap();

/*  window mapping

    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
*/

    button.connect_clicked(move |_| {
        let link = input.get_text().to_string();
        download_video(&link);
    });


    window.show_all();

    gtk::main();

}

async fn download_video(link: &str) -> String {
    let download_path: String = rustube::download_best_quality(&link).await
        .unwrap()
        .as_path()
        .display()
        .to_string();
    return download_path;
}