use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.NetworkManager.WiMax.Nsp")]
trait Nsp {
    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// NetworkType property
    #[dbus_proxy(property)]
    fn network_type(&self) -> zbus::Result<u32>;

    /// SignalQuality property
    #[dbus_proxy(property)]
    fn signal_quality(&self) -> zbus::Result<u32>;
}
