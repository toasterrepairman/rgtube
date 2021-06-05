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

fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("rgtube.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window1").unwrap();

/*  window mapping

    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
*/

/*  button mapping below
    
    button.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });
*/

    window.show_all();

    gtk::main();

}
