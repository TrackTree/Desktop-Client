/*
    <3rd party assests>
    ICONS: https://free-icons.github.io/free-icons/
*/
slint::slint! {
    import { GridBox, HorizontalBox } from "std-widgets.slint";

    component Header inherits Rectangle {
        width: 1366px;
        height: 30px;
        padding: 1px;
        background: #323232;
        y: 10px;
    }

    component SideBar inherits Rectangle {
        width: 600px;
        height: 768px;
        background: #323232;
    }
    export component MainWindow inherits Window {
        width: 1366px;
        height: 768px;
        title: "Track Tree";
        icon: @image-url("images/icons/favicon-16x16.png");

       GridLayout {
        Header {colspan: 6;}
        SideBar {colspan: 2;}
       }
    }
}

fn main() {
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    app.run().unwrap();
    println!("Exiting application.");
}
