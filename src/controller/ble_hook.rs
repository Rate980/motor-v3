trait BleHook {
    fn on_connect(&mut self);
    fn on_disconnect(&mut self);
    fn on_stop(&mut self);
    fn on_start(&mut self);
}
