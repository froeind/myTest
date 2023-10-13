use native_windows_gui as nwg;

fn main() {
    // Initialisiere die GUI
    nwg::init().expect("Failed to initialize Native Windows GUI");

    // Erzeuge ein neues Fenster
    let window = nwg::Window::default();

    // Setze den Titel des Fensters
    window.set_text("Hallo, Welt!");

    // Wenn das Fenster geschlossen wird, beende die GUI
    window.on_close(|| {
        nwg::stop_thread_dispatch();
    });

    // Zeige das Fenster an
    window.show();

    // Starte die GUI-Schleife
    nwg::dispatch_thread_events();
}