
slint::slint! {
    export component MainWindow inherits Window {
        /*
            Main container ontent:
            - Size
            - Title
            - Favicon
        */
        height: 768px;
        width: 1024px;
        title: "Track Tree";
        icon: @image-url("images/icons/favicon-16x16.png");
        Text {
            text: "Hello, Slint!";
            color: #eee;
        }
    }
}


fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
