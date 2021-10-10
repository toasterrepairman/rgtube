// #################################################
// ## rgtube by toaster repairguy#1101            ##
// ##                                             ## 
// ## software is provided as-is without warranty ##
// ## or liability                                ##
// #################################################

extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use ytd_rs::{YoutubeDL, ResultType, Arg};
use std::path::PathBuf;
use futures::prelude::*;
use tokio::*;
use std::{thread};
use glib::{clone, MainContext};
use dirs;

#[tokio::main]
async fn main() {
    build_ui();
}

fn build_ui() {
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

    input.connect_activate(clone!(@strong input => move |_| {
        input.set_sensitive(false);
        let link = &input.get_text().as_str().to_string();
        download_video(link.to_string());
        input.set_sensitive(true);

        println!("input activated!");
    }));


    button.connect_clicked(clone!(@strong button => move |_| {
        button.set_sensitive(false);
        input.activate();
        button.set_sensitive(true);

        println!("button pressed!");
    }));
 

    window.show_all();
    gtk::main();
}

fn download_video(link: String) -> String {
    thread::spawn(move || {
        let args = vec![Arg::new("--quiet"), Arg::new_with_arg("--output", "%(title).90s%(ext)s")];
        let path = PathBuf::from(&dirs::download_dir().unwrap());
        let ytd = YoutubeDL::new(&path, args, &link)
            .expect("you borked it dummy");
        // start download
        let download = ytd.download();
        // check what the result is and print out the path to the download or the error
        match download.result_type() {
            ResultType::SUCCESS => println!("Your download: {}", download.output_dir().to_string_lossy()),
            ResultType::IOERROR | ResultType::FAILURE =>
                    println!("Couldn't start download: {}", download.output()),
        };
    });
    return ("we did it :)").to_string();
}
